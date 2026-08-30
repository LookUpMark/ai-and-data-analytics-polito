# **LAB 5: WAITPID**

## Obiettivi Generali del Laboratorio

1.  **Implementare `waitpid`:** Realizzare il supporto kernel per la system call `waitpid`, permettendo a un processo di attendere la terminazione di un altro processo specifico (identificato dal suo PID) e di recuperarne lo stato di uscita.
2.  **Gestire la Terminazione dei Processi:** Modificare il flusso di terminazione (`sys__exit`) per interagire correttamente con `waitpid`, introducendo lo stato "zombie" e gestendo la distruzione della struttura `proc` al momento giusto.
3.  **Assegnare e Gestire i PID:** Implementare un meccanismo per assegnare identificatori di processo (PID) univoci ai nuovi processi e una tabella per mappare i PID alle strutture `proc` corrispondenti.
4.  **(Opzionale) Implementare `getpid` e `fork`:** Realizzare le system call `getpid` (per ottenere il PID del processo corrente) e `fork` (per creare un nuovo processo duplicando quello esistente), necessarie per testare `waitpid` in scenari user-level più realistici.

---

## Parte 1: Attesa della Terminazione del Processo (`proc_wait`)

*(Basato sulle slide 1, 2, 4 e codice in `proc.c`, `proc.h`)*

**Obiettivo:** Creare una funzione kernel interna, `proc_wait`, che permetta al thread corrente di attendere la terminazione di un *altro* processo specifico, identificato da un puntatore diretto alla sua struttura `proc`. Questa funzione è un blocco fondamentale su cui costruire `sys_waitpid`.

**Concetto Chiave: Sincronizzazione tra `_exit` e `wait`**

*   Quando un processo `P_child` termina, chiama `sys__exit`.
*   Quando un processo `P_parent` chiama `waitpid` (o, internamente, `proc_wait`) per attendere `P_child`, deve bloccarsi se `P_child` non ha ancora terminato.
*   `sys__exit` di `P_child` deve poter segnalare a `P_parent` (bloccato in `proc_wait`) che la terminazione è avvenuta.
*   Lo stato di uscita di `P_child` deve essere conservato finché `P_parent` non lo recupera con `proc_wait`.
*   La struttura `proc` di `P_child` non può essere distrutta subito in `sys__exit`, ma solo dopo che `P_parent` ha completato la `proc_wait`.

**Implementazione:**

1.  **Modifica `struct proc` (`proc.h`):** Aggiungere campi per la sincronizzazione e lo stato di uscita. La soluzione usa una macro `USE_SEMAPHORE_FOR_WAITPID` per scegliere tra semaforo o CV+lock. Useremo l'implementazione con **Semaforo** (impostazione predefinita `USE_SEMAPHORE_FOR_WAITPID 1`).
    ```c
    // In struct proc:
    #if OPT_WAITPID
            int p_status;                   // Status salvato da sys__exit
            pid_t p_pid;                    // PID del processo
    #if USE_SEMAPHORE_FOR_WAITPID
    	struct semaphore *p_sem;        // Semaforo per la sincronizzazione exit/wait
    #else
            // Alternativa con CV + Lock (non usata nella config di default)
            struct cv *p_cv;
            struct lock *p_lock;
    #endif
    #endif
    ```
2.  **Inizializzazione in `proc_create` (tramite `proc_init_waitpid` in `proc.c`):**
    *   Quando un processo viene creato, bisogna inizializzare il nuovo semaforo `p_sem`.
    *   **Importante:** Il semaforo deve essere creato con un contatore iniziale di **0**. Questo perché il processo "in attesa" (`P_parent`) chiamerà `P(p_sem)` e dovrà bloccarsi *finché* il processo "atteso" (`P_child`) non farà `V(p_sem)` durante la sua uscita.
    ```c
    // Dentro proc_init_waitpid (chiamata da proc_create):
    proc->p_status = 0; // Stato iniziale (o un valore speciale tipo "in esecuzione"?)
    #if USE_SEMAPHORE_FOR_WAITPID
      // Crea il semaforo con nome e contatore iniziale 0
      proc->p_sem = sem_create(name, 0);
      // ... gestione errore sem_create ...
    #else
      // ... creazione CV + Lock ...
    #endif
    ```
3.  **Implementazione `proc_wait(struct proc *proc)` (`proc.c`):** Questa è la funzione chiamata dal thread che *attende*.
    ```c
    int
    proc_wait(struct proc *proc)
    {
    #if OPT_WAITPID
            int return_status;
            /* Controlli: non si può attendere NULL o il kernel stesso */
    	KASSERT(proc != NULL);
    	KASSERT(proc != kproc);

            /* Attendi sul semaforo del processo 'proc' */
            /* Se il processo 'proc' non ha ancora chiamato V(proc->p_sem)
             * (cioè non ha ancora raggiunto la fine di sys__exit), questa
             * chiamata a P() bloccherà il thread corrente (il "genitore"). */
    #if USE_SEMAPHORE_FOR_WAITPID
            P(proc->p_sem); // Attende il segnale V() da sys__exit
    #else
            // Alternativa con CV
            lock_acquire(proc->p_lock);
            cv_wait(proc->p_cv); // Attende cv_signal() da sys__exit
            lock_release(proc->p_lock);
    #endif
            /* A questo punto, P() è ritornato (o cv_wait è stato svegliato).
             * Significa che il processo 'proc' ha terminato e ha segnalato.
             * Recuperiamo lo stato di uscita salvato. */
            return_status = proc->p_status;

            /* Ora che abbiamo ricevuto la segnalazione e lo stato,
             * possiamo distruggere la struttura proc del processo terminato. */
            proc_destroy(proc);

            /* Ritorna lo stato di uscita */
            return return_status;
    #else
            /* Implementazione base senza waitpid: non fa nulla */
            (void)proc;
            return 0;
    #endif
    }
    ```
4.  **Modifica `sys__exit` (in `proc_syscalls.c` o simile):** Questa è la funzione chiamata dal thread del processo che *termina*.
    ```c
    void
    sys__exit(int status)
    {
    #if OPT_WAITPID
      struct proc *p = curproc; // Processo corrente che sta terminando

      // 1. Salva lo stato di uscita nella struttura proc.
      //    La maschera & 0xff assicura che venga salvato solo il byte basso,
      //    come da convenzione POSIX per exit status.
      p->p_status = status & 0xff;

      // 2. Rimuovi il thread corrente dal conteggio dei thread del processo.
      //    NOTA: Questo è un punto potenzialmente critico per race condition
      //    se proc_destroy viene chiamato troppo presto da proc_wait.
      //    Vedi discussione sotto.
      proc_remthread(curthread);

      // 3. Segnala al processo in attesa (se ce n'è uno) che la terminazione
      //    è avvenuta, permettendogli di sbloccarsi da P(p->p_sem).
    #if USE_SEMAPHORE_FOR_WAITPID
      V(p->p_sem); // Incrementa il semaforo, sveglia un eventuale P()
    #else
      // Alternativa con CV
      lock_acquire(p->p_lock);
      cv_signal(p->p_cv); // Sveglia un eventuale cv_wait()
      lock_release(p->p_lock);
    #endif

      // 4. Termina l'esecuzione del thread corrente.
      //    IMPORTANTE: Non chiamare più as_destroy qui!
      //    La distruzione dell'address space e della struct proc
      //    avverrà ora DENTRO proc_wait, dopo la segnalazione.
      thread_exit(); // Non ritorna

    #else
      // Implementazione originale senza waitpid: distrugge l'address space qui
      struct addrspace *as = proc_getas();
      as_destroy(as);
      thread_exit();
    #endif

      panic("thread_exit returned (should not happen)\n");
      (void) status; // Per evitare warning 'unused parameter'
    }
    ```

**Spiegazione Flusso `proc_wait` / `sys__exit`:**

*   Un processo `P_parent` vuole attendere `P_child`. Chiama una funzione che internamente arriva a `proc_wait(P_child)`.
*   `proc_wait` chiama `P(P_child->p_sem)`. Poiché il semaforo è 0, `P_parent` si blocca.
*   `P_child` finisce il suo lavoro e chiama `exit(status)` (user space).
*   Questo causa una trap e viene eseguita `sys__exit(status)` nel kernel.
*   `sys__exit` salva `status` in `P_child->p_status`.
*   `sys__exit` chiama `proc_remthread(curthread)` (per aggiornare `p_numthreads`).
*   `sys__exit` chiama `V(P_child->p_sem)`. Il contatore del semaforo diventa 1.
*   La chiamata `V` risveglia `P_parent` che era bloccato in `P()`.
*   `P_parent` esce da `P()`, legge `P_child->p_status`.
*   `P_parent` chiama `proc_destroy(P_child)` per liberare la struttura `proc` di `P_child`.
*   `P_parent` ritorna lo stato letto.
*   Nel frattempo, `sys__exit` di `P_child` ha chiamato `thread_exit()`, e il thread di `P_child` è terminato.

---

## Parte 2: Distruzione della Struttura `proc` e Race Condition

*(Basato sulle slide 2, 5 e Q&A #1)*

**Obiettivo:** Gestire correttamente la deallocazione della `struct proc` ed evitare race condition.

**Problema (Slide 2, Q&A #1):**

*   La funzione `proc_destroy` (chiamata ora da `proc_wait`) contiene un `KASSERT(proc->p_numthreads == 0)`. Serve ad assicurarsi che non si distrugga una struttura `proc` mentre ha ancora thread attivi associati.
*   Il flusso descritto sopra ha una **potenziale race condition:**
    1.  `sys__exit` (in `P_child`) chiama `V(p_sem)`.
    2.  `proc_wait` (in `P_parent`) viene svegliato immediatamente da `P()`.
    3.  `proc_wait` procede e chiama `proc_destroy(P_child)`.
    4.  `proc_destroy` controlla `P_child->p_numthreads`.
    5.  **Contemporaneamente**, `sys__exit` (in `P_child`) chiama `thread_exit()`. All'interno di `thread_exit`, c'è una chiamata a `proc_remthread(curthread)` che decrementa `p_numthreads` e imposta `t_proc` a `NULL`.
*   **Se `proc_destroy` legge `p_numthreads` *prima* che `proc_remthread` (chiamato da `thread_exit`) lo abbia decrementato a 0, l'asserzione `KASSERT(p_numthreads == 0)` fallirà, causando un panic!**

**Soluzione Suggerita (Slide 3):**

*   Modificare `sys__exit` per chiamare `proc_remthread(curthread)` **prima** di chiamare `V(p_sem)`.
    ```c
    // In sys__exit:
    // ... salva status ...
    proc_remthread(curthread); // Decrementa p_numthreads ORA
    V(p_sem); // POI segnala
    thread_exit(); // Infine, esci dal thread (che ora non è più legato al proc)
    ```
*   **Modificare `thread_exit`:** Bisogna assicurarsi che `thread_exit` possa gestire correttamente il caso in cui viene chiamata da un thread il cui campo `t_proc` è *già* stato impostato a `NULL` da `proc_remthread` in `sys__exit`. Non deve tentare di chiamare `proc_remthread` una seconda volta. (Questo richiede una modifica interna a `thread_exit` o a `proc_remthread` per essere idempotente o per controllare `t_proc`).

**Modifica a `proc_destroy` (Slide 5):**

*   La logica di `proc_destroy` rimane simile, ma ora viene chiamata da `proc_wait` *dopo* che la terminazione è stata segnalata e lo stato è stato recuperato.
*   `proc_destroy` si occupa ancora di liberare l'address space (`as_destroy(as)`) e le altre risorse VFS (`VOP_DECREF(p_cwd)`), oltre a pulire lo spinlock e le strutture waitpid (`proc_end_waitpid`) e infine liberare la `struct proc` stessa (`kfree(proc)`).
*   **Importante (Slide 5):** `sys__exit` **NON** deve più chiamare `as_destroy`. Questo compito è ora delegato a `proc_destroy` chiamato da `proc_wait`.

**Codice `proc_destroy` (con commenti sull'ordine):**

```c
void
proc_destroy(struct proc *proc)
{
	KASSERT(proc != NULL);
	KASSERT(proc != kproc);

	// 1. Libera risorse VFS
	if (proc->p_cwd) {
		VOP_DECREF(proc->p_cwd);
		proc->p_cwd = NULL;
	}

	// 2. Libera risorse VM (Address Space)
	if (proc->p_addrspace) {
		struct addrspace *as;
		// Gestione speciale se si distrugge il processo corrente
                // (non dovrebbe accadere se chiamato da proc_wait)
		if (proc == curproc) {
			as = proc_setas(NULL);
			as_deactivate();
		}
		else {
			as = proc->p_addrspace;
			proc->p_addrspace = NULL;
		}
		as_destroy(as); // Dealloca le pagine fisiche!
	}

        // 3. Verifica numero thread (grazie alla modifica in sys__exit,
        //    questo assert dovrebbe ora passare quando chiamato da proc_wait)
	KASSERT(proc->p_numthreads == 0);

        // 4. Pulisci spinlock del processo
	spinlock_cleanup(&proc->p_lock);

        // 5. Rimuovi da tabella PID e distruggi semaforo/CV di waitpid
	proc_end_waitpid(proc); // Funzione helper aggiunta

        // 6. Libera memoria della struttura
	kfree(proc->p_name);
	kfree(proc);
}

// Funzione helper per pulire le strutture waitpid
static void
proc_end_waitpid(struct proc *proc) {
#if OPT_WAITPID
  // Rimuovi dalla tabella dei processi (vedi Parte 3)
  int i;
  spinlock_acquire(&processTable.lk);
  i = proc->p_pid;
  if (i > 0 && i <= MAX_PROC && processTable.proc[i] == proc) {
      processTable.proc[i] = NULL;
  }
  spinlock_release(&processTable.lk);

  // Distruggi il semaforo o CV/Lock
#if USE_SEMAPHORE_FOR_WAITPID
  if (proc->p_sem != NULL) { // Aggiunto controllo NULL per sicurezza
      sem_destroy(proc->p_sem);
      proc->p_sem = NULL; // Buona pratica
  }
#else
  // ... destroy CV and Lock ...
#endif
#else
  (void)proc;
#endif
}
```

---

## Parte 3: Assegnazione PID e Tabella dei Processi

*(Basato sulle slide 3, 6 e codice `proc.c`)*

**Obiettivo:** Implementare un sistema per assegnare PID univoci ai processi e una tabella per trovare un `struct proc *` dato un PID, necessario per `sys_waitpid`.

**Implementazione:**

1.  **Definire Limiti PID:** I valori `PID_MIN` e `PID_MAX` sono definiti in `kern/include/limits.h`. Il tipo `pid_t` è usato per i PID.
2.  **Tabella dei Processi (`processTable` in `proc.c`):** Viene definita una struttura globale statica `_processTable` (poi referenziata come `processTable`).
    ```c
    #define MAX_PROC 100 // Limite massimo di processi (arbitrario)
    static struct _processTable {
      int active;                      // Flag per indicare se la tabella è attiva
      struct proc *proc[MAX_PROC+1]; // Array di puntatori a struct proc. [0] non usato.
      int last_i;                      // Ultimo indice usato per l'assegnazione (strategia circolare)
      struct spinlock lk;              // Spinlock per proteggere la tabella
    } processTable;
    ```
3.  **Inizializzazione Tabella (`proc_bootstrap` in `proc.c`):**
    *   Oltre a creare `kproc`, inizializza lo spinlock della tabella e imposta `active = 1`.
    *   Non registra `kproc` nella tabella.
    ```c
    void
    proc_bootstrap(void)
    {
    	kproc = proc_create("[kernel]"); // kproc NON avrà PID o semaforo waitpid
    	if (kproc == NULL) { panic(...); }
    #if OPT_WAITPID
    	spinlock_init(&processTable.lk);
    	// Inizializza l'array a NULL (o fatto da .bss?)
        for(int i=0; i<=MAX_PROC; i++) processTable.proc[i] = NULL;
        processTable.last_i = 0; // Inizia assegnazione da 1
    	processTable.active = 1;
    #endif
    }
    ```
4.  **Assegnazione PID (`proc_init_waitpid` in `proc.c`):** Questa funzione (chiamata da `proc_create`) ora cerca anche uno slot libero nella `processTable`.
    *   Usa una strategia circolare semplice: parte da `last_i + 1`, cerca uno slot `NULL`.
    *   Se trova uno slot `i`, assegna `processTable.proc[i] = proc`, aggiorna `processTable.last_i = i`, e imposta `proc->p_pid = i`.
    *   Se non trova slot liberi, va in panic (limite `MAX_PROC` raggiunto).
    *   L'intera ricerca e assegnazione è protetta dallo spinlock `processTable.lk`.
    ```c
    // Dentro proc_init_waitpid (prima parte):
    #if OPT_WAITPID
      int i;
      spinlock_acquire(&processTable.lk);
      i = processTable.last_i + 1;
      proc->p_pid = 0; // PID iniziale non valido
      if (i > MAX_PROC) i = 1; // Gestione wrap-around
      // Cerca slot libero circolarmente
      while (i != processTable.last_i) {
        if (processTable.proc[i] == NULL) { // Trovato slot libero
          processTable.proc[i] = proc;      // Assegna puntatore
          processTable.last_i = i;          // Aggiorna ultimo indice
          proc->p_pid = i;                  // Imposta PID nella struct proc
          break;                            // Esci dalla ricerca
        }
        i++;
        if (i > MAX_PROC) i = 1; // Gestione wrap-around
      }
      spinlock_release(&processTable.lk);
      // Se non abbiamo trovato un PID...
      if (proc->p_pid == 0) {
        panic("too many processes. proc table is full\n");
      }
      // ... (resto di proc_init_waitpid: crea semaforo/cv) ...
    #endif
    ```
5.  **Ricerca PID (`proc_search_pid` in `proc.c`):** Funzione per ottenere `struct proc *` da un `pid_t`.
    *   Verifica che il `pid` sia nei limiti validi.
    *   Accede direttamente a `processTable.proc[pid]`. (Non serve lock per sola lettura se si assume che il puntatore sia valido o NULL). La soluzione fornita non usa lock qui.
    *   Fa un `KASSERT(p->p_pid == pid)` per verifica.
    ```c
    struct proc *
    proc_search_pid(pid_t pid) {
    #if OPT_WAITPID
      struct proc *p;
      // Verifica range PID (PID 0 non usato, > MAX_PROC non valido)
      KASSERT(pid > 0 && pid <= MAX_PROC);
      // Accesso diretto (assumendo tabella inizializzata)
      // NB: Nessun lock qui nella soluzione, potrebbe essere problematico se un
      // processo viene distrutto concorrentemente? Rischio basso se il PID
      // proviene da una fonte affidabile (es. figlio appena creato).
      p = processTable.proc[pid];

      // Se p non è NULL, verifica che il PID nella struct corrisponda
      // (Protezione contro slot riutilizzati o errori?)
      if (p != NULL) {
           KASSERT(p->p_pid == pid);
      } // Se p è NULL, il processo non esiste (o è già stato distrutto)
      return p;
    #else
      (void)pid;
      return NULL;
    #endif
    }
    ```
6.  **Rimozione dalla Tabella (`proc_end_waitpid` in `proc.c`):** Chiamata da `proc_destroy`.
    *   Acquisisce `processTable.lk`.
    *   Imposta `processTable.proc[proc->p_pid] = NULL`.
    *   Rilascia `processTable.lk`.
    *   Distrugge semaforo/CV. (Vedi codice in Task 3.2)

---

## Parte 4: Implementazione di `sys_waitpid` e `sys_getpid`

*(Basato sulle slide 3, 6 e codice `proc_syscalls.c`, `syscall.c`, `syscall.h`)*

**Obiettivo:** Implementare le system call `sys_waitpid` e `sys_getpid` che verranno chiamate dallo user space.

**`sys_getpid(void)` (`proc_syscalls.c`):**

```c
pid_t
sys_getpid(void)
{
#if OPT_WAITPID
  // Assumiamo che curproc sia sempre valido se siamo in una syscall
  KASSERT(curproc != NULL);
  // Ritorna il PID memorizzato nella struct proc del processo corrente
  return curproc->p_pid;
#else
  return -1; // Non implementato
#endif
}
```

**Spiegazione:** Molto semplice: ritorna il campo `p_pid` della struttura `proc` del processo corrente (`curproc`).

**`sys_waitpid(pid_t pid, userptr_t statusp, int options)` (`proc_syscalls.c`):**

```c
int
sys_waitpid(pid_t pid, userptr_t statusp, int options)
{
#if OPT_WAITPID
  struct proc *p;
  int s; // Stato di uscita

  // 1. Cerca il processo figlio usando il PID fornito
  p = proc_search_pid(pid);

  // 2. Gestione Opzioni (Ignorata qui)
  (void)options; /* Options (like WNOHANG) non sono gestite */

  // 3. Controlla se il processo esiste
  if (p == NULL) {
      // Processo non trovato (PID non valido o processo già terminato e raccolto?)
      // In Unix, ritornerebbe errore (es. ESRCH)
      return -1; // Errore generico
  }

  // TODO: Controllo Parentela
  // In un sistema Unix completo, dovremmo verificare che 'pid' sia un figlio
  // del processo corrente (curproc). Questa implementazione permette a
  // qualsiasi processo di attendere qualsiasi altro, il che è una falla
  // di sicurezza/design.

  // 4. Chiama proc_wait per attendere la terminazione e ottenere lo stato
  //    proc_wait bloccherà finché il processo 'p' non chiama sys__exit e V(p->p_sem)
  s = proc_wait(p); // 'p' (la struct proc) verrà distrutto dentro proc_wait

  // 5. Copia (opzionale) lo stato di uscita allo user space
  if (statusp != NULL) {
      // !!! MANCA copyout !!! Questo è un accesso diretto a user space!
      // Dovrebbe essere: copyout(&s, statusp, sizeof(int));
      *(int*)statusp = s; // Accesso diretto e INSICURO
  }

  // 6. Ritorna il PID del processo terminato
  return pid;
#else
  // Funzionalità non compilata
  (void)options;
  (void)pid;
  (void)statusp;
  return -1; // Non implementato
#endif
}
```

**Spiegazione:**

1.  Cerca il `struct proc *` corrispondente al `pid` usando `proc_search_pid`.
2.  Ignora `options`.
3.  Se `p` è `NULL`, il processo non esiste, ritorna errore.
4.  **Manca Controllo Parentela:** Non verifica se `curproc` è il genitore di `p`.
5.  Chiama `proc_wait(p)`. Questa chiamata blocca il processo corrente finché `p` non termina, recupera lo stato di uscita (`s`), e distrugge `p`.
6.  Se `statusp` (puntatore user space) non è `NULL`, tenta di scrivere lo stato `s` a quell'indirizzo. **ATTENZIONE: Usa scrittura diretta `*(int*)statusp = s`, che è INSICURA. Dovrebbe usare `copyout`.**
7.  Ritorna il `pid` del processo atteso.

**Integrazione nel Dispatcher `syscall()` (`syscall.c`):**

Aggiungere i `case` per `SYS_getpid` e `SYS_waitpid`:

```c
 // Dentro lo switch(callno) in syscall():
     case SYS_waitpid:
         // Estrai args: pid=a0, statusp=a1, options=a2
         retval = sys_waitpid((pid_t)tf->tf_a0,
                        (userptr_t)tf->tf_a1,
                        (int)tf->tf_a2);
         // sys_waitpid ritorna -1 su errore (es. PID non trovato)
         if (retval < 0) {
              // TODO: Mappare l'errore specifico (ESRCH?) invece di ENOSYS
              err = ENOSYS;
         } else {
              err = 0; // retval contiene il PID
         }
         break;

     case SYS_getpid:
         // Nessun argomento
         retval = sys_getpid();
         if (retval < 0) { // Errore se non implementato
             err = ENOSYS;
         } else {
             err = 0; // retval contiene il PID
         }
         break;
```

**Aggiungere Prototipi (`syscall.h`):**

```c
 // Includere vicino agli altri prototipi sys_
 int sys_waitpid(pid_t pid, userptr_t statusp, int options);
 pid_t sys_getpid(void);
```

---

## Parte 5: Implementazione Opzionale di `fork`

*(Basato sulla slide 3 e codice `proc_syscalls.c`, `syscall.c`, `syscall.h`)*

**Obiettivo:** Creare un nuovo processo duplicando il processo corrente.

**Concetti Chiave:**

*   **Duplicazione:** `fork` crea un processo figlio che è (quasi) una copia esatta del genitore al momento della chiamata.
*   **Address Space:** Il figlio ottiene una copia *separata* dell'address space del genitore (`as_copy`).
*   **Trapframe:** Bisogna duplicare il trapframe del genitore per il figlio, ma modificare i valori di ritorno: nel figlio `fork` ritorna 0, nel genitore ritorna il PID del figlio.
*   **Thread:** Viene creato un nuovo thread kernel per eseguire il processo figlio. Questo thread inizierà l'esecuzione in una funzione speciale (`enter_forked_process`) che userà il trapframe duplicato per entrare in user mode.

**Implementazione:**

**`sys_fork(struct trapframe *ctf, pid_t *retval)` (`proc_syscalls.c`):**

*   `ctf`: Trapframe del processo corrente (genitore) passato da `syscall()`.
*   `retval`: Puntatore dove scrivere il valore di ritorno (PID del figlio per il genitore).

```c
#if OPT_FORK
// Funzione helper per avviare il thread figlio
static void
call_enter_forked_process(void *tfv, unsigned long dummy) {
  struct trapframe *tf = (struct trapframe *)tfv; // Trapframe duplicato del figlio
  (void)dummy; // Argomento non usato
  // Chiama la funzione che userà il trapframe per passare a user mode
  enter_forked_process(tf);

  // Non dovrebbe mai arrivare qui
  panic("enter_forked_process returned (should not happen)\n");
}

// Implementazione della system call fork
int sys_fork(struct trapframe *ctf, pid_t *retval) {
  struct trapframe *tf_child; // Puntatore al trapframe duplicato per il figlio
  struct proc *newp;          // Puntatore alla nuova struct proc per il figlio
  int result;

  KASSERT(curproc != NULL); // Assicurati che ci sia un processo corrente (genitore)

  // 1. Crea una nuova struttura proc per il figlio
  //    Il nome è spesso lo stesso del genitore (o modificato)
  newp = proc_create_runprogram(curproc->p_name);
  if (newp == NULL) {
    return ENOMEM; // Errore: memoria esaurita
  }

  // 2. Duplica l'address space del genitore per il figlio
  //    as_copy (da dumbvm.c o dal sistema VM avanzato) alloca nuovi frame
  //    fisici e copia il contenuto delle pagine del genitore.
  result = as_copy(curproc->p_addrspace, &(newp->p_addrspace));
  if(result != 0){ // as_copy ritorna 0 su successo, codice errore altrimenti
    proc_destroy(newp); // Pulisci la struct proc creata
    return result; // Ritorna l'errore da as_copy (es. ENOMEM)
  }

  // TODO: Collegare figlio e genitore (es. aggiungere newp a una lista
  // figli di curproc) per gestire correttamente la terminazione del genitore.

  // 3. Duplica il trapframe del genitore per il figlio
  //    Il figlio ha bisogno di una copia dello stato CPU del genitore
  //    per riprendere l'esecuzione dal punto giusto dopo fork.
  tf_child = kmalloc(sizeof(struct trapframe));
  if(tf_child == NULL){
    // as_destroy(newp->p_addrspace); // Libera l'address space duplicato!
    proc_destroy(newp); // Pulisci la struct proc
    return ENOMEM;
  }
  memcpy(tf_child, ctf, sizeof(struct trapframe)); // Copia il contenuto

  // 4. Crea il nuovo thread kernel per il figlio
  //    Passa il trapframe duplicato (tf_child) come argomento
  //    alla funzione di avvio del thread figlio (call_enter_forked_process).
  result = thread_fork(
		 curthread->t_name, // Nome del thread (può essere modificato)
                 newp, // Associa il nuovo thread al nuovo processo figlio
		 call_enter_forked_process, // Funzione da eseguire nel nuovo thread
		 (void *)tf_child,  // Argomento per la funzione (il trapframe figlio)
                 (unsigned long)0/*unused*/);

  // Se la creazione del thread fallisce...
  if (result){
    // as_destroy(newp->p_addrspace); // Libera l'address space
    proc_destroy(newp); // Libera la struct proc
    kfree(tf_child); // Libera la copia del trapframe
    return result; // Ritorna l'errore da thread_fork (es. ENOMEM)
  }

  // 5. Imposta il valore di ritorno per il processo genitore
  //    Il genitore riceve il PID del figlio appena creato.
  *retval = newp->p_pid;

  // Successo
  return 0;
}
#endif // OPT_FORK
```

**`enter_forked_process(struct trapframe *tf)` (`syscall.c`):**

*   Questa funzione viene eseguita dal *nuovo thread* (figlio).
*   Riceve il puntatore al trapframe *duplicato*.
*   Deve modificare questo trapframe per impostare il valore di ritorno di `fork` a 0 per il figlio.
*   Deve avanzare l'EPC per evitare di richiamare `fork`.
*   Deve attivare l'address space del figlio (anche se `as_activate` viene chiamato anche da `mips_usermode`).
*   Deve chiamare `mips_usermode()` passando il trapframe modificato per passare a user mode e iniziare l'esecuzione del figlio.

```c
void
enter_forked_process(struct trapframe *tf)
{
#if OPT_FORK
	// Duplica il trapframe sullo stack del kernel del figlio.
	// Questo è importante perché il tf originale (passato come argomento)
	// era stato allocato con kmalloc nel genitore e verrà liberato lì (o no?).
	// Lavorare su una copia locale sullo stack è più sicuro.
	struct trapframe forkedTf = *tf;
        // Libera la memoria allocata dal genitore per tf_child?
        // kfree(tf); // Se il genitore non lo fa. Design da chiarire.

	// Imposta il valore di ritorno per il processo figlio: fork ritorna 0.
	forkedTf.tf_v0 = 0;
        // Segnala successo
        forkedTf.tf_a3 = 0;

	// Avanza il program counter per tornare all'istruzione DOPO la syscall fork
	forkedTf.tf_epc += 4;

	// Attiva l'address space del processo figlio (ora curproc)
	as_activate(); // Potrebbe non essere strettamente necessario se mips_usermode lo fa

	// Passa a user mode usando il trapframe modificato
	mips_usermode(&forkedTf);
#else
	(void)tf;
#endif
}
```

**Integrazione nel Dispatcher `syscall()` (`syscall.c`):**

```c
 // Dentro lo switch(callno) in syscall():
 #if OPT_FORK
     case SYS_fork:
         // Chiama sys_fork passando il trapframe del genitore (tf)
         // e un puntatore a retval dove sys_fork scriverà il PID del figlio.
         err = sys_fork(tf, &retval);
         // err conterrà 0 in caso di successo, o un codice errore.
         // retval conterrà il PID del figlio in caso di successo.
         break;
 #endif
```

**Aggiungere Prototipo (`syscall.h`):**

```c
 // Aggiungere dentro #if OPT_SYSCALLS
 #if OPT_FORK
 int sys_fork(struct trapframe *ctf, pid_t *retval);
 #endif
```

---

## Parte 6: Compilazione e Test (Completo)

**Obiettivo:** Compilare e testare `waitpid`, `getpid` e (se implementato) `fork`.

**Passo-Passo:**

1.  **Configurazione Kernel:**
    *   Abilita le opzioni `OPT_WAITPID` e (se implementato) `OPT_FORK` nel file di configurazione.
    *   Assicurati che i file modificati (`proc.c`, `proc_syscalls.c`, `syscall.c`) siano compilati.
2.  **Compilazione:** Ciclo `config`, `bmake depend`, `bmake`, `bmake install`.
3.  **Esecuzione e Test:**
    *   `cd $HOME/os161/root`
    *   `sys161 kernel-NOME_CONF`
    *   **Test `getpid`/`waitpid` (senza `fork`):**
        *   Potrebbe essere necessario modificare `kern/main/menu.c` (la funzione `common_prog`) per lanciare un processo, ottenere il suo PID con `sys_getpid` (chiamato internamente o tramite una funzione helper che legga `proc->p_pid`), e poi chiamare `sys_waitpid` sul PID ottenuto, come suggerito nello scenario della slide 4 del PDF "PdS 2025 - Laboratorio 5". Si dovrebbe verificare che `sys_waitpid` ritorni il PID corretto e lo stato di uscita del processo lanciato.
    *   **Test `fork`/`getpid`/`waitpid`:**
        *   Se `fork` è implementato, eseguire `p testbin/forktest`. Questo programma esegue vari test su `fork`, `getpid`, `waitpid` e `_exit`. Controllare che l'output corrisponda a quello atteso (processi figli creati, PID corretti, attesa completata, stati di uscita corretti).
        *   Eseguire altri test come `testbin/farm` o `testbin/sty` se disponibili.

**Output Atteso:** I test specifici (`forktest`, etc.) dovrebbero completare senza errori e mostrare l'interazione corretta tra processi padre e figli tramite `fork` e `waitpid`. Il test manuale modificando `common_prog` dovrebbe mostrare che il kernel può attendere correttamente un processo lanciato tramite `runprogram`.