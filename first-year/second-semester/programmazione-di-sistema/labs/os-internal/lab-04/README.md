# LAB 4: Sincronizzazione in OS161

---

## Obiettivi Generali del Laboratorio

1.  **Introduzione alla Sincronizzazione:** Comprendere i problemi di concorrenza che sorgono in un sistema operativo multi-thread/multi-processo e la necessità di meccanismi per coordinare l'accesso alle risorse condivise.
2.  **Conoscere le Primitive Base di OS/161:** Familiarizzare con le primitive di sincronizzazione già fornite: Spinlock, Semafori e Wait Channels.
3.  **Implementare i Lock:** Realizzare una primitiva di lock per la mutua esclusione che eviti il busy-waiting, introducendo il concetto di *ownership*.
4.  **Implementare le Condition Variables (CV):** Realizzare le variabili di condizione per permettere ai thread di attendere che una specifica condizione diventi vera, lavorando in congiunzione con i lock.

---

## Parte 1: Primitive di Sincronizzazione Esistenti in OS/161 (Slide 3)

OS/161 fornisce già alcune primitive di base su cui costruiremo quelle più complesse:

1.  **Spinlock:**
    *   **Scopo:** Mutua esclusione a grana molto fine, adatta per proteggere sezioni critiche *molto brevi* (poche istruzioni macchina), specialmente all'interno del kernel o in contesti dove non è possibile dormire (interrupt handler).
    *   **Meccanismo:** **Busy-Waiting**. Un thread che tenta di acquisire uno spinlock già detenuto continua a ciclare (spin) controllando lo stato del lock finché non si libera. Questo consuma tempo CPU ma evita il sovraccarico di mettere il thread in sleep e risvegliarlo.
    *   **Funzioni:** `spinlock_acquire(struct spinlock *)`, `spinlock_release(struct spinlock *)`.
    *   **Implementazione interna (Rif. Quesito #1):** Spesso implementati usando istruzioni atomiche hardware come Test-and-Set. Una tecnica comune è "Test-and-Test-and-Set": prima si legge il valore del lock in un ciclo (`spinlock_data_get`) senza tentare di modificarlo (meno traffico sul bus di memoria in sistemi multiprocessore). Solo quando il lock *sembra* libero, si tenta l'operazione atomica `spinlock_data_testandset` che legge il valore, lo imposta a "occupato" e restituisce il valore *vecchio*, tutto in un'unica operazione indivisibile. L'acquire ha successo se il valore vecchio restituito era "libero".
        *   *Perché `_get` e `_testandset` (Q1)?* Si usa `_get` per il ciclo di busy-waiting primario perché è un'operazione di sola lettura, meno costosa (in termini di coerenza della cache e traffico sul bus) rispetto a `_testandset` che è un'operazione atomica read-modify-write. Si tenta la costosa `_testandset` solo quando `_get` indica che il lock potrebbe essere libero.
        *   *Equivalenza `spinlock_acquire` vs `spinlock_acquire2` (Q1)?* No. La versione `spinlock_acquire` fornita nel quesito esce correttamente dal loop quando `_testandset` ritorna 0 (successo). La versione `spinlock_acquire2` ha un errore logico: fa `continue` *anche* quando `_testandset` ritorna 0 (successo), quindi ciclerebbe all'infinito senza mai acquisire il lock. Dovrebbe avere un `break` al posto del `continue` nel caso `if (... == 0)`.
2.  **Semafori (di Dijkstra):**
    *   **Scopo:** Meccanismo più generale per il controllo dell'accesso a risorse (contatore >= 0) o per la segnalazione tra thread.
    *   **Meccanismo:** **Blocking (Sleep)**. Se un thread esegue `P()` (wait) su un semaforo il cui contatore è 0, il thread viene messo in stato di sleep e inserito in una coda di attesa associata al semaforo, rilasciando la CPU. Non c'è busy-waiting.
    *   **Funzioni:** `sem_create(name, initial_count)`, `sem_destroy(sem)`, `P(sem)` (wait/proberen), `V(sem)` (signal/verhogen).
    *   **Implementazione OS/161:** Usa internamente uno spinlock (`sem_lock`) per proteggere l'accesso al contatore (`sem_count`) e un Wait Channel (`sem_wchan`) per mettere in sleep e risvegliare i thread bloccati su `P()`.
3.  **Wait Channels (WChan):**
    *   **Scopo:** Meccanismo di base per mettere a dormire (`wchan_sleep`) e risvegliare (`wchan_wakeone`, `wchan_wakeall`) i thread.
    *   **Meccanismo:** Un WChan è essenzialmente una coda di thread in attesa associata a un nome (per debug). Le operazioni su WChan richiedono di possedere uno **spinlock** specifico, che viene rilasciato atomicamente durante `wchan_sleep` e riacquisito prima che `wchan_sleep` ritorni. Questo previene race condition tra il controllo della condizione per dormire e l'effettivo mettersi in sleep.

**Cosa Manca (Slide 3):** Mancano implementazioni dirette di **Lock** (mutex con sleep e ownership) e **Condition Variables**.

---

## Parte 2: Implementazione dei Lock

*(Basato sulle slide 4, 5, Q&A #2, Q&A #3 e codice `synch.c`/`synch.h`)*

**Obiettivo:** Implementare la primitiva Lock (`struct lock`) per fornire mutua esclusione senza busy-waiting.

### Concetto di Lock

*   **Mutua Esclusione:** Come gli spinlock, garantiscono che solo un thread alla volta possa entrare in una sezione critica protetta dal lock.
*   **Non Busy-Waiting (Slide 4):** A differenza degli spinlock, se un thread (`T2`) tenta di acquisire un lock già detenuto da un altro thread (`T1`), `T2` viene messo in **sleep** (bloccato) invece di ciclare attivamente. Verrà risvegliato quando `T1` rilascerà il lock.
*   **Ownership (Slide 4, Q&A #2):** Un lock ha un concetto di "proprietario" (owner). Solo il thread che ha acquisito con successo il lock (il proprietario corrente) può rilasciarlo. Tentare di rilasciare un lock non posseduto è un errore.
*   **Interfaccia (Slide 4):**
    *   `struct lock *lock_create(const char *name)`: Crea e inizializza un nuovo lock.
    *   `void lock_destroy(struct lock *lock)`: Distrugge un lock (deve essere non detenuto).
    *   `void lock_acquire(struct lock *lock)`: Acquisisce il lock. Se il lock è già detenuto, mette il thread corrente in sleep finché non viene liberato e può essere acquisito.
    *   `void lock_release(struct lock *lock)`: Rilascia il lock. Può essere chiamato solo dal thread che detiene il lock. Deve risvegliare un eventuale thread in attesa.
    *   `bool lock_do_i_hold(struct lock *lock)`: Restituisce `true` se il thread corrente detiene il lock, `false` altrimenti.

### Differenza Lock vs Spinlock (Rif. Q&A #3)

*   **Meccanismo di Attesa:** Spinlock usa busy-waiting, Lock usa sleep.
*   **Contesto d'Uso:**
    *   **Spinlock:** Adatto per sezioni critiche **molto brevi** (poche istruzioni) dove il costo di mettere un thread in sleep e risvegliarlo (context switch) sarebbe maggiore del tempo speso nel busy-waiting. Utile anche in contesti dove non si può dormire (es. interrupt handler). Da usare con **cautela** per evitare di sprecare CPU o causare deadlock se detenuti per troppo tempo.
    *   **Lock:** Adatto per sezioni critiche di durata **potenzialmente lunga** o dove l'attesa potrebbe essere lunga. Evita lo spreco di CPU del busy-waiting mettendo i thread in attesa in sleep. **Non usare** in interrupt handler.
*   **Ownership:** Il concetto è esplicito nei Lock, meno negli Spinlock (anche se implicitamente solo chi lo acquisisce può rilasciarlo).

### Differenza Lock vs Semaforo Binario (Rif. Q&A #2)

*   Un semaforo inizializzato a 1 può essere usato per ottenere mutua esclusione (come un lock binario).
*   **Differenza Chiave (Ownership):** Un semaforo non ha concetto di ownership. Qualsiasi thread può chiamare `V()` (signal) su un semaforo, anche se non è stato quel thread a chiamare `P()` (wait) precedentemente. Invece, un lock *deve* essere rilasciato dallo stesso thread che lo ha acquisito. Questa proprietà dei lock è utile per il debug e per prevenire certi tipi di errori.

### Implementazione Scelta (Slide 5, Codice)

Le slide menzionano due possibili strategie:
1.  Usare i Semafori: Implementare il lock usando un semaforo binario e gestendo l'ownership separatamente.
2.  Usare Spinlock e WChan: Simile all'implementazione interna dei semafori stessi.

La soluzione fornita in `synch.c`/`synch.h` usa la **Strategia 1 (Semafori)**, abilitata dalla macro `USE_SEMAPHORE_FOR_LOCK 1`. Analizziamo questa.

**Struttura `struct lock` (`synch.h`):**

```c
struct lock {
        char *lk_name;         // Nome per debug
#if USE_SEMAPHORE_FOR_LOCK
	struct semaphore *lk_sem; // Semaforo binario per la mutua esclusione
#else
	struct wchan *lk_wchan; // Alternativa: Wait channel per l'attesa
#endif
	struct spinlock lk_lock; // Spinlock per proteggere i dati interni del lock
        volatile struct thread *lk_owner; // Puntatore al thread proprietario
};
```

**Spiegazione Struttura:**

*   `lk_name`: Stringa per identificare il lock durante il debug.
*   `lk_sem`: Un puntatore a un semaforo. Verrà creato con valore iniziale 1. `P(lk_sem)` bloccherà se il lock è già preso, `V(lk_sem)` lo renderà disponibile.
*   `lk_lock`: Uno **spinlock** usato per proteggere l'accesso agli altri campi della struttura `lock` stessa (in particolare `lk_owner`), garantendo atomicità nelle operazioni che li controllano o modificano.
*   `lk_owner`: Un puntatore al `struct thread` del thread che attualmente detiene il lock. È `NULL` se il lock è libero. `volatile` è usato perché il valore può essere modificato da un thread e letto da un altro concorrentemente (anche se protetto dallo spinlock).

**`lock_create(const char *name)` (`synch.c`):**

```c
struct lock *
lock_create(const char *name)
{
        struct lock *lock;

        lock = kmalloc(sizeof(*lock)); // Alloca memoria per la struct lock
        if (lock == NULL) { return NULL; }

        lock->lk_name = kstrdup(name); // Duplica il nome (permette al chiamante di liberare l'originale)
        if (lock->lk_name == NULL) { kfree(lock); return NULL; }

#if USE_SEMAPHORE_FOR_LOCK
        // Crea il semaforo binario associato, con nome uguale e valore iniziale 1
        lock->lk_sem = sem_create(lock->lk_name, 1);
	if (lock->lk_sem == NULL) { // Se la creazione del semaforo fallisce...
	  kfree(lock->lk_name); // ...libera le risorse allocate finora
	  kfree(lock);
	  return NULL;
	}
#else
        // Codice per l'alternativa con WChan (non attivo in questa configurazione)
	lock->lk_wchan = wchan_create(lock->lk_name);
	if (lock->lk_wchan == NULL) { /* ... gestione errore ... */ }
#endif
	// Inizializza il puntatore al proprietario a NULL (lock libero)
	lock->lk_owner = NULL;
	// Inizializza lo spinlock interno
	spinlock_init(&lock->lk_lock);

        return lock; // Ritorna il lock creato
}
```

**Spiegazione:** Alloca la struttura, duplica il nome, crea il semaforo binario (`sem_create` con count=1), inizializza `lk_owner` a `NULL` e inizializza lo spinlock interno.

**`lock_destroy(struct lock *lock)` (`synch.c`):**

```c
void
lock_destroy(struct lock *lock)
{
        KASSERT(lock != NULL);
        // Aggiungere KASSERT(lock->lk_owner == NULL); qui sarebbe una buona idea
        // per assicurarsi che il lock non sia detenuto quando viene distrutto.

	// Pulisce lo spinlock interno
	spinlock_cleanup(&lock->lk_lock);
#if USE_SEMAPHORE_FOR_LOCK
        // Distrugge il semaforo associato
        sem_destroy(lock->lk_sem);
#else
        // Codice per l'alternativa con WChan
	wchan_destroy(lock->lk_wchan);
#endif
        // Libera la memoria allocata per il nome e per la struttura lock
        kfree(lock->lk_name);
        kfree(lock);
}
```

**Spiegazione:** Libera tutte le risorse associate al lock: lo spinlock interno, il semaforo (o wchan), la stringa del nome e la struttura lock stessa.

**`lock_acquire(struct lock *lock)` (`synch.c`):**

```c
void
lock_acquire(struct lock *lock)
{
        KASSERT(lock != NULL);
        // Controllo (opzionale ma utile): Panico se si tenta di ri-acquisire un lock già posseduto
	// if (lock_do_i_hold(lock)) { panic or handle re-entrancy }
	// La soluzione fornita ha un controllo KASSERT(!(lock_do_i_hold(lock)))
	KASSERT(!(lock_do_i_hold(lock))); // Assicura che non stiamo ri-acquisendo

        // Non si può acquisire un lock (che può bloccare) da un interrupt handler
        KASSERT(curthread->t_in_interrupt == false);

#if USE_SEMAPHORE_FOR_LOCK
        /* Esegui P sul semaforo binario. Questo bloccherà se sem_count è 0
         * (cioè, se il lock è già detenuto). La chiamata a P gestisce
         * internamente lo sleep e il wake-up. */
        P(lock->lk_sem);

        /* Ora che P è ritornato, sappiamo che nessun altro può *superare* P
         * su questo semaforo finché non facciamo V. Siamo in mutua esclusione
         * rispetto ad altri acquirenti. Ora impostiamo l'owner.
         * Proteggiamo l'accesso a lk_owner con lo spinlock interno. */
	spinlock_acquire(&lock->lk_lock);
        // KASSERT(lock->lk_owner == NULL); // Dovrebbe essere NULL qui
        lock->lk_owner = curthread; // Imposta il proprietario al thread corrente
	spinlock_release(&lock->lk_lock);
#else
        // Codice per l'alternativa con WChan:
	// spinlock_acquire(&lock->lk_lock);
	// while (lock->lk_owner != NULL) { // Finché il lock è posseduto...
	//   wchan_sleep(lock->lk_wchan, &lock->lk_lock); // ...dormi sul wchan
	// }
        // // Ora il lock è libero
        // KASSERT(lock->lk_owner == NULL);
        // lock->lk_owner = curthread; // Diventa il nuovo proprietario
	// spinlock_release(&lock->lk_lock);
#endif
        // (void)lock; // Rimosso perché lock è usato
}
```

**Spiegazione (Implementazione con Semaforo):**

1.  `KASSERT`: Controlli di base (lock non nullo, non sono in interrupt, non detengo già il lock).
2.  `P(lock->lk_sem)`: Tenta di decrementare il semaforo. Se il contatore è 1 (lock libero), lo decrementa a 0 e procede. Se è 0 (lock occupato), il thread corrente viene messo in sleep sul wait channel del semaforo finché un altro thread non chiama `V(lock->lk_sem)`.
3.  `spinlock_acquire(&lock->lk_lock)`: Acquisisce lo spinlock interno. Questo è necessario per impostare `lk_owner` atomicamente rispetto ad altri thread che potrebbero leggere `lk_owner` (es. in `lock_do_i_hold`).
4.  `lock->lk_owner = curthread;`: Imposta il thread corrente come proprietario del lock.
5.  `spinlock_release(&lock->lk_lock)`: Rilascia lo spinlock interno.

**`lock_release(struct lock *lock)` (`synch.c`):**

```c
void
lock_release(struct lock *lock)
{
	KASSERT(lock != NULL);
	// Verifica l'ownership: solo il proprietario può rilasciare
	KASSERT(lock_do_i_hold(lock));

	// Proteggi l'accesso a lk_owner con lo spinlock interno
	spinlock_acquire(&lock->lk_lock);
        lock->lk_owner = NULL; // Rimuovi l'ownership
	spinlock_release(&lock->lk_lock); // Rilascia subito lo spinlock interno

#if USE_SEMAPHORE_FOR_LOCK
        /* Esegui V sul semaforo. Questo incrementa il contatore a 1
         * e risveglia un eventuale thread bloccato in P(lock->lk_sem). */
        V(lock->lk_sem);
#else
        // Codice per l'alternativa con WChan:
        // spinlock_acquire(&lock->lk_lock); // Ri-acquisisci spinlock per wakeone?
                                          // No, wakeone lo prende come argomento
        // wchan_wakeone(lock->lk_wchan, &lock->lk_lock); // Sveglia un thread in attesa
	// spinlock_release(&lock->lk_lock);
#endif
       // (void)lock; // Rimosso perché lock è usato
}
```

**Spiegazione (Implementazione con Semaforo):**

1.  `KASSERT`: Controlli (lock non nullo, *devo* essere il proprietario).
2.  `spinlock_acquire(&lock->lk_lock)`: Protegge la modifica di `lk_owner`.
3.  `lock->lk_owner = NULL;`: Annulla l'ownership.
4.  `spinlock_release(&lock->lk_lock)`: Rilascia la protezione su `lk_owner`.
5.  `V(lock->lk_sem)`: Incrementa il contatore del semaforo (torna a 1). Se c'erano thread bloccati in `P()` su questo semaforo, `V()` ne risveglierà uno (che poi tenterà di impostare `lk_owner`).

**`lock_do_i_hold(struct lock *lock)` (`synch.c`):**

```c
bool
lock_do_i_hold(struct lock *lock)
{
#if OPT_SYNCH // Assumendo che OPT_SYNCH sia definito per abilitare questa parte
        bool res;
	// Proteggi la lettura di lk_owner
	spinlock_acquire(&lock->lk_lock);
	// Confronta il proprietario salvato con il thread corrente
	res = (lock->lk_owner == curthread);
	spinlock_release(&lock->lk_lock);
	return res; // Ritorna true o false
#else
        (void)lock; // Sopprimi warning se non implementato
        // Ritorno dummy se il codice non è compilato
        return true; // SBAGLIATO: non mettere true! Meglio false o panic?
                     // La soluzione fornita ha return true.
#endif
}
```

**Spiegazione:**

1.  Acquisisce lo spinlock interno per leggere `lk_owner` in modo sicuro.
2.  Confronta `lock->lk_owner` con `curthread` (puntatore al thread corrente).
3.  Rilascia lo spinlock.
4.  Restituisce il risultato del confronto.

---

## Parte 3: Implementazione delle Condition Variables (CV)

*(Basato sulle slide 6, 7 e codice `synch.c`/`synch.h`)*

**Obiettivo:** Implementare le variabili di condizione (`struct cv`) per permettere ai thread di attendere (sleep) che una certa condizione diventi vera, coordinandosi tramite un lock associato.

### Concetto di Condition Variable

*   **Scopo:** Consentire a un thread di attendere *atomicamente* che una condizione legata a dati condivisi diventi vera, rilasciando temporaneamente il lock che protegge quei dati.
*   **Non è una Variabile:** Il nome "variabile di condizione" è fuorviante. La CV *non* memorizza la condizione stessa; è solo un punto di incontro (una coda di attesa) per i thread che aspettano quella condizione. La condizione deve essere verificata dal codice del thread usando variabili condivise protette dal lock.
*   **Associata a un Lock:** Ogni operazione su una CV (`wait`, `signal`, `broadcast`) richiede che il thread chiamante detenga un lock specifico. Questo lock garantisce la mutua esclusione durante il controllo della condizione e la modifica delle variabili condivise.
*   **Operazioni Atomiche (Semantica Mesa):**
    *   `cv_wait(cv, lock)`:
        1.  *Atomicamente*: Rilascia il `lock`.
        2.  Mette il thread corrente in sleep sulla coda di attesa della `cv`.
        3.  *Dopo essere stato risvegliato* (da `signal` o `broadcast`): Ri-acquisisce il `lock` prima di ritornare.
        **Importante (Mesa):** Dopo essere stato risvegliato, il thread deve *ri-controllare* la condizione in un ciclo (`while (!condizione)`), perché un altro thread potrebbe averla resa falsa di nuovo tra il risveglio e la ri-acquisizione del lock.
    *   `cv_signal(cv, lock)`: Risveglia *un* thread (se presente) che è in attesa su quella `cv`. Il thread chiamante deve detenere il `lock`.
    *   `cv_broadcast(cv, lock)`: Risveglia *tutti* i thread che sono in attesa su quella `cv`. Il thread chiamante deve detenere il `lock`.

### Implementazione (Slide 7, Codice)

Le slide richiedono l'implementazione usando **Wait Channel e Spinlock**. Questa è la strategia seguita nel codice fornito (assumendo `OPT_SYNCH` definito).

**Struttura `struct cv` (`synch.h`):**

```c
struct cv {
        char *cv_name;        // Nome per debug
	struct wchan *cv_wchan; // Wait channel per la coda dei thread in attesa
	struct spinlock cv_lock; // Spinlock per proteggere l'accesso a cv_wchan
                                // (Anche se wchan ha protezione interna,
                                // potrebbe servire per future estensioni o
                                // per coerenza con le altre strutture).
                                // La soluzione lo include.
};
```

**Spiegazione Struttura:**

*   `cv_name`: Nome per debug.
*   `cv_wchan`: Il wait channel dove i thread chiamanti `cv_wait` verranno messi a dormire.
*   `cv_lock`: Uno spinlock per proteggere la struttura CV stessa (principalmente il `cv_wchan`).

**`cv_create(const char *name)` (`synch.c`):**

```c
struct cv *
cv_create(const char *name)
{
        struct cv *cv;

        cv = kmalloc(sizeof(*cv)); // Alloca memoria per la struct cv
        if (cv == NULL) { return NULL; }

        cv->cv_name = kstrdup(name); // Duplica il nome
        if (cv->cv_name==NULL) { kfree(cv); return NULL; }

#if OPT_SYNCH
	// Crea il wait channel associato alla CV
	cv->cv_wchan = wchan_create(cv->cv_name);
	if (cv->cv_wchan == NULL) { // Gestione errore creazione wchan
	        kfree(cv->cv_name);
		kfree(cv);
		return NULL;
	}
        // Inizializza lo spinlock interno della CV
        spinlock_init(&cv->cv_lock);
#endif
        return cv; // Ritorna la CV creata
}
```

**Spiegazione:** Alloca la struttura, duplica il nome, crea il wait channel e inizializza lo spinlock interno.

**`cv_destroy(struct cv *cv)` (`synch.c`):**

```c
void
cv_destroy(struct cv *cv)
{
        KASSERT(cv != NULL);

#if OPT_SYNCH
	// Pulisce lo spinlock interno
	spinlock_cleanup(&cv->cv_lock);
	// Distrugge il wait channel (fallirà se ci sono thread ancora in attesa)
	wchan_destroy(cv->cv_wchan);
#endif
        // Libera la memoria per il nome e la struttura CV
        kfree(cv->cv_name);
        kfree(cv);
}
```

**Spiegazione:** Libera le risorse: spinlock, wait channel, nome e struttura CV.

**`cv_wait(struct cv *cv, struct lock *lock)` (`synch.c`):**

```c
void
cv_wait(struct cv *cv, struct lock *lock)
{
#if OPT_SYNCH
        KASSERT(lock != NULL); // Il lock non può essere NULL
	KASSERT(cv != NULL);   // La CV non può essere NULL
	// Il thread corrente DEVE detenere il lock associato prima di chiamare wait
	KASSERT(lock_do_i_hold(lock));

	// Acquisire lo spinlock della CV per operare sul wchan in modo sicuro
	spinlock_acquire(&cv->cv_lock);

	// Rilasciare il lock ESTERNO (passato come argomento)
        // Questo permette ad altri thread di acquisire il lock e potenzialmente
        // modificare la condizione rendendola vera.
	lock_release(lock);

	// Mettersi in sleep sul wait channel della CV.
        // Passiamo lo spinlock della CV a wchan_sleep. Questo spinlock
        // verrà rilasciato atomicamente da wchan_sleep prima di dormire
        // e riacquisito al risveglio.
	wchan_sleep(cv->cv_wchan, &cv->cv_lock);

	// Siamo stati svegliati (da signal o broadcast). Rilasciamo lo spinlock
        // interno della CV che wchan_sleep ci ha restituito.
	spinlock_release(&cv->cv_lock);

	// Ri-acquisire il lock ESTERNO prima di ritornare.
        // Questo è necessario per poter controllare la condizione in sicurezza
        // dopo il risveglio (nel ciclo while del chiamante).
	lock_acquire(lock);
#else
        // Sopprimi warning se non implementato
        (void)cv;
        (void)lock;
#endif
}
```

**Spiegazione:**

1.  `KASSERT`: Verifica che `cv` e `lock` siano validi e che il thread corrente detenga il `lock`.
2.  `spinlock_acquire(&cv->cv_lock)`: Acquisisce lo spinlock *della CV* per proteggere l'operazione sul `wchan`.
3.  `lock_release(lock)`: **Passaggio cruciale!** Rilascia il lock *esterno* (quello passato come argomento) per permettere ad altri thread (potenziali "segnalatori") di acquisirlo e modificare la condizione.
4.  `wchan_sleep(cv->cv_wchan, &cv->cv_lock)`: Mette il thread in sleep sul wait channel `cv_wchan`. **Atomicamente**, `wchan_sleep` rilascia lo spinlock `cv->cv_lock` proprio prima di mettere il thread in sleep. Al risveglio, `wchan_sleep` ri-acquisisce `cv->cv_lock` prima di ritornare.
5.  `spinlock_release(&cv->cv_lock)`: Rilascia lo spinlock *della CV* che era stato riacquisito da `wchan_sleep`.
6.  `lock_acquire(lock)`: **Altro passaggio cruciale!** Ri-acquisisce il lock *esterno* prima di ritornare al chiamante. Il chiamante ora detiene di nuovo il lock e può ricontrollare la condizione in sicurezza.

**`cv_signal(struct cv *cv, struct lock *lock)` (`synch.c`):**

```c
void
cv_signal(struct cv *cv, struct lock *lock)
{
#if OPT_SYNCH
        KASSERT(lock != NULL);
	KASSERT(cv != NULL);
	// Il thread corrente deve detenere il lock associato
	KASSERT(lock_do_i_hold(lock));

	// Acquisire lo spinlock della CV per operare sul wchan
	spinlock_acquire(&cv->cv_lock);

	// Sveglia UN thread (se ce n'è uno) in attesa sul wchan della CV.
        // Passiamo lo spinlock della CV come richiesto da wchan_wakeone.
	wchan_wakeone(cv->cv_wchan, &cv->cv_lock);

	// Rilascia lo spinlock della CV
	spinlock_release(&cv->cv_lock);
#else
	(void)cv;
	(void)lock;
#endif
}
```

**Spiegazione:**

1.  `KASSERT`: Verifica `cv`, `lock`, e ownership del `lock`.
2.  `spinlock_acquire(&cv->cv_lock)`: Protegge l'accesso al `wchan`.
3.  `wchan_wakeone(cv->cv_wchan, &cv->cv_lock)`: Risveglia un thread dalla coda del `wchan`. Il thread risvegliato uscirà da `wchan_sleep` (dentro `cv_wait`) e tenterà di riacquisire il lock esterno.
4.  `spinlock_release(&cv->cv_lock)`: Rilascia lo spinlock della CV.

**`cv_broadcast(struct cv *cv, struct lock *lock)` (`synch.c`):**

```c
void
cv_broadcast(struct cv *cv, struct lock *lock)
{
#if OPT_SYNCH
        KASSERT(lock != NULL);
	KASSERT(cv != NULL);
	// Il thread corrente deve detenere il lock associato
	KASSERT(lock_do_i_hold(lock));

	// Acquisire lo spinlock della CV per operare sul wchan
	spinlock_acquire(&cv->cv_lock);

	// Sveglia TUTTI i thread in attesa sul wchan della CV.
        // Passiamo lo spinlock della CV come richiesto da wchan_wakeall.
	wchan_wakeall(cv->cv_wchan, &cv->cv_lock);

	// Rilascia lo spinlock della CV
	spinlock_release(&cv->cv_lock);
#else
	(void)cv;
	(void)lock;
#endif
}
```

**Spiegazione:** Identica a `cv_signal`, ma usa `wchan_wakeall` per risvegliare tutti i thread in attesa invece di uno solo. Utile quando un evento rende la condizione vera per potenzialmente molti thread.

---

## Parte 4: Compilazione e Test

**Obiettivo:** Compilare il kernel con le nuove primitive di sincronizzazione e testarle.

**Passo-Passo:**

1.  **Configurazione Kernel:**
    *   Assicurati che l'opzione `OPT_SYNCH` (o un nome simile definito nel tuo `conf.kern`) sia abilitata nel file di configurazione del kernel che stai usando, in modo che il codice di implementazione per `lock_do_i_hold`, `cv_wait`, `cv_signal`, `cv_broadcast` venga compilato.
    *   Verifica che `kern/thread/synch.c` (o dove si trova il codice) sia incluso nella compilazione.
2.  **Compilazione:**
    *   Esegui il solito ciclo di compilazione:
        ```bash
        cd $HOME/os161/os161-base-2.x/kern/conf
        ./config NOME_CONF
        cd ../compile/NOME_CONF
        bmake depend && bmake && bmake install
        ```
3.  **Esecuzione e Test:**
    *   `cd $HOME/os161/root`
    *   `sys161 kernel-NOME_CONF`
    *   **Test Lock:** Esegui test di concorrenza che usano `lock_acquire`/`lock_release` per proteggere una sezione critica (es., i test `sy2`). Verifica che la mutua esclusione sia rispettata e che non ci siano deadlock o race condition evidenti.
    *   **Test CV (Slide 7):** Esegui i programmi di test menzionati che usano le condition variables: `sy3` (che chiama `cvtest`) e `sy4` (che chiama `cvtest2`). Questi test verificano scenari tipici come produttore-consumatore o barriere, dove i thread devono attendere condizioni specifiche usando `cv_wait` e segnalarle usando `cv_signal`/`cv_broadcast`. Osserva l'output per assicurarti che i thread si coordinino correttamente.

**Output Atteso:** I test di sincronizzazione (`sy2`, `sy3`, `sy4`) dovrebbero completare senza errori, panic, deadlock o output che indichino race condition. L'ordine dell'output dovrebbe riflettere la corretta sincronizzazione tra i thread.