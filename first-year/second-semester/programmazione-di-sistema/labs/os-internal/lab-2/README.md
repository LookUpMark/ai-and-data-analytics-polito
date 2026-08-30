# LAB 2: System Calls in OS161

---

## Obiettivi Generali del Laboratorio

1.  **Eseguire un Programma Utente:** Comprendere il meccanismo di base con cui OS/161 avvia ed esegue programmi nello spazio utente.
2.  **Comprendere le System Call e le Trap:** Capire perché i programmi utente necessitano delle system call per interagire con il kernel, come funziona il meccanismo delle trap e il ruolo fondamentale del `trapframe`.
3.  **Implementare System Call Essenziali:** Scrivere il codice kernel per gestire tre system call fondamentali: `write()`, `read()`, e `_exit()`, permettendo ai programmi utente di eseguire I/O su console e terminare correttamente.

---

## Parte 1: Esecuzione di Programmi Utente e il Bisogno di System Call

### Lanciare un Programma Utente

Dopo aver avviato il kernel OS/161, si ottiene un prompt. Da qui, si possono eseguire programmi compilati per lo spazio utente che risiedono nel filesystem simulato. Il comando è `p <percorso_del_programma>`.

**Esempio:**
Per lanciare il programma `hello` (che stampa "Hello world!"):
```bash
OS/161 kernel [? for menu]: p testbin/hello
```
Per lanciare il programma `palin` (verifica palindromi):
```bash
OS/161 kernel [? for menu]: p testbin/palin
```

### Perché i Programmi Utente Falliscono Inizialmente?

Se provi a lanciare questi programmi su un kernel OS/161 *base* (senza le modifiche di questo laboratorio), molto probabilmente il sistema andrà in **panic** (si bloccherà con un messaggio di errore) o stamperà messaggi come "Unknown syscall XX". Questo accade perché ai programmi utente mancano i meccanismi per comunicare con il kernel e ottenere i servizi necessari. In particolare (Slide 4):

1.  **Mancanza di System Calls:** I programmi utente eseguono operazioni come la stampa su schermo (`printf` in C) o la lettura da tastiera (`scanf` o simili). A basso livello, queste funzioni della libreria C si traducono in richieste al sistema operativo tramite **system calls** (come `write` per scrivere, `read` per leggere, `_exit` per terminare). Se il kernel non è stato programmato per capire e gestire queste richieste, non può soddisfarle.
2.  **Gestione Memoria Limitata:** Il sistema di memoria iniziale (`dumbvm` senza deallocazione) è molto rudimentale.
3.  **Mancanza Argomenti/Sincronizzazione:** Meccanismi come passare argomenti (`argc`/`argv`) o una corretta sincronizzazione user/kernel potrebbero mancare.

### Il Meccanismo delle Trap e System Call

Per motivi di sicurezza e stabilità, un programma utente non può accedere direttamente alla memoria del kernel o chiamare funzioni del kernel come se fossero normali funzioni. Deve passare attraverso un meccanismo controllato:

1.  **Richiesta:** Il programma utente, quando necessita di un servizio kernel (es. scrivere sulla console), esegue un'istruzione macchina speciale (in MIPS: `syscall`). Questa istruzione contiene un **codice numerico** che identifica univocamente la system call richiesta (es. `SYS_write`). I parametri per la system call (es. dove scrivere, cosa scrivere, quanto scrivere) vengono caricati in registri specifici (`a0` a `a3`) secondo una convenzione prestabilita.
2.  **Trap:** L'istruzione `syscall` causa una **trap**, ovvero un'interruzione controllata dell'esecuzione del programma utente. Il controllo passa dalla modalità utente (user mode) alla modalità kernel (kernel mode).
3.  **Salvataggio Contesto (`trapframe`):** Prima che il kernel possa gestire la trap, lo stato completo della CPU del processo utente (valore di tutti i registri, program counter, etc.) viene salvato in una struttura dati speciale allocata nello stack del kernel: il **`trapframe`** (`struct trapframe`). Questo salvataggio è cruciale per poter riprendere l'esecuzione del programma utente esattamente da dove era stata interrotta.
4.  **Gestione Trap nel Kernel:** Il kernel esamina la causa della trap. Se è una system call, chiama una funzione centrale chiamata **syscall dispatcher** (la funzione `syscall()` nel nostro caso).
5.  **Dispatcher (`syscall()`):** Questa funzione legge il numero della system call dal `trapframe` (convenzionalmente salvato nel campo corrispondente al registro `v0`). Tramite una struttura `switch`, seleziona la funzione kernel appropriata (es. `sys_write`) per gestire quella specifica richiesta. Passa a questa funzione i parametri letti dai campi del `trapframe` corrispondenti ai registri `a0`-`a3`.
6.  **Esecuzione Funzione Kernel (`sys_...`):** La funzione specifica (es. `sys_write`) esegue l'operazione richiesta (es. stampare caratteri sulla console usando `putch`).
7.  **Ritorno al Dispatcher:** La funzione `sys_` restituisce un valore (o un codice di errore) al dispatcher `syscall()`.
8.  **Aggiornamento Trapframe per Ritorno:** Il dispatcher `syscall()` prepara il ritorno allo user space:
    *   Memorizza il **valore di ritorno** (se successo) o il **codice di errore** (se fallimento) nel campo del `trapframe` corrispondente al registro `v0`.
    *   Imposta un **flag di successo/errore** nel campo corrispondente al registro `a3` (0 per successo, 1 per errore).
    *   **Incrementa il Program Counter** salvato nel trapframe (`tf->tf_epc += 4`) per puntare all'istruzione *dopo* la `syscall`.
9.  **Ritorno a User Mode:** Il kernel esegue un'istruzione speciale per ripristinare lo stato della CPU dai valori salvati nel `trapframe` e ritorna all'esecuzione del programma utente in user mode.
10. **Gestione Ritorno in User Space:** Il codice della libreria C user space che ha originato la system call controlla il flag di errore (`a3`). Se c'è stato un errore, memorizza il codice di errore (da `v0`) nella variabile globale `errno` e restituisce -1 alla funzione C chiamante (es. `printf`). Se non ci sono stati errori, restituisce il valore di successo (da `v0`) alla funzione C chiamante.

### Perché le System Call Sono Necessarie? (Rif. Quesito #3)

Non si può semplicemente chiamare `kprintf` (una funzione kernel) da un programma utente? **No.**

*   **Protezione:** Il kernel e i processi utente vivono in spazi di indirizzamento separati e operano a livelli di privilegio differenti. Il kernel ha accesso a tutto, l'utente ha accesso limitato. Questa separazione protegge il kernel e gli altri processi da errori o intenti malevoli di un singolo programma.
*   **Controllo:** Funzioni come `kprintf` (e le sottostanti `putch`, etc.) accedono direttamente all'hardware o a risorse condivise. Permettere a qualsiasi programma utente di chiamarle direttamente sarebbe caotico e insicuro.
*   **Interfaccia Stabile:** Le system call definiscono un'interfaccia stabile e ben definita (una API) tra user space e kernel space. Il kernel garantisce che, se la system call viene usata correttamente, eseguirà l'operazione richiesta in modo sicuro.

**In breve:** Le system call sono il meccanismo **necessario, sicuro e controllato** che permette ai programmi utente con privilegi limitati di richiedere servizi al kernel che opera con privilegi elevati.

### Dove Trovare le Definizioni? (Slide 5)

*   **Numeri delle System Call:** I codici numerici (es. `SYS_write`, `SYS_read`, `SYS__exit`) sono definiti in:
    `kern/include/kern/syscall.h`
*   **Dispatcher e Gestione Trapframe:** La logica principale per la gestione delle trap da system call e l'uso del `trapframe` si trova in:
    `kern/syscall/syscall.c` (nella soluzione fornita) o talvolta in `kern/arch/mips/syscall/syscall.c`.
*   **Implementazioni `sys_...`:** Le funzioni che *fanno* il lavoro (come `sys_write`) si trovano tipicamente in:
    `kern/syscall/` (es. nel file `file_syscalls.c` della soluzione fornita).

---

## Parte 2: Implementazione delle System Call `read`, `write`, `_exit`

*(Basato sulle slide 6, 7 e sui file `file_syscalls.c`, `syscall.c`, `syscall.h`)*

Implementeremo ora le funzioni kernel `sys_read`, `sys_write` e `sys__exit` e le integreremo nel dispatcher `syscall()`.

**File Principali da Modificare:**

1.  `kern/syscall/file_syscalls.c`: Qui scriveremo le funzioni `sys_write`, `sys_read`, `sys__exit`. (Potresti dover creare questo file se non esiste).
2.  `kern/syscall/syscall.c`: Modificheremo la funzione `syscall()` per aggiungere i `case` nello `switch` per le nuove system call.
3.  `kern/include/syscall.h`: Aggiungeremo i prototipi per le nostre funzioni `sys_`.

**Convenzioni MIPS per le System Call (Riepilogo):**

*   **Input Kernel:** Syscall number in `v0`, Arg1 in `a0`, Arg2 in `a1`, Arg3 in `a2`, Arg4 in `a3`.
*   **Output Kernel (Successo):** Return value in `v0`, `a3 = 0`.
*   **Output Kernel (Errore):** Error code (errno) in `v0`, `a3 = 1`.
*   **Kernel deve fare:** `tf->tf_epc += 4`.

---

### Task 2.1: Implementare `sys_write`

**Obiettivo:** Gestire le richieste di scrittura da user space, limitatamente a `STDOUT_FILENO` (1) e `STDERR_FILENO` (2), inviando l'output alla console.

**Ragionamento:**

1.  Creare `sys_write(int fd, userptr_t buf_ptr, size_t size)` in `file_syscalls.c`.
2.  Validare `fd`: se non è 1 o 2, ritorna -1 (errore).
3.  Iterare da `i = 0` a `size - 1`.
4.  **!!! PROBLEMA DI SICUREZZA !!!:** Per ogni `i`, leggere il carattere dal buffer utente `buf_ptr[i]`. **L'implementazione fornita fa un cast diretto `(char *)buf_ptr` e accede a `p[i]`. Questo è INSICURO perché accede direttamente alla memoria utente dal kernel.** L'approccio corretto richiederebbe `copyin()` per copiare i dati in modo sicuro nel kernel prima di usarli. *Noi spiegheremo il codice fornito, ma questa è una grave vulnerabilità.*
5.  Stampare il carattere sulla console usando `putch(char)`.
6.  Restituire `size` in caso di successo.

**Codice (`kern/syscall/file_syscalls.c`):**

```c
#include <types.h>
#include <kern/unistd.h> // Per STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO
#include <clock.h>
#include <copyinout.h> // Header per copyin/copyout (non usate qui!)
#include <syscall.h>
#include <lib.h>       // Per kprintf, putch, getch
#include <proc.h>      // Per proc_getas
#include <thread.h>    // Per thread_exit
#include <addrspace.h> // Per as_destroy

int
sys_write(int fd, userptr_t buf_ptr, size_t size)
{
  int i;
  // !!! Cast diretto e accesso a memoria utente -> INSICURO !!!
  char *p = (char *)buf_ptr;

  // 1. Validare File Descriptor (solo console)
  if (fd != STDOUT_FILENO && fd != STDERR_FILENO) {
    kprintf("sys_write supported only to stdout/stderr\n"); // Debug kernel
    return -1; // Errore generico
  }

  // 2. Ciclo di scrittura (con accesso INSICURO)
  for (i = 0; i < (int)size; i++) {
    // Accesso diretto a p[i] (memoria utente)
    char c = p[i];
    // Stampa carattere sulla console
    putch(c);
  }

  // 3. Successo: ritorna il numero di byte richiesti
  return (int)size;
}
```

**Spiegazione:**

*   Include necessari per costanti e funzioni kernel.
*   Parametri `fd` (file descriptor), `buf_ptr` (puntatore user), `size` (numero byte).
*   Cast **insicuro** di `buf_ptr`.
*   Controllo `fd` per limitare a stdout/stderr.
*   Ciclo `for` che legge **insicuramente** da `p[i]` e stampa con `putch`.
*   Ritorna `size` in caso di successo (assumendo che `putch` non fallisca).

**Discussione Alternative (Rif. Quesito #1):**

*   Sostituire `putch(p[i])` con `kprintf("%c", p[i])` produrrebbe lo stesso risultato sulla console ma manterrebbe lo stesso problema di sicurezza.
*   Sostituire l'intero ciclo con `kprintf("%s", p)` sarebbe funzionalmente diverso (orientato a stringhe C NUL-terminate, non a buffer di dimensione fissa) e ancora più **insicuro** perché `kprintf` potrebbe leggere oltre `size` o causare page fault.
*   `kprintf("%s", &p)` è un errore di tipo.
*   `file_write(...)` implicherebbe una gestione file non richiesta qui.
*   **Conclusione:** Per `sys_write`, l'approccio sicuro richiede `copyin` per portare i dati nel kernel, poi `putch` (o simile) per l'output.

---

### Task 2.2: Implementare `sys_read`

**Obiettivo:** Gestire le richieste di lettura da user space, limitatamente a `STDIN_FILENO` (0), leggendo dalla console.

**Ragionamento:**

1.  Creare `sys_read(int fd, userptr_t buf_ptr, size_t size)` in `file_syscalls.c`.
2.  Validare `fd`: se non è 0, ritorna -1.
3.  Iterare da `i = 0` a `size - 1`.
4.  Leggere un carattere dalla console usando `getch()`. Questa funzione è bloccante (attende l'input).
5.  Controllare il ritorno di `getch()`: se è < 0, significa EOF o errore; ritornare `i` (numero di byte letti finora).
6.  **!!! PROBLEMA DI SICUREZZA !!!:** Scrivere il carattere letto (`c`) nel buffer utente `buf_ptr[i]`. **L'implementazione fornita usa l'accesso diretto `p[i] = c`, che è INSICURO.** L'approccio corretto richiederebbe `copyout()` per copiare il carattere dal kernel allo spazio utente in modo sicuro.
7.  Se il ciclo termina normalmente, restituire `size`.

**Codice (`kern/syscall/file_syscalls.c`):**

```c
int
sys_read(int fd, userptr_t buf_ptr, size_t size)
{
  int i;
  // !!! Cast diretto e accesso a memoria utente -> INSICURO !!!
  char *p = (char *)buf_ptr;

  // 1. Validare File Descriptor (solo console)
  if (fd != STDIN_FILENO) {
    kprintf("sys_read supported only from stdin\n"); // Debug kernel
    return -1; // Errore generico
  }

  // 2. Ciclo di lettura (con accesso INSICURO)
  for (i = 0; i < (int)size; i++) {
    // Leggi carattere dalla console (bloccante)
    char c = getch();

    // Controlla EOF/Errore (getch < 0)
    if (c < 0) {
      // Fine input o errore, ritorna quanti byte letti finora
      return i;
    }

    // Scrivi carattere nel buffer utente (Accesso INSICURO)
    p[i] = c;

    // Opzionale: interrompere su newline?
    // if (c == '\n') { return i + 1; } // Comportamento tipico lettura riga
  }

  // 3. Successo: letti tutti i 'size' byte richiesti senza EOF/errore
  return (int)size;
}
```

**Spiegazione:**

*   Simile a `sys_write` per include e parametri.
*   Cast **insicuro** di `buf_ptr`.
*   Controllo `fd` per limitare a stdin.
*   Ciclo `for` che legge da `getch()` e scrive **insicuramente** in `p[i]`.
*   Gestione del ritorno di `getch()` per EOF/errore.
*   Ritorna `i` (byte letti) su EOF/errore, `size` se il buffer viene riempito.

---

### Task 2.3: Implementare `sys__exit`

**Obiettivo:** Permettere a un processo utente di terminare rilasciando le sue risorse principali (memoria) e terminando il thread kernel associato.

**Ragionamento:**

1.  Creare `sys__exit(int status)` in `file_syscalls.c`. (Notare doppio underscore: `SYS__exit`).
2.  Ottenere puntatore all'`addrspace` del processo corrente (`proc_getas()`).
3.  **Distruggere l'Address Space:** Chiamare `as_destroy(as)`. Questa funzione (specialmente se modificata come nel Lab 3) dovrebbe deallocare tutta la memoria fisica associata alle regioni del processo (codice, dati, stack).
4.  **Terminare il Thread Kernel:** Chiamare `thread_exit()`. Questo termina il thread corrente, dealloca le sue risorse kernel, e passa il controllo allo scheduler. **Non ritorna.**
5.  Ignorare `status` per ora (`(void)status;`).
6.  Aggiungere `panic()` dopo `thread_exit` come sicurezza, non dovrebbe mai essere raggiunta.

**Codice (`kern/syscall/file_syscalls.c`):**

```c
void
sys__exit(int status)
{
  /* Ottieni l'address space del processo corrente */
  struct addrspace *as = proc_getas();
  KASSERT(as != NULL); // Assicurati che esista

  /* Distruggi l'address space (libera memoria utente) */
  as_destroy(as);

  /* Termina il thread kernel associato (non ritorna) */
  thread_exit();

  /* Se thread_exit ritorna, è un errore grave */
  panic("thread_exit returned (should not happen)\n");

  /* Indica che 'status' non è usato (per ora) */
  (void) status;
}
```

**Spiegazione:**

*   Ottiene l'address space.
*   Chiama `as_destroy` (fondamentale per liberare memoria, specialmente dopo il Lab 3).
*   Chiama `thread_exit` per terminare l'esecuzione.

---

### Task 2.4: Integrare le Syscall nel Dispatcher (`syscall.c`)

**Obiettivo:** Modificare la funzione `syscall()` per riconoscere i numeri `SYS_write`, `SYS_read`, `SYS__exit` e chiamare le funzioni `sys_` corrispondenti.

**Ragionamento:**

1.  Aprire `kern/syscall/syscall.c` e trovare `syscall(struct trapframe *tf)`.
2.  Localizzare lo `switch (callno)`, dove `callno = tf->tf_v0`.
3.  Aggiungere `case SYS_write:`, `case SYS_read:`, `case SYS__exit:`.
4.  **Dentro ogni `case`:**
    *   Estrarre gli argomenti dai registri del trapframe (`tf->tf_a0`, `tf->tf_a1`, ...).
    *   Chiamare la funzione `sys_` corrispondente (`sys_write`, `sys_read`, `sys__exit`).
    *   Gestire il valore di ritorno di `sys_read`/`sys_write` per impostare `retval` e `err` correttamente.
    *   Per `sys__exit`, semplicemente chiamarla (non ritorna).
5.  Il codice *dopo* lo switch gestisce già l'aggiornamento di `tf->tf_v0`, `tf->tf_a3` e `tf->tf_epc`.

**Codice (`kern/syscall/syscall.c` - Aggiunte allo `switch`):**

```c
 // Dentro la funzione syscall(struct trapframe *tf)
 // ...
 switch (callno) {
     // ... case SYS_reboot, SYS___time ...

     /* ----- AGGIUNTE PER LAB 2 ----- */
     case SYS_write:
         // Estrai args: fd=a0, buf=a1, size=a2
         retval = sys_write((int)tf->tf_a0, (userptr_t)tf->tf_a1, (int)tf->tf_a2);
         if (retval < 0) { // Errore da sys_write
             // La soluzione usa ENOSYS, ma potrebbe essere EBADF, etc.
             err = ENOSYS;
         } else { // Successo
             err = 0;
             // retval contiene già i byte scritti
         }
         break;

     case SYS_read:
         // Estrai args: fd=a0, buf=a1, size=a2
         retval = sys_read((int)tf->tf_a0, (userptr_t)tf->tf_a1, (int)tf->tf_a2);
         if (retval < 0) { // Errore da sys_read
             err = ENOSYS; // Come sopra
         } else { // Successo
             err = 0;
             // retval contiene già i byte letti
         }
         break;

     case SYS__exit:
         // Estrai arg: status=a0
         // sys__exit non ritorna, quindi non impostiamo retval.
         // Impostiamo err=0 per coerenza prima della chiamata.
         err = 0;
         sys__exit((int)tf->tf_a0);
         // Non serve break perché non ritorna, ma buona pratica.
         break; // Non raggiunto
     /* --- FINE AGGIUNTE LAB 2 --- */

     default:
         kprintf("Unknown syscall %d\n", callno);
         err = ENOSYS;
         break;
 } // Fine switch

 // ... Codice comune per gestione err/retval e incremento tf->epc ...
```

**Spiegazione:**

*   Aggiunti i `case` per le tre nuove system call.
*   Estratti correttamente gli argomenti dai registri `tf->aX`.
*   Chiamate le funzioni `sys_` implementate.
*   Gestito il valore di ritorno (`retval`) e l'errore (`err`) per `read`/`write`.
*   Chiamata diretta a `sys__exit` che non ritorna.

---

### Task 2.5: Aggiungere Prototipi (`syscall.h`)

**Obiettivo:** Dichiarare i prototipi delle nuove funzioni `sys_` nell'header file corretto.

**Ragionamento:**

1.  Aprire `kern/include/syscall.h`.
2.  Aggiungere le dichiarazioni per `sys_write`, `sys_read`, e `sys__exit` insieme agli altri prototipi `sys_`.

**Codice (`kern/include/syscall.h` - Aggiunte):**

```c
 #ifndef _SYSCALL_H_
 #define _SYSCALL_H_

 /* ... altri include e dichiarazioni ... */

 /*
  * Prototypes for IN-KERNEL entry points for system call implementations.
  */

 int sys_reboot(int code);
 int sys___time(userptr_t user_seconds, userptr_t user_nanoseconds);

 /* ----- AGGIUNTE PER LAB 2 ----- */
 int sys_write(int fd, userptr_t buf_ptr, size_t size);
 int sys_read(int fd, userptr_t buf_ptr, size_t size);
 void sys__exit(int status); // Notare il doppio underscore
 /* --- FINE AGGIUNTE LAB 2 --- */

 #endif /* _SYSCALL_H_ */
```

**Spiegazione:** Aggiunti semplicemente i prototipi corretti per le funzioni implementate in `file_syscalls.c`.

---

## Parte 3: Compilazione e Test

**Obiettivo:** Compilare il kernel con le modifiche e verificare che i programmi utente base funzionino.

**Passo-Passo:**

1.  **Configurazione Kernel:**
    *   Assicurati che `kern/syscall/file_syscalls.c` sia compilato. Verifica il file `kern/conf/conf.kern` (o file inclusi) e aggiungi `file kern/syscall/file_syscalls.c` se necessario.
    *   Scegli una configurazione kernel (es. `ASST1`, o creane una `LAB2`) che includa questo file.
2.  **Compilazione:**
    *   `cd $HOME/os161/os161-base-2.x/kern/conf`
    *   `./config NOME_CONF`
    *   `cd ../compile/NOME_CONF`
    *   `bmake depend && bmake && bmake install`
3.  **Esecuzione e Test:**
    *   `cd $HOME/os161/root`
    *   `sys161 kernel-NOME_CONF`
    *   Al prompt del kernel, testa i programmi:
        *   `p testbin/hello` -> Dovrebbe stampare "Hello world!" e tornare al prompt. (Test `write`, `_exit`)
        *   `p testbin/palin` -> Dovrebbe chiedere input, leggere, stampare risultato e tornare al prompt. (Test `read`, `write`, `_exit`)
        *   `p bin/cat` -> Dovrebbe leggere input finché non riceve EOF (Ctrl+D), poi ristampare l'input e tornare al prompt. (Test `read`, `write`, `_exit`)

**Output Atteso:** I programmi dovrebbero avviarsi, interagire tramite console (lettura/scrittura), e terminare correttamente senza causare panic nel kernel, restituendo al prompt di OS/161.