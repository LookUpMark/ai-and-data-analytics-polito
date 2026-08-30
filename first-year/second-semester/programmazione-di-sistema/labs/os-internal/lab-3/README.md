# Laboratorio 3: Gestione della Memoria in OS161

Questo laboratorio esplora la gestione della memoria nel sistema operativo OS161, concentrandosi sul sistema DUMBVM. L'obiettivo è comprenderne il funzionamento, implementare la deallocazione delle pagine e migliorare l'efficienza complessiva attraverso la riduzione della frammentazione.

In OS161, lo spazio degli indirizzi è logicamente suddiviso in due aree principali:
*   **Spazio Kernel** (da `0x80000000` a `0xFFFFFFFF`): Dedicato al kernel del sistema operativo.
*   **Spazio Utente** (da `0x00000000` a `0x7FFFFFFF`): Utilizzato dai processi utente.

Lo spazio kernel è ulteriormente strutturato in segmenti mappati in modo specifico:
*   **Kseg0** (da `0x80000000` a `0x9FFFFFFF`): Utilizzato per il codice e i dati del kernel. Presenta un mapping diretto e non cacheable sulla memoria fisica (`indirizzo_fisico = indirizzo_virtuale - 0x80000000`).
*   **Kseg1** (da `0xA0000000` a `0xBFFFFFFF`): Usato per accedere a periferiche e dispositivi hardware, anch'esso con mapping diretto e non cacheable.
*   **Kseg2** (da `0xC0000000` a `0xFFFFFFFF`): Generalmente non sfruttato nelle configurazioni base di OS161, destinato a future espansioni o mapping più complessi tramite la MMU (Memory Management Unit).

OS161 include un allocatore di memoria fisica rudimentale chiamato DUMBVM (`dumbvm.c`). Le sue caratteristiche principali sono:
*   **Allocazioni Contigue:** Richiede blocchi di memoria fisica contigui.
*   **Nessuna Deallocazione Originale:** Nella sua versione di base, non rilascia mai la memoria una volta allocata, portando a un esaurimento progressivo della RAM.
*   **Frammentazione Interna:** Alloca sempre un numero intero di pagine (1 pagina = 4096 byte), indipendentemente dalla dimensione richiesta, generando sprechi di memoria all'interno delle pagine stesse.

Le operazioni di allocazione di pagine fisiche in DUMBVM si basano inizialmente su `getppages()` (in `kern/arch/mips/vm/dumbvm.c`), che a sua volta invoca `ram_stealmem()` (in `kern/arch/mips/vm/ram.c`).

Le modifiche proposte in questo laboratorio mirano a migliorare la gestione della memoria in OS161, specificamente:
1.  Introduzione di una funzione `getfreepages()` per riutilizzare pagine precedentemente liberate.
2.  Gestione della deallocazione della memoria alla terminazione di un thread/processo, modificando `as_destroy()` e `free_kpages()`.
3.  Modifica di `getppages()` per privilegiare l'uso di pagine già libere rispetto all'acquisizione di nuova memoria tramite `ram_stealmem()`.
Queste azioni puntano a ridurre la frammentazione e migliorare l'efficienza tracciando lo stato delle pagine fisiche.

---

## Quesito #1: Analisi di `ram_stealmem(npages)`

### Funzionamento e Finalità

La funzione `ram_stealmem(npages)` è un allocatore di memoria fisica a basso livello utilizzato in OS/161, principalmente durante la fase di bootstrap del sistema o come meccanismo di fallback quando altri metodi di allocazione falliscono. La sua operazione si basa sulla gestione di due puntatori globali: `first_phys_addr`, che indica l'inizio della memoria fisica *ancora disponibile*, e `last_phys_addr`, che rappresenta la fine della RAM utilizzabile dal kernel.

Quando `ram_stealmem` viene invocata con `npages` (il numero di pagine fisiche richieste):
1.  Determina la dimensione totale in byte necessaria: `size = npages * PAGE_SIZE`.
2.  Verifica la disponibilità di spazio: controlla se l'allocazione richiesta (`first_phys_addr + size`) supererebbe il limite superiore della memoria fisica (`last_phys_addr`). In caso di insufficiente memoria, la funzione restituisce `0`.
3.  Se lo spazio è sufficiente, l'indirizzo fisico di partenza per il blocco allocato viene impostato sull'attuale valore di `first_phys_addr`.
4.  Successivamente, `first_phys_addr` viene avanzato della dimensione appena allocata (`first_phys_addr += size`), "rubando" così il blocco di memoria per le future allocazioni.
5.  Infine, restituisce l'indirizzo fisico di partenza del blocco appena allocato.

Questo meccanismo garantisce sempre l'**allocazione di memoria contigua** poiché preleva un unico blocco consecutivo di memoria direttamente dalla regione disponibile e non ancora utilizzata, scorrendo linearmente il puntatore `first_phys_addr`.

### Pseudocodice

```plaintext
ram_stealmem (npages)
  paddr_t phys_addr // Indirizzo fisico di partenza del blocco allocato
  size_t size = npages * PAGE_SIZE // Dimensione totale in byte

  // first_phys_addr: prima pagina fisica libera disponibile
  // last_phys_addr: ultima pagina fisica disponibile
  if (first_phys_addr + size > last_phys_addr)
    return 0 // Memoria non disponibile

  phys_addr = first_phys_addr // L'indirizzo da restituire è l'attuale primo libero
  first_phys_addr += size // Avanza il puntatore, "rubando" le pagine

  return phys_addr // Ritorna l'indirizzo fisico di partenza del blocco allocato
```

### Compilazione ed Esecuzione

`ram_stealmem` è una funzione interna del kernel OS161, tipicamente definita in `kern/arch/mips/vm/ram.c`. Viene compilata come parte del processo di costruzione del kernel (es. `bmake && bmake install`). Non è direttamente invocabile dall'utente o da programmi applicativi, ma è chiamata internamente dal kernel, in particolare durante il bootstrap del sistema e dalla versione originale di `getppages` in `dumbvm.c`.

### Output Atteso

L'output di `ram_stealmem(npages)` è un valore di tipo `paddr_t`. In caso di successo, restituisce l'indirizzo fisico di base del blocco contiguo di `npages` pagine allocate. Se l'allocazione fallisce per insufficienza di memoria, restituisce `0`. Ci si aspetta che questa funzione esaurisca progressivamente la RAM disponibile, fornendo blocchi contigui di memoria fisica.

---

## Quesito #2: Indirizzi Fisici vs. Indirizzi Virtuali in OS/161

| Caratteristica      | Indirizzo Fisico (Physical Address)                     | Indirizzo Virtuale (Virtual Address)                            |
| :------------------ | :------------------------------------------------------ | :-------------------------------------------------------------- |
| **Definizione**     | Riferimento diretto a una specifica locazione in memoria RAM. | Indirizzo logico utilizzato da un programma, astratto dalla memoria RAM fisica. |
| **Utilizzo OS/161** | Utilizzato dal kernel a basso livello (es. bootstrap, `ram_stealmem`). Le regioni Kseg0 e Kseg1 del kernel mappano direttamente indirizzi fisici. | Usato da processi utente e dalla maggior parte del codice/dati del kernel (es. Kseg0). |
| **Motivazione**     | Accesso diretto all'hardware di memoria; allocazione delle risorse fisiche sottostanti. | **Isolamento/Protezione:** Ogni processo ha il proprio spazio indirizzi privato, prevenendo accessi non autorizzati.<br>**Astrazione:** I programmi vedono uno spazio di memoria contiguo, nascondendo la frammentazione fisica.<br>**Flessibilità:** Il sistema operativo può allocare pagine fisiche non contigue a un blocco virtuale contiguo.<br>**Condivisione:** Più processi possono mappare la stessa pagina fisica (es. librerie condivise), risparmiando RAM. |
| **Unicità**         | Unico per l'intero sistema hardware.                 | Unico all'interno del contesto di un singolo processo. Processi diversi possono usare gli stessi indirizzi virtuali per riferirsi a locazioni fisiche diverse. |
| **Traduzione**      | Utilizzato direttamente dal processore per accedere alla RAM. | Tradotto in indirizzo fisico dalla Memory Management Unit (MMU) o dalla Translation Lookaside Buffer (TLB) con l'assistenza del sistema operativo. |

**Esempi Pratici in OS/161:**
*   **Indirizzo Fisico:** Se `ram_stealmem(1)` viene chiamata all'avvio, potrebbe restituire `0x00120000`, che è l'indirizzo effettivo dell'inizio di un frame di pagina fisica nella RAM.
*   **Indirizzo Virtuale:**
    *   Il codice eseguibile di un programma utente viene tipicamente caricato a partire dall'indirizzo virtuale `0x00400000` all'interno dello spazio utente.
    *   Lo stack di un processo utente si trova in cima allo spazio utente virtuale, con l'indirizzo `USERSTACK` (tipicamente `0x7fffffff`).
    *   La macro kernel `PADDR_TO_KVADDR(paddr)` converte un indirizzo fisico `paddr` nel suo corrispondente indirizzo virtuale nel Kseg0 del kernel (aggiungendo `0x80000000`).
    *   La funzione `vm_fault` in `dumbvm.c` è responsabile di tradurre un `faultaddress` (indirizzo virtuale che ha causato un page fault) in un `paddr` (indirizzo fisico) per poi caricare la traduzione nella TLB.

---

## Quesito #3: Flusso di Deallocazione e Riutilizzo della Memoria (`DUMBVM_WITH_FREE=1`)

L'implementazione della deallocazione in `dumbvm.c`, attivata tramite la macro di compilazione `DUMBVM_WITH_FREE=1`, introduce la capacità di tracciare, liberare e riutilizzare le pagine di memoria fisica. Questo approccio riduce l'esaurimento irreversibile della RAM tipico della versione base di DUMBVM.

### Strutture Dati e Inizializzazione

Per gestire lo stato delle pagine fisiche, vengono introdotte due array globali:
*   `freeRamFrames`: Un array di `unsigned char` dove ogni elemento rappresenta un frame di pagina fisica. Un valore di `1` indica che il frame è libero, `0` che è occupato.
*   `allocSize`: Un array di `unsigned long` in cui `allocSize[i] = N` significa che un blocco di `N` pagine è stato allocato a partire dal frame fisico con indice `i`. Questo è fondamentale per `free_kpages` per sapere quante pagine liberare.
*   `nRamFrames`: Il numero totale di frame di pagina fisica disponibili nel sistema.
*   `allocTableActive`: Un flag booleano che indica se il sistema di gestione della memoria libera è attivo.
*   `freemem_lock`: Uno spinlock utilizzato per proteggere l'accesso concorrente a queste strutture dati, garantendo la coerenza in ambienti multi-threaded.

**Inizializzazione del Sistema di Memoria (`vm_bootstrap()`):**
Questa funzione è richiamata una volta all'avvio del sistema. Il suo compito è calcolare il numero totale di frame di RAM, allocare dinamicamente gli array `freeRamFrames` e `allocSize` utilizzando `kmalloc` (un allocatore kernel che ruba memoria da `ram_stealmem`), e inizializzarli. Tutti i frame sono inizialmente marcati come occupati (`0`) e le dimensioni di allocazione a `0`. Infine, `allocTableActive` viene impostato a `1` sotto protezione dello spinlock.

```c
// kern/arch/mips/vm/dumbvm.c (con DUMBVM_WITH_FREE=1)

#if DUMBVM_WITH_FREE

// Dichiarazioni globali (omesse per brevità, ma presenti nel file reale)
// static unsigned char *freeRamFrames = NULL;
// static unsigned long *allocSize = NULL;
// static long nRamFrames = 0;
// static volatile int allocTableActive = 0;
// static struct spinlock freemem_lock = SPINLOCK_INITIALIZER;

void
vm_bootstrap(void)
{
  int i;
  // Calcola il numero totale di frame di RAM disponibili
  nRamFrames = ((int)ram_getsize())/PAGE_SIZE;

  // Alloca memoria per i due array di tracciamento
  freeRamFrames = kmalloc(sizeof(unsigned char)*nRamFrames);
  if (freeRamFrames==NULL) return; // Gestione errore allocazione

  allocSize      = kmalloc(sizeof(unsigned long)*nRamFrames);
  if (allocSize==NULL) {
    kfree(freeRamFrames); // Libera la memoria se la seconda allocazione fallisce
    freeRamFrames = NULL;
    return;
  }

  // Inizializza tutti i frame come occupati e le dimensioni a zero
  for (i=0; i<nRamFrames; i++) {
    freeRamFrames[i] = (unsigned char)0; // 0 = occupato
    allocSize[i]     = 0;
  }

  // Attiva la tabella di allocazione sotto protezione del lock
  spinlock_acquire(&freemem_lock);
  allocTableActive = 1;
  spinlock_release(&freemem_lock);
}
#endif // DUMBVM_WITH_FREE
```

### Allocazione di Pagine Fisiche (`getppages`, `getfreeppages`)

Il processo di allocazione è modificato per tentare prima di riutilizzare le pagine libere.

**`getfreeppages(npages)`:**
Questa è la funzione principale per cercare blocchi di pagine libere e contigue. Scansiona l'array `freeRamFrames` alla ricerca del primo blocco di `npages` frame consecutivi marcati come liberi. Se trova un tale blocco (implementazione "first-fit"), li marca come occupati e registra la dimensione dell'allocazione in `allocSize` per consentire future deallocazioni corrette. Restituisce l'indirizzo fisico del primo frame trovato o `0` se non trova un blocco adatto. Tutte le operazioni sono protette da `freemem_lock`.

```c
// kern/arch/mips/vm/dumbvm.c (con DUMBVM_WITH_FREE=1)

static paddr_t
getfreeppages(unsigned long npages) {
  paddr_t addr;
  long i, first_candidate_idx = -1; // Indice del primo frame di una sequenza potenziale
  long found_block_idx = -1; // Indice del primo frame del blocco trovato
  long current_contiguous_free = 0; // Contatore di pagine libere contigue
  long target_npages = (long)npages;

  // Se la gestione della memoria libera non è attiva, non possiamo cercare pagine libere
  if (!isTableActive()) return 0;

  spinlock_acquire(&freemem_lock); // Protegge l'accesso a freeRamFrames e allocSize

  // Scansiona tutti i frame di RAM
  for (i = 0; i < nRamFrames; i++) {
    if (freeRamFrames[i] == 1) { // Se il frame corrente è libero
      if (current_contiguous_free == 0) { // Se è l'inizio di una nuova sequenza di pagine libere
        first_candidate_idx = i;
      }
      current_contiguous_free++;

      if (current_contiguous_free >= target_npages) { // Se abbiamo trovato un blocco sufficiente
        found_block_idx = first_candidate_idx;
        break; // Trovato il blocco, esci dal ciclo
      }
    } else { // Se il frame corrente è occupato
      current_contiguous_free = 0; // Reset della sequenza contigua
      first_candidate_idx = -1;
    }
  }

  if (found_block_idx >= 0) { // Se un blocco di pagine libere è stato trovato
    // Marca le pagine trovate come occupate
    for (i = found_block_idx; i < found_block_idx + target_npages; i++) {
      freeRamFrames[i] = (unsigned char)0; // 0 = occupato
    }
    // Registra la dimensione dell'allocazione a partire dal primo frame
    allocSize[found_block_idx] = target_npages;
    // Calcola l'indirizzo fisico del primo frame del blocco
    addr = (paddr_t)found_block_idx * PAGE_SIZE;
  }
  else { // Nessun blocco di pagine libere sufficiente trovato
    addr = 0;
  }

  spinlock_release(&freemem_lock);
  return addr;
}
```

**`getppages(npages)`:**
Questa è la funzione principale per l'allocazione di pagine fisiche. La versione modificata tenta prima di allocare le pagine tramite `getfreeppages`. Solo se `getfreeppages` fallisce (restituendo `0`), si ricorre a `ram_stealmem` per "rubare" nuova memoria. Se l'allocazione ha successo da qualsiasi fonte, e la gestione della memoria libera è attiva, `allocSize` viene aggiornato per tracciare la dimensione del blocco allocato.

```c
// kern/arch/mips/vm/dumbvm.c (con DUMBVM_WITH_FREE=1)

static paddr_t
getppages(unsigned long npages)
{
  paddr_t addr;

  // 1. Tenta di ottenere pagine libere e riutilizzabili
  addr = getfreeppages(npages);

  // 2. Se non sono state trovate pagine libere, ricorre a ram_stealmem
  if (addr == 0) {
    spinlock_acquire(&stealmem_lock); // ram_stealmem usa un suo lock
    addr = ram_stealmem(npages);
    spinlock_release(&stealmem_lock);
  }

  // 3. Se l'allocazione è riuscita e la tabella è attiva, aggiorna allocSize
  // Questo è importante per le deallocazioni future di blocchi da ram_stealmem
  if (addr != 0 && isTableActive()) {
    spinlock_acquire(&freemem_lock);
    allocSize[addr / PAGE_SIZE] = npages;
    spinlock_release(&freemem_lock);
  }

  return addr;
}
```

### Deallocazione di Pagine Fisiche (`freeppages`, `as_destroy`, `free_kpages`)

Il sistema ora supporta la deallocazione, rendendo la memoria riutilizzabile.

**`freeppages(addr, npages)`:**
Questa funzione è responsabile di marcare un blocco di `npages` pagine fisiche a partire da `addr` come libere. Converte l'indirizzo fisico `addr` nel suo indice di frame e poi imposta i bit corrispondenti in `freeRamFrames` a `1` (libero). Anche qui, l'operazione è protetta da `freemem_lock`. Include assert per la robustezza.

```c
// kern/arch/mips/vm/dumbvm.c (con DUMBVM_WITH_FREE=1)

static int
freeppages(paddr_t addr, unsigned long npages){
  long i, first_frame_idx;
  long num_pages_to_free = (long)npages;

  // Se la gestione della memoria libera non è attiva, non si può liberare
  if (!isTableActive()) return 0;

  first_frame_idx = addr / PAGE_SIZE;

  // Asserzioni per controllare la validità degli indici
  KASSERT(allocSize != NULL); // allocSize deve essere stato inizializzato
  KASSERT(nRamFrames > first_frame_idx); // first_frame_idx deve essere entro i limiti della RAM
  // KASSERT(allocSize[first_frame_idx] == num_pages_to_free); // Questa assertione è rischiosa se blocchi contigui vengono liberati parzialmente.

  spinlock_acquire(&freemem_lock); // Protegge l'accesso a freeRamFrames

  // Marca tutte le pagine del blocco come libere
  for (i = first_frame_idx; i < first_frame_idx + num_pages_to_free; i++) {
    freeRamFrames[i] = (unsigned char)1; // 1 = libero
  }
  // Resetta la dimensione dell'allocazione per questo blocco
  // Questo previene che free_kpages in futuro cerchi di liberare lo stesso blocco
  // usando un valore allocSize obsoleto.
  allocSize[first_frame_idx] = 0;

  spinlock_release(&freemem_lock);

  return 1; // Successo
}
```

**`as_destroy(struct addrspace *as)`:**
Questa funzione viene invocata quando un processo termina. Il suo ruolo è deallocare la memoria fisica associata all'address space del processo. Per ogni regione di memoria del processo (codice, dati, stack), `as_destroy` recupera l'indirizzo fisico di base e il numero di pagine e poi chiama `freeppages` per ciascuna. Infine, la struttura `addrspace` stessa viene liberata con `kfree(as)`.

```c
// kern/arch/mips/vm/dumbvm.c (con DUMBVM_WITH_FREE=1)

void as_destroy(struct addrspace *as){
  dumbvm_can_sleep(); // Indica che questa funzione può andare in sleep (per la gestione della memoria)

  // Libera le pagine fisiche per ciascuna regione dell'address space
  // as_pbaseX sono gli indirizzi fisici base delle regioni, as_npagesX il numero di pagine
  if (as->as_pbase1 != 0) freeppages(as->as_pbase1, as->as_npages1);
  if (as->as_pbase2 != 0) freeppages(as->as_pbase2, as->as_npages2);
  // Lo stack ha una dimensione fissa (DUMBVM_STACKPAGES)
  if (as->as_stackpbase != 0) freeppages(as->as_stackpbase, DUMBVM_STACKPAGES);

  // Libera la struttura addrspace stessa
  kfree(as);
}
```

**`free_kpages(vaddr_t addr)`:**
Questa funzione è usata dal kernel per deallocare memoria precedentemente allocata per uso interno (`alloc_kpages`). Converte l'indirizzo virtuale del kernel (`addr`) in un indirizzo fisico. Utilizza l'array `allocSize` (`allocSize[paddr / PAGE_SIZE]`) per determinare il numero originale di pagine che erano state allocate per quel blocco specifico. Questo è cruciale perché `free_kpages` non riceve il numero di pagine come argomento diretto, ma deve desumerlo dall'informazione di allocazione. Una volta ottenuto il numero di pagine, chiama `freeppages`.

```c
// kern/arch/mips/vm/dumbvm.c (con DUMBVM_WITH_FREE=1)

void
free_kpages(vaddr_t addr){
  // Controlla se la gestione della memoria libera è attiva
  if (isTableActive()) {
    // Converte l'indirizzo virtuale del kernel (Kseg0) in indirizzo fisico
    paddr_t paddr = addr - MIPS_KSEG0;
    // Calcola l'indice del primo frame
    long first_frame_idx = paddr / PAGE_SIZE;

    // Asserzioni per la validità degli indici e della tabella
    KASSERT(allocSize != NULL);
    KASSERT(nRamFrames > first_frame_idx);

    // Recupera il numero di pagine da liberare dal nostro record allocSize
    unsigned long num_pages_to_free = allocSize[first_frame_idx];

    // Se il valore è > 0, significa che c'era un'allocazione a partire da questo frame
    if (num_pages_to_free > 0) {
        freeppages(paddr, num_pages_to_free);
    }
    // else: se num_pages_to_free è 0, o è una deallocazione non tracciata
    // o un tentativo di liberare un blocco già libero/non inizializzato correttamente.
    // In un sistema di produzione, si potrebbero loggare questi casi.
  }
}
```

### Compilazione ed Esecuzione

Tutte le funzioni sopra menzionate vengono compilate come parte del kernel OS161 quando la direttiva di precompilazione `DUMBVM_WITH_FREE` è definita.
*   `vm_bootstrap()`: Eseguita una volta all'avvio del sistema.
*   `as_create()`, `as_define_region()`, `as_prepare_load()`: Chiamate durante la creazione e il caricamento di un nuovo processo utente.
*   `getppages()`: Chiamata da `as_prepare_load()` e `alloc_kpages()` per ottenere memoria.
*   `alloc_kpages()`: Usata da varie parti del kernel per l'allocazione dinamica della memoria per le proprie strutture dati.
*   `as_destroy()`: Invocata dal kernel quando un processo termina la sua esecuzione.
*   `free_kpages()`: Usata dal kernel per rilasciare memoria kernel precedentemente allocata.
*   `getfreeppages()` e `freeppages()`: Chiamate internamente dalle funzioni di allocazione/deallocazione di livello superiore.

### Comportamento del Sistema Atteso

Con l'implementazione di queste modifiche, il comportamento di OS161 in relazione alla gestione della memoria sarà significativamente migliorato:
*   **Riutilizzo della Memoria:** Le pagine fisiche precedentemente occupate da processi terminati o da strutture dati kernel non più necessarie vengono contrassegnate come libere e possono essere riutilizzate per nuove allocazioni. Questo è il beneficio principale.
*   **Riduzione della Dipendenza da `ram_stealmem`:** La funzione `getppages` ora tenta di riutilizzare la memoria esistente prima di ricorrere a `ram_stealmem`. Questo rallenta notevolmente l'esaurimento irreversibile della RAM, permettendo al sistema di gestire più processi o processi con cicli di vita più lunghi senza crash per mancanza di memoria.
*   **Miglioramento (Limitato) della Frammentazione:** Sebbene `getfreeppages` cerchi blocchi contigui (potenzialmente portando a frammentazione esterna nel lungo termine se le allocazioni/deallocazioni sono di dimensioni molto variabili), la capacità di riutilizzare la memoria è un passo cruciale verso una gestione più efficiente e sostenibile rispetto all'allocazione solo "a ruba".
*   **Correttezza di `free_kpages`:** La capacità di `free_kpages` di liberare il numero corretto di pagine dipende dalla manutenzione accurata e dalla consultazione di `allocSize` per tutte le allocazioni. Un errore in `allocSize` potrebbe portare a tentativi di liberare più o meno pagine del dovuto, causando corruzione o perdite di memoria.