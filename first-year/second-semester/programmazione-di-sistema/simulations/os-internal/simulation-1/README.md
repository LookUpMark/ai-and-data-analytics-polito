# PDS OS internals 13/01/2025

## Domanda 1

Si considerino le affermazioni che seguono, a proposito di possibili vantaggi e svantaggi di una inverted page table (IPT), rispetto a una tabella delle pagine standard (eventualmente gerarchica) PT. Si dica di ognuno se sia vera o falsa, motivando la risposta.

#### Vantaggi: L’IPT permette di risparmiare di memoria:  
1. Si risparmia sempre memoria.
```
No, in quanto il risparmio di memoria dipende dalla quantità di processi allocati. Per pochi processi, il risparmio è più o meno nullo, mentre va ad aumentare con l'aumentare stesso del numero di processi.
```
 
2. Dipende dalle dimensioni della RAM, dal numero di processi e dal loro spazio di indirizzamento virtuale.
```
Si, in quanto a grandezza della IPT è proporzionale alla dimensione della RAM e ogni processo avrebbe una page table propria, per cui utilizzando un'unica IPT si avrebbe un risparmio avendo di fatto meno overhead.
```
 
3. Si risparmia sempre memoria quando lo spazio di indirizzamento di un  processo è maggiore della dimensione della RAM.
```
No, sempre per lo stesso motivo della risposta 1. Per molti processi si avrà comunque un risparmio in memoria in quanto nella IPT ogni entry corrisponde unicamente ad un frame fisico effettivamente allocato.
```
 
4. Si può risparmiare anche in casi in cui so spazio di indirizzamento di ogni processo è inferiore alla dimensione della RAM.
```
Si, sempre per lo stesso motivo. Non vi è alcun overhead dovuto all'avere una page table separata per ogni processo, la IPT è comune a tutti i processi.
```

#### Svantaggi: L’IPT è lenta, perché non garantisce accesso diretto ma occorre una ricerca:
1. La chiave di ricerca è la coppia (pid,frame).
```
No, la chiave di ricerca nella IPT è la coppia (pid, p) dove p è il numero di pagina.
```
 
2. La chiave di ricerca è la coppia (pid,pagina).
```
Si, risposta precedente.
```
 
3. Per migliorare le prestazioni si sostituisce la IPT con una tabella di HASH.
```
No, una tabella HASH è un metodo alternativo alla IPT, non una sostituzione.
```
 
4. Per migliorare le prestazioni si aggiunge alla IPT una tabella di HASH.
```
Si, l'utilizzo in coppia di una IPT con una tabella HASH permette di avere tutti i vantaggi in termini di memoria della IPT con le performance di ricerca della tabella HASH.
```

---

Sia dato un processo avente spazio di indirizzamento virtuale di 48 GB, in un sistema dotato di 16GB di RAM, con architettura a 64 bit (in cui si indirizza il Byte) e gestione della memoria paginata (pagine/frame da 4KB). Si supponga che 4GB di RAM sia allocato in modo statico al kernel. Si vogliono confrontare una soluzione basata su tabella delle pagine standard (una tabella per ogni processo) e una basata su IPT. Si calcolino:

#### A. Le dimensioni della tabella delle pagine (a un solo livello) per il processo e della IPT. Si ipotizzi che il pid di un processo possa essere rappresentato su 12 bit. Si utilizzino 28 bit per gli indici di pagina e/o di frame (nella PT o nella IPT) e si tenga conto che, per allineamento, una cella di IPT o PT può solo essere di 32 o 64 bit.
```
dim_pagine_frame = 4KB
spazio_indirizzamento_virtuale = 48GB
RAM = 16GB - 4GB = 12GB liberi

num_pagine_virtuali = 48GB / 4KB = 12M
num_frame_fisici = 12GB / 4KB = 3M

# Page Table

Ogni entry della tabella deve avere un offset:
12M -> 2^24 -> 24 bit di offset

Inoltre, deve avere 4 bit aggiuntivi:
1 bit di validity, 1 bit di reference, 1 bit di modify, 1 bit di protection

I restanti bit sono dovuti all'allineamento:
24 + 4 + 4 = 32 bit = 4B

La dimensione della page table sarà quindi:
dim_page_table = 12M * 4B = 48MB

# IPT

Ogni entry della IPT è identificata dalla chiave (pid, p) dove p è la pagina.

Sappiamo già che il numero di pagine può essere 12M, per cui 24 bit verranno utilizzati per esso.

Abbiamo sempre i 4 bit aggiuntivi.

I restanti bit sono quelli del PID, che necessita di almeno 12 bit. Per l'allineamento scegliamo quindi 64 bit (8B).

La dimensione della IPT sarà quindi:
dim_ipt = 8B * 3M = 24MB
```

#### B. Si dica infine, utilizzando la IPT proposta (12 bit di pid, 28 bit per un indice di pagina/frame), quale è la 
dimensione massima possibile per lo spazio di indirizzamento virtuale di un processo.
```
Dal momento che utilizziamo 28 bit per rappresentare un indice di pagina, è facile dedurre che il numero massimo di pagine virtuali disponibili è pari a 2^28 = 256M.

Considerando la dimensione di ciascuna pagina, otteniamo:
max_dim = 256M * 4KB = 1TB
```

---

## Domanda 2

Sia dato un disco organizzato con blocchi fisici e logici di dimensione 8KB. Il disco contiene più partizioni: 
la partizione A, di NB blocchi, è formattata per un file system che alloca staticamente NM blocchi per i 
metadati (che includono directory, file control blocks e una bitmap per la gestione dello spazio libero) e ND 
blocchi per i dati dei file. La bitmap ha un bit per ciascuno degli ND blocchi di dati. NM/4 blocchi di metadati sono riservati alla bitmap. 
 
Si risponda alle seguenti domande: 
  
#### A. Si calcoli il rapporto ND/NM.
```
dim_blocchi = 8KB
partizione_a -> NB blocchi = NM blocchi (metadati) + ND blocchi (dati)
bitmap ha ND bit, uno per ciascun blocco di dati, mentre NM/4 blocchi di metadati sono riservati alla stessa

La bitmap occuperà:
dim_bitmap = (NM/4) * 8KB = 2KB * NM che deve essere pari a ND in quanto appunto la bitmap possiede ND bit

Abbiamo quindi che:
ND = (2KB*8) * NM -> ND/NM = 16Kbits
```

#### B. Supponendo che la bitmap indichi un rapporto blocchi liberi / usati del 33,33% (quindi 1 blocco libero 
ogni 3 usati), si calcoli (in funzione di NM) la dimensione massima per un intervallo contiguo di blocchi 
liberi, assumendo la configurazione più favorevole della bitmap (favorevole significa in grado di ottenere 
partizioni libere più grandi). Si dia la stessa risposta anche assumendo la configurazione della bitmap 
meno favorevole.
```
1 blocco libero ogni 3 usati = 1/4 = 0.25

La configurazione più favorevole si avrebbe nel caso in cui tutti i blocchi liberi fossero contigui di default, per cui il massimo intervallo ottenibile sarebbe:
dim_bitmap * 0.25 = 0.25 * 2KB * 8 * NM = 4K * NM bits

La configurazione peggiore si avrebbe nel caso in cui nessun blocco libero ne avesse un altro adiacente, per cui l'intervallo massimo sarebbe pari a 1.
```

#### C. Si supponga che un file control block (FCB) abbia dimensione 256B e NM/4 blocchi di metadati siano 
riservati agli FCB, per un massimo di 16K file. Si calcolino ND, NM e NB. Si esprima anche la 
dimensione della bitmap e della partizione A, espressa in Byte.
```
fcb = 256B
spazio_fcb = NM/4
max_file = 16K

Sappiamo che un blocco ha una dimensione di 8KB. All'interno di un blocco posso contenere:
fcb_per_blocco = 8KB / 0.25KB = 32 fcb

So che al massimo posso avere 16K file, e so anche che per posso avere un massimo di NM/4 fcb, per cui:
16K = fcb_per_blocco * spazio_fcb = 32 * NM/4 -> NM = 2K

Posso adesso calcolare ND:
ND = 16K * NM = 16K * 2K = 32M

Infine trovo NB come somma:
NB = NM + ND = 32M + 2K

La bitmap avrà quindi dimensione pari a:
dim_bitmap = 2K * NM = 2K * 2K = 4MB

La partizione A invece:
dim_a = (32M + 2K) * 8KB = 256GB + 16MB
```

---

## Domanda 3
Si risponda alle seguenti domande sulla gestione della memoria: 
  
#### A. Si consideri il caricamento dinamico (dynamic loading) e il link dinamico (dynamic linking). È possibile caricare dinamicamente un programma senza cha sia necessario il dynamic linking? Il dynamic linking richiede che un programma sia anche caricabile dinamicamente (dynamic loading)?
```
Dynamic loading e dynamic linking sono due meccanismi indipendenti: da un lato, il dynamic loading consiste nella possibilità di caricare le parti necessarie al funzionamento del programma solo quando necessario; il dynamic linking riguarda più parti esterne al programma, come ad esempio librerie. Entrambi i meccanismi possono essere utilizzati l'uno indipendentemente dall'altro.
```

#### B. Si spieghi brevemente perché un’Inverted Page Table necessiti di una tabella di HASH e si spieghi perchè la soluzione di IPT + tabella di HASH e’ diversa da una soluzione con PT basata solamente su tabella di Hash?
```
Una IPT combinata con una tabella HASH è in grado di ottenere i vantaggi in termini di risparmio di memoria della IPT con la velocità nella ricerca della tabella HASH. La soluzione IPT + HASH si differenzia da una semplice tabella HASH perché implementa due livelli di indirezione: la HASH table fornisce l'indice dell'entry nella IPT, che a sua volta contiene il frame fisico e i metadati associati. In una soluzione con sola HASH table, invece, la tabella mapperebbe direttamente l'indirizzo virtuale al frame fisico in un singolo livello, per cui tutti i dati, compresi gli indirizzi virtuali, dovrebbero essere immagazzinati nella stessa HASH table.
```

#### C. Si consideri una CPU dotata di TLB: la TLB può contenere entry di più processi simultaneamente o è vincolata a contenere entry per un solo processo? Il “valid” bit presente in un entry della TLB e’ una semplice copia del “valid” bit presente nella Page Table (motivare)?
```
Esistono delle TLB create appositamente per contenere entry di più processi simultaneamente (TLB tagged con ASID (Address Space Identifier)), così come esistono TLB che possono gestire unicamente entry di un processo alla volta. Nel caso di quest'ultima è necessario resettare la TLB ad ogni context switch. No, il valid bit nella TLB ha un significato differente da quello nella PT: quello nella TLB sta a indicare unicamente se l'entry è libero, a differenza di quello nella PT che sta a indicare se all'entry è associato un frame fisico o questo deve essere caricato.
```

---

## Domanda 4

É dato un Sistema OS161. Si supponga di aver aggiunto le istruzioni seguenti a kern/conf/conf.kern
```
defoption project 
optfile project syscall/project.c 
```
e di aver creato il file PROJECT in kern/conf, copiato dal file DUMBVM. 
 
#### A. Si dica se le azioni descritte nel seguito, piú l’esecuzione (in kern/conf) di ./config PROJECT, sono sufficienti 
affinché il file opzionale syscall/project.c sia compilato quando si eseguono (in kern/compile/PROJECT) i comandi:
```
bmake depend 
bmake
```
```
No, queste operazioni non sono sufficienti, in quanto in questo modo abbiamo solamente generato la macro OPT-PROJECT, tuttavia questa risulterà inizializzata a 0: questo avviene perchè, copiando il file di configurazione da DUMBVM, all'interno del file non sarà presente la riga di codice "option project", che si occupa per l'appunto di impostare la macro a 1.
```

#### B. Quale file, tra project.h e opt-project.h viene generato automaticamente dal comando ./config PROJECT? 
Il file viene sempre generato, oppure solo se l’istruzione seguente compare nel file PROJECT?
```
options project
```
```
Il file che viene generato automaticamente è opt-project.h, project.h è solo l'header del file .c che deve essere immesso nella cartella main. Il file opt verrà sempre generato se l'opzione "defoption project" verrà inserita in conf.kern , l'unica cosa che cambierà sarà l'esecuzione o meno dell'opzione a seconda del valore della macro (vedi risposta precedente).
```

#### C. Cosa contiene il file (quello generato automaticamente, citato nella domanda precedente)?
```
Il file contiene per l'appunto la macro OPT_PROJECT, inizializzata a 0 o 1 a secondo che l'opzione venga inclusa nel file PROJECT.
```

#### D. Si supponga di inserire in main.c l’istruzione
```
project_init(); 
```
Considerando che la funzione project_init() e’ implementata nel file syscall/project.c, come si puo’ fare in modo che l’istruzione sia considerate e compilate solo nelle versioni del kernel in cui l’opzione project e’ abilitata?
```
Si potrebbe pensare di utilizzare all'interno del main.c la compilazione condizionale, che faccia uso di statements quali #if ed #endif combinati con l'utilizzo della macro precedentemente generata.
```

---

