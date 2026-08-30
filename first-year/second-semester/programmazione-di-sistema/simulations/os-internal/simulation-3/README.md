# Simulazione 3 - Sistemi Operativi Interni

## Indice
- [Analisi Indirizzi di Memoria](#analisi-indirizzi-di-memoria)
- [Conversione Indirizzo Logico a Fisico](#conversione-indirizzo-logico-a-fisico)
- [Allocazione della Memoria Fisica in dumbvm](#allocazione-della-memoria-fisica-in-dumbvm)
- [Implementazione di Locks e Condition Variables in OS161](#implementazione-di-locks-e-condition-variables-in-os161)

## Analisi Indirizzi di Memoria

## Dati
- **RAM**: 4MB (0x400000 bytes)
- **Limite Kernel/User**: 2GB (0x80000000)

## Analisi Indirizzi

### Indirizzo: 0x80803005
- **Tipo**: Kernel (> 0x80000000)
- **Validità fisica**: Non valido (> 4MB)
- **Validità logica**: Non valido (> 0x80400000)
- **Conclusione**: Indirizzo non valido né come fisico né come logico kernel

### Indirizzo: 0x312010
- **Tipo**: User (< 0x80000000)
- **Validità fisica**: Valido (< 4MB)
- **Validità logica**: Valido
- **Conclusione**: Indirizzo valido sia come fisico che come logico user

### Indirizzo: 0x532100
- **Tipo**: User (< 0x80000000)
- **Validità fisica**: Non valido (> 4MB)
- **Validità logica**: Valido
- **Conclusione**: Indirizzo valido solo come logico user

---

## Conversione Indirizzo Logico a Fisico

### Dati del problema
- **Indirizzo logico user**: 0x4010
- **as_pbase1**: 0x100000 (indirizzo fisico base per segmento 1)
- **as_pbase2**: 0x200000 (indirizzo fisico base per segmento 2)
- **as_vbase1**: 0x3000 (indirizzo virtuale base per segmento 1)
- **as_vbase2**: 0x6000 (indirizzo virtuale base per segmento 2)
- **as_npages1**: 2 (numero di pagine per segmento 1)
- **as_npages2**: 4 (numero di pagine per segmento 2)

### Analisi

1. **Verifica del segmento**:
   - L'indirizzo 0x4010 è maggiore di as_vbase1 (0x3000) e minore di as_vbase2 (0x6000)
   - Quindi appartiene al **segmento 1** (codice)

2. **Calcolo dell'offset**:
   - Offset = Indirizzo logico - as_vbase1 = 0x4010 - 0x3000 = 0x1010

3. **Calcolo dell'indirizzo fisico**:
   - Indirizzo fisico = as_pbase1 + offset = 0x100000 + 0x1010 = 0x101010

4. **Verifica di validità**:
   - Dimensione segmento 1 = as_npages1 * PAGE_SIZE = 2 * 4096 = 8192 bytes (0x2000)
   - Offset massimo consentito = 0x2000 - 1 = 0x1FFF
   - Offset calcolato = 0x1010, che è < 0x1FFF
   - L'indirizzo è valido (cade all'interno del segmento allocato)

### Conclusione
L'indirizzo logico user 0x4010 corrisponde all'indirizzo fisico 0x101010

---

## Allocazione della Memoria Fisica in dumbvm

### Domanda
La memoria fisica in dumbvm viene allocata in multipli di una pagina, nonostante sia uno schema di allocazione contigua, perché:

### Analisi delle opzioni

1. **Allocare per multipli di una pagina riduce la frammentazione interna**
   - **FALSO** ✗
   - Allocare per multipli di una pagina riduce la frammentazione esterna, non quella interna
   - In realtà, allocare per multipli di una pagina può aumentare la frammentazione interna, poiché potrebbe rimanere spazio inutilizzato all'interno delle pagine

2. **La MMU in MIPS ha una TLB, quindi la traduzione logico-fisica necessita di pagine**
   - **VERO** ✓
   - La MMU (Memory Management Unit) nel MIPS possiede una TLB (Translation Lookaside Buffer)
   - La TLB memorizza le corrispondenze tra pagine virtuali e frame fisici
   - Per funzionare correttamente, la memoria deve essere organizzata in pagine di dimensione fissa

3. **dumbvm implementa una page table**
   - **FALSO** ✗
   - dumbvm non implementa page table complete come in sistemi più complessi
   - Le page table sono strutture tipicamente utilizzate dai processi in sistemi con memoria virtuale avanzata

4. **kmalloc può allocare solo per multipli di pagine**
   - **FALSO** ✗
   - kmalloc è un allocatore di memoria kernel generico che può allocare blocchi di qualsiasi dimensione
   - Non è limitato a allocare solo multipli di pagine
   - Può allocare memoria in base al valore specificato con sizeof()

---

## Implementazione di Locks e Condition Variables in OS161

### Domanda 1
**La funzione `cv_wait` riceve un lock come parametro perché:**

1. **È necessario come parametro nella chiamata interna a `wchan_sleep`**
   - [ ] VERO
   - [X] FALSO
   - **Motivazione:** La funzione `wchan_sleep` si occuperà unicamente di rilasciare lo spinlock interno alla cv, non il lock esterno.

2. **Il thread chiamante deve essere il proprietario del lock**
   - [X] VERO
   - [ ] FALSO
   - **Motivazione:** Prima di mettersi in attesa, il thread chiamante deve rilasciare il lock che egli stesso aveva acquisito.

3. **Il lock deve essere rilasciato e riacquisito da `cv_wait`**
   - [X] VERO
   - [ ] FALSO
   - **Motivazione:** Questo è esattamente ciò che fa `cv_wait`: rilascia il lock quando il thread va in attesa (dopo aver acquisito lo spinlock interno) e lo riacquisisce quando il thread viene risvegliato.

4. **Il lock deve essere rilasciato e riacquisito da `wchan_sleep`**
   - [ ] VERO
   - [X] FALSO
   - **Motivazione:** Ciò che viene rilasciato dalla `wchan_sleep` è lo spinlock interno della cv.

### Domanda 2
**Il lock può essere implementato:**

1. **Da un semaforo binario, senza alcun altro elemento/requisito**
   - [ ] VERO
   - [X] FALSO
   - **Motivazione:** Servono altri parametri tra cui il nome del lock stesso e il proprietario corrente.

2. **Da un semaforo binario, più un elemento/requisito aggiuntivo**
   - [X] VERO
   - [ ] FALSO
   - **Motivazione:** Se non consideriamo il nome del lock, è necessario almeno il puntatore al thread possessore del lock.

3. **Da una condition variable**
   - [ ] VERO
   - [X] FALSO
   - **Motivazione:** Quella è un'altra struttura di sincronizzazione che non obbligatoriamente deve essere utilizzata.

4. **Da un wait channel**
   - [X] VERO
   - [ ] FALSO
   - **Motivazione:** Un wait channel, combinato con uno spinlock e un meccanismo per gestire l'ownership, può implementare un lock efficacemente.

### Domanda 3
**L'implementazione della funzione `lock_acquire` può includere una chiamata a:**

1. **Funzioni `spinlock_data_get` e `spinlock_data_testandset`**
   - [ ] VERO
   - [X] FALSO
   - **Motivazione:** Quelle sono istruzioni atomiche a basso livello. Viene utilizzata la `spinlock_acquire`.

2. **Funzione `P` su un semaforo**
   - [X] VERO
   - [ ] FALSO
   - **Motivazione:** Serve per mettersi in attesa sul contatore del semaforo.

3. **Funzione `cv_wait`**
   - [ ] VERO
   - [X] FALSO
   - **Motivazione:** Come specificato nella domanda precedente, le cv sono un costrutto supplementare ai lock mutex, non sono necessari per la loro implementazione e/o utilizzo.

4. **Funzione `wchan_sleep`**
   - [X] VERO
   - [ ] FALSO
   - **Motivazione:** Nel caso di una implementazione del lock tramite wait channels, il thread verrebbe messo in una coda di attesa per essere poi svegliato per l'acquisizione del lock.
