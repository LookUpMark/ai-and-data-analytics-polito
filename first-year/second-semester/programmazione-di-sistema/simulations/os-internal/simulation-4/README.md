# Simulazione 4 - Sistemi Operativi Interni

## Istruzioni
Rispondere **YES/NO** a ognuna delle domande seguenti (motivazioni/spiegazioni richieste).

## Domanda 1: Thrashing e Working Set

In un sistema di memoria virtuale con un grado fisso di multiprogrammazione, il thrashing può verificarsi anche se la somma dei working set di tutti i processi è inferiore alla memoria fisica disponibile, ma l'algoritmo di sostituzione della pagina non è ottimizzato?

- [X] YES
- [ ] NO
- **Motivazione**: Anche quando la memoria fisica totale è teoricamente sufficiente per contenere tutti i working set, un algoritmo di sostituzione delle pagine non ottimizzato può causare thrashing. L'algoritmo potrebbe sostituire pagine che saranno richieste nuovamente a breve, causando frequenti page fault e continui swap in/swap out. Questo può accadere se l'algoritmo non riesce a identificare correttamente le pagine meno utili da sostituire, generando una cascata di page fault che porta al thrashing anche quando teoricamente ci sarebbe memoria sufficiente.

## Domanda 2: Inverted Page Table

In un sistema con paginazione basata su inverted page table (IPT), è possibile che più pagine virtuali di processi diversi condividano una singola entry della tabella delle pagine senza causare un conflitto nella traduzione degli indirizzi?

- [ ] YES
- [X] NO
- **Motivazione**: In un'Inverted Page Table (IPT), ogni entry corrisponde a un frame fisico specifico e può mappare solo una pagina virtuale di un processo alla volta. Ogni entry dell'IPT contiene l'identificatore del processo (PID) e il numero della pagina virtuale attualmente mappata a quel frame fisico. Se più pagine virtuali di processi diversi tentassero di condividere la stessa entry, ci sarebbe un conflitto nella traduzione degli indirizzi, poiché il sistema non potrebbe determinare quale pagina virtuale è effettivamente mappata a quel frame fisico. L'IPT è progettata specificamente per avere una corrispondenza uno-a-uno tra frame fisici e pagine virtuali attualmente in memoria.

## Domanda 3: Sostituzione Locale delle Pagine

In un sistema di paginazione a richiesta che utilizza la sostituzione delle pagine locale (a un singolo processo), un processo con frequenza di page fault molto elevata può comunque causare una riduzione della memoria fisica disponibile per altri processi?

- [ ] YES
- [X] NO
- **Motivazione**: Nei sistemi con sostituzione locale delle pagine, ogni processo riceve un'allocazione fissa di frame fisici che viene determinata al momento dell'avvio del processo o in base a politiche di allocazione specifiche. Quando un processo genera un page fault, il sistema operativo seleziona per la sostituzione solo una pagina tra quelle già assegnate a quel processo. Di conseguenza, anche se un processo ha una frequenza molto elevata di page fault, può sostituire solo le proprie pagine senza influire sulla memoria allocata ad altri processi. Questo isolamento protegge gli altri processi dagli effetti negativi di un singolo processo problematico, anche se può causare un rallentamento generale del sistema a causa dell'overhead di gestione dei frequenti page fault.

---

## Esercizio 1: Inverted Page Table (IPT)

### Problema
Si consideri un sistema a 64 bit con uno spazio di indirizzi virtuale di 2^48 Byte, una dimensione di pagina di 4KB e 16 GB di memoria fisica. Si supponga che il sistema utilizzi una inverted page table (IPT). Calcolare e spiegare quanto segue:
1. Il numero di bit necessari per il numero di frame fisico.
2. Il numero totale di entry nella inverted page table.
3. L'indirizzo virtuale assegnato a un processo è 0x00007FFFFFFFF000. Determinare l'indirizzo fisico se questo indirizzo virtuale è mappato al frame fisico 1024. Mostrare tutti i passaggi, incluso il calcolo del numero di pagina e dell'offset.

### Dati del problema
- **Architettura**: 64 bit
- **Spazio di indirizzamento virtuale**: 2^48 Byte
- **Dimensione pagina**: 4KB (4096 Byte)
- **Memoria fisica**: 16GB

### Soluzione

#### 1. Numero di bit necessari per il numero di frame fisico
- Memoria fisica totale = 16GB = 16 * 2^30 byte = 2^34 byte
- Dimensione di una pagina/frame = 4KB = 2^12 byte
- Numero totale di frame fisici = Memoria fisica / Dimensione frame = 2^34 / 2^12 = 2^22 frame
- **Bit necessari per rappresentare 2^22 frame = 22 bit**

#### 2. Numero totale di entry nella inverted page table (IPT)
- L'IPT ha una entry per ogni frame fisico
- **Numero totale di entry = Numero di frame fisici = 2^22 = 4.194.304 entry**

#### 3. Calcolo dell'indirizzo fisico
##### Scomposizione dell'indirizzo virtuale:
- Indirizzo virtuale = 0x00007FFFFFFFF000 (48 bit effettivi)
- Dimensione pagina = 4KB = 2^12 byte = 0x1000 byte
- Offset all'interno della pagina = ultimi 12 bit dell'indirizzo = 0x000
- Numero di pagina virtuale = (Indirizzo virtuale >> 12) = (0x00007FFFFFFFF000 >> 12) = 0x00007FFFFFFFFFFF

##### Calcolo dell'indirizzo fisico:
- Frame fisico assegnato = 1024 = 0x400
- Indirizzo di base del frame = Frame fisico * Dimensione pagina = 0x400 * 0x1000 = 0x400000
- Offset all'interno della pagina = Indirizzo virtuale & 0xFFF = 0x000
- **Indirizzo fisico = Indirizzo di base del frame + Offset = 0x400000 + 0x000 = 0x400000**

---

## Esercizio 2: File System con Allocazione Linked

### Problema
Si consideri un file system che utilizza l'allocazione di tipo linked per l'allocazione dei file. Il disco è diviso in blocchi di 4 KB ciascuno. Ogni blocco contiene un puntatore al blocco successivo, che occupa 4 byte del blocco. Un file richiede 5 MB di spazio di archiviazione.

A. Calcolare il numero di blocchi del disco necessari per allocare l'intero file, incluso lo spazio necessario per i puntatori.

B. Se i puntatori sono stati ridotti a 2 byte (presupponendo una capacità del disco inferiore), ricalcolare il numero di blocchi necessari. Questa modifica renderebbe l'archiviazione dei file più efficiente in termini di utilizzo dei blocchi?

C. Confrontare il sovraccarico introdotto dai puntatori in entrambi i casi come percentuale dello spazio di archiviazione totale usato.

### Dati del problema
- **Tipo di allocazione**: Linked allocation
- **Dimensione blocco**: 4 KB (4096 Byte)
- **Dimensione puntatore**: 4 Byte (caso A) / 2 Byte (caso B)
- **Dimensione file**: 5 MB = 5 * 1024 KB = 5120 KB = 5.242.880 Byte

### Soluzione

#### A. Calcolo del numero di blocchi con puntatori da 4 Byte
- Spazio utile per blocco = 4096 - 4 = 4092 Byte
- Dimensione file in Byte = 5 * 1024 * 1024 = 5.242.880 Byte
- Numero di blocchi necessari = ⌈5.242.880 / 4092⌉ = ⌈1.281,25⌉ = 1.282 blocchi

#### B. Calcolo del numero di blocchi con puntatori da 2 Byte
- Spazio utile per blocco = 4096 - 2 = 4094 Byte
- Dimensione file in Byte = 5.242.880 Byte
- Numero di blocchi necessari = ⌈5.242.880 / 4094⌉ = ⌈1.280,63⌉ = 1.281 blocchi

La modifica rende l'archiviazione leggermente più efficiente poiché richiede un blocco in meno. La differenza è minima in questo caso (risparmio di un solo blocco, circa lo 0,08%), ma potrebbe essere più significativa per file più grandi o in sistemi con molti file.

#### C. Confronto del sovraccarico
- **Caso A (puntatori 4 Byte)**: 
  - Spazio totale occupato = 1.282 * 4096 = 5.251.072 Byte
  - Spazio occupato dai puntatori = 1.282 * 4 = 5.128 Byte
  - Percentuale di sovraccarico = (5.128 / 5.251.072) * 100 ≈ 0,098%

- **Caso B (puntatori 2 Byte)**:
  - Spazio totale occupato = 1.281 * 4096 = 5.247.936 Byte
  - Spazio occupato dai puntatori = 1.281 * 2 = 2.562 Byte
  - Percentuale di sovraccarico = (2.562 / 5.247.936) * 100 ≈ 0,049%

La riduzione della dimensione dei puntatori dimezza il sovraccarico percentuale (da circa 0,098% a 0,049%), ma in entrambi i casi il sovraccarico è molto piccolo (meno dello 0,1% dello spazio totale). Questo è dovuto al fatto che la dimensione del file è relativamente grande rispetto alla dimensione dei puntatori.

---

## Esercizio 3: Paginazione, DMA e Interrupt

### Problema
In questo esercizio si esplorano concetti relativi alla paginazione, al Direct Memory Access (DMA) e alla gestione degli interrupt nei sistemi operativi.

### Domanda 1: Page Fault durante DMA
In un sistema con memoria virtuale basata su paginazione a richiesta e DMA, è possibile che si verifichi un page fault durante un trasferimento DMA, causando l'esito negativo del trasferimento se non vengono prese le dovute precauzioni?

- [X] YES
- [ ] NO
- **Motivazione**: Sì, è possibile. Il controller DMA accede direttamente alla memoria fisica e non può gestire page fault. Se le pagine coinvolte nel trasferimento vengono swappate su disco durante l'operazione DMA, il trasferimento fallirà poiché il controller non ha meccanismi per richiamare le pagine. Per evitare questo problema, i sistemi operativi utilizzano il "memory pinning" che blocca le pagine in memoria durante le operazioni DMA.

### Domanda 2: Operazioni I/O in parallelo
In un sistema con più dispositivi di I/O e una singola CPU, le operazioni di I/O possono essere eseguite in parallelo se la CPU è occupata nell'esecuzione di un processo?

- [X] YES
- [ ] NO
- **Motivazione**: Sì, grazie al DMA (Direct Memory Access). Una volta che la CPU ha configurato il controller DMA specificando indirizzi e quantità di dati da trasferire, il controller opera indipendentemente mentre la CPU può dedicarsi ad altre attività. Al termine dell'operazione, il DMA notifica la CPU tramite interrupt. Questo consente a più dispositivi I/O di operare simultaneamente anche con la CPU impegnata in altri processi.

### Domanda 3: Perdita di Interrupt
In un sistema che utilizza I/O basato su interrupt, è possibile che un interrupt venga perso se il controller di interrupt è occupato nell'elaborazione di un altro interrupt e il dispositivo che ha attivato il secondo interrupt non supporta l'accomodamento degli interrupt?

- [X] YES
- [ ] NO
- **Motivazione**: Sì, se il controller di interrupt non possiede un buffer di accodamento e sta già elaborando un interrupt, un nuovo segnale può andare perso. Questo accade quando il dispositivo che genera il secondo interrupt non può mantenere attivo il segnale (interrupt latching) fino al suo riconoscimento. I sistemi moderni mitigano questo problema con controller dotati di buffer, prioritizzazione degli interrupt e meccanismi di ritrasmissione degli interrupt non riconosciuti.

---

## Esercizio 4: Hard Disk Drive

### Problema
Prendere in considerazione un'unità disco rigido (HDD) con le seguenti specifiche:
- Dimensione del settore: 512 byte
- Numero di tracce per faccia: 5.000
- Numero di settori per traccia: 300
- Numero di piatti a doppia faccia: 6
- Velocità di rotazione del piatto: 7.200 giri/min (giri al minuto)

### Domande
1. Calcolare la velocità di trasferimento dati massima possibile in megabyte al secondo (MB/s), supponendo che sia possibile trasferire una traccia di dati per giro.
2. Se il disco subisce un arresto anomalo della testina su un piatto, in che modo ciò influisce sulla capacità totale e sulla disponibilità dei dati supponendo che non siano in atto meccanismi RAID o di backup?
3. Se il disco ha un tempo di ricerca medio di 4 ms e deve leggere un file da 1 GB suddiviso in 200 tracce non contigue, calcolare il tempo totale necessario per leggere il file. Includi il tempo per la ricerca, la latenza rotazionale e il trasferimento dei dati. Si supponga che la latenza rotazionale media sia di 4,165 ms e che sia possibile leggere una traccia per giro.

### Soluzione

#### 1. Velocità di trasferimento dati massima
- Dati per traccia = Numero settori per traccia × Dimensione settore = 300 × 512 byte = 153.600 byte ≈ 0,147 MB
- Giri al secondo = 7.200 giri/min ÷ 60 = 120 giri/s
- Velocità di trasferimento massima = 120 giri/s × 0,147 MB = 17,64 MB/s

#### 2. Effetti di un arresto anomalo della testina
In caso di arresto anomalo (head crash) su un piatto:
- Capacità persa: Due facce vengono danneggiate (entrambe le facce del piatto)
- Capacità per faccia = 5.000 tracce × 300 settori × 512 byte = 768.000.000 byte ≈ 732,42 MB
- Capacità totale originale = 12 facce × 732,42 MB = 8.789,04 MB
- Capacità dopo il danno = 10 facce × 732,42 MB = 7.324,20 MB
- Riduzione della capacità = 1.464,84 MB (circa 16,67%)
- Disponibilità dei dati: I file con blocchi memorizzati sul piatto danneggiato risulteranno inaccessibili o corrotti, con probabili problemi di integrità del file system.

#### 3. Tempo totale di lettura del file
Per leggere un file da 1 GB suddiviso in 200 tracce non contigue:
- Dimensione media per traccia del file = 1 GB ÷ 200 = 5 MB per traccia
- Dati per traccia fisica del disco = 300 settori × 512 byte = 153.600 byte ≈ 0,147 MB
- Numero di tracce fisiche necessarie per ogni traccia logica del file = 5 MB ÷ 0,147 MB ≈ 34 tracce fisiche
- Pertanto, ogni "traccia" menzionata nel problema rappresenta una porzione logica del file, non una traccia fisica del disco

Per ogni traccia logica del file (porzione di 5 MB):
- Tempo di ricerca (per posizionarsi sulla prima traccia fisica) = 4 ms
- Lettura di 34 tracce fisiche consecutive, ciascuna con:
  - Latenza rotazionale media (solo per la prima traccia) = 4,165 ms
  - Tempo di trasferimento per traccia fisica = 1 ÷ 120 s = 8,33 ms
  
Tempo per leggere una traccia logica = 4 + 4,165 + (34 × 8,33) = 8,165 + 283,22 = 291,385 ms

Tempo totale per leggere 200 tracce logiche non contigue = 200 × 291,385 ms = 58.277 ms ≈ 58,28 secondi