# LAB 1: Introduzione a OS161

---

## Obiettivi Generali del Laboratorio

1.  **Introduzione all'ambiente OS161:** Familiarizzare con il sistema operativo didattico OS/161, la sua architettura (basata su Unix, eseguita su una VM MIPS) e gli strumenti forniti (toolchain di compilazione, ambiente di esecuzione, debugger).
2.  **Navigazione dei File:** Imparare a navigare la struttura delle directory del codice sorgente di OS/161, comprendendo lo scopo delle cartelle principali come `kern`, `conf`, `compile`, `include`, `main`, e la directory `root`.
3.  **Lavorare con il Kernel:** Capire il processo base per modificare il kernel: editare i file sorgente, configurare le opzioni di compilazione, compilare (buildare) il kernel e installarlo.
4.  **Eseguire e Debuggare OS161:** Imparare come avviare il kernel compilato nel simulatore System/161 e come utilizzare il debugger GDB per analizzare l'esecuzione del kernel.

---

## Parte 1: Introduzione e Navigazione

*(Basato sulle slide 3, 4, 5, 6)*

### Cos'è OS161?

*   OS/161 è un sistema operativo semplificato, ispirato a Unix, progettato specificamente per scopi didattici.
*   Gira su un **simulatore** chiamato **System/161**, che emula hardware **MIPS**.
*   La versione utilizzata (basata sulla 2.x) supporta il **multicore**.
*   L'ambiente fornito include:
    *   **Toolchain MIPS:** Compilatori specifici (`mips-harvard-os161-gcc`) per creare codice eseguibile sull'emulatore MIPS.
    *   **System/161:** L'emulatore MIPS che esegue il nostro kernel OS/161.
    *   **Codice Sorgente OS/161:** Il codice del sistema operativo da modificare.
    *   **Debugger:** Strumenti come GDB (`mips-harvard-os161-gdb`) per l'analisi degli errori.

### Struttura delle Directory Principali

Assumendo l'installazione in `$HOME/os161`:

```
$HOME/os161/
├── os161-base-2.x/      # Codice sorgente principale di OS161
│   ├── kern/            # ---> Codice specifico del KERNEL <---
│   │   ├── main/        # Codice di avvio (main.c), menu
│   │   ├── include/     # Header file (.h) specifici del kernel
│   │   ├── conf/        # ---> File di CONFIGURAZIONE del kernel <---
│   │   ├── compile/     # ---> Directory per le COMPILAZIONI <---
│   │   ├── lib/         # Librerie del kernel (kprintf, ...)
│   │   ├── arch/        # Codice dipendente dall'architettura (MIPS)
│   │   ├── vm/          # Gestione memoria virtuale
│   │   ├── fs/          # File system
│   │   ├── dev/         # Driver dei dispositivi
│   │   ├── thread/      # Gestione dei thread
│   │   ├── proc/        # Gestione dei processi
│   │   └── ...          # Altre sottodirectory
│   ├── include/         # Header (.h) comuni (kernel/user)
│   ├── lib/             # Librerie comuni (libc)
│   ├── bin/             # Comandi utente (shell, ...)
│   ├── sbin/            # Comandi di sistema
│   ├── testbin/         # Programmi di test
│   └── ...              # Altro (man pages, ...)
│
├── root/                # ---> Directory di LAVORO (esecuzione/debug) <---
│                        # Contiene kernel installati (kernel-NOME_CONF)
│                        # Contiene sys161.conf, LHD*.img
│
└── tools/               # Toolchain di compilazione MIPS
```

### Spiegazione delle Directory Chiave (Slide 6)

*   **`$HOME/os161/root`**:
    *   Directory da cui si lanciano `sys161` e `mips-harvard-os161-gdb`.
    *   Contiene il kernel compilato e installato (es. `kernel-HELLO_CONF`).
    *   Contiene il file di configurazione del simulatore (`sys161.conf`) e le immagini disco (`LHD*.img`).
*   **`$HOME/os161/os161-base-2.x/kern`**:
    *   Contiene il codice sorgente del kernel che modificheremo.
*   **`$HOME/os161/os161-base-2.x/kern/conf`**:
    *   Contiene i file per **configurare** una build del kernel.
    *   `conf.kern` (o simile): Elenca *tutti* i file sorgente e le opzioni (`defoption`, `optfile`).
    *   File specifici (es. `DUMBVM`, `ASST1`, `HELLO_CONF`): Definiscono *quali* opzioni e file includere in una specifica build.
    *   Qui si usa il comando `./config` per generare una directory di compilazione.
*   **`$HOME/os161/os161-base-2.x/kern/compile`**:
    *   Contiene le sottodirectory di build (es. `compile/HELLO_CONF`), una per ogni configurazione generata da `./config`.
    *   È *dentro* queste sottodirectory che si eseguono i comandi `bmake`.

---

## Parte 2: Lavorare con il Kernel (Slide 7, 8)

Il ciclo di sviluppo tipico in OS/161:

1.  **Modificare File Sorgente:** Editare file `.c` e `.h` per aggiungere funzionalità, usando `#if OPT_...` per la compilazione condizionale.
2.  **Configurare e Buildare:** Definire/abilitare opzioni nel file di configurazione, usare `./config`, `bmake depend`, `bmake`, `bmake install`.
3.  **Eseguire e Verificare:** Lanciare il kernel compilato con `sys161` dalla directory `root` e testare il comportamento.
4.  **Debuggare:** Usare `sys161 -w` e `mips-harvard-os161-gdb` per trovare e correggere errori.

---

## Parte 3: Task Pratico - Modificare il Kernel (Esercizio "Hello World")

*(Implementa l'esercizio della slide 9)*

**Obiettivo:** Aggiungere una funzione `hello` opzionale che stampi un messaggio all'avvio del kernel.

**Ragionamento Passo-Passo e Implementazione:**

1.  **Creare la Funzione (`hello.c`)**
    *   **Logica:** Creare un file separato per la nuova funzionalità (`hello.c`) per modularità.
    *   **Azione:** Creare il file `kern/main/hello.c`.
    *   **Codice (`kern/main/hello.c`):**
        ```c
        #include <types.h> // Tipi base OS/161
        #include <lib.h>   // Librerie kernel (kprintf)
        #include "hello.h" // Prototipo (dall'header che creeremo)

        /*
         * Stampa un messaggio sulla console.
         * Se msg è NULL, stampa un messaggio di default.
         */
        void hello(char *msg) {
            if (msg != NULL) {
                // Messaggio fornito
                kprintf("%s\n", msg);
            } else {
                // Messaggio di default
                kprintf("Hello OS/161\n");
            }
        }
        ```
    *   **Spiegazione:** Include le librerie necessarie, definisce `hello(char *msg)`, controlla se `msg` è valido e usa `kprintf` per stampare il messaggio appropriato.

2.  **Creare l'Header File (`hello.h`)**
    *   **Logica:** Dichiarare la funzione in un header (`.h`) per renderla visibile ad altri file e per la compilazione condizionale tramite opzioni. Posizionarlo in `kern/main` come da soluzione fornita.
    *   **Azione:** Creare il file `kern/main/hello.h`.
    *   **Codice (`kern/main/hello.h`):**
        ```c
        #ifndef _HELLO_H_  // Header Guard Start
        #define _HELLO_H_

        /* Questo file è generato automaticamente da 'config'.
         * Contiene #define OPT_HELLO 1 se l'opzione 'hello' è abilitata. */
        #include "opt-hello.h"

        #if OPT_HELLO // Compila il blocco solo se OPT_HELLO è definito e non zero

        /* Prototipo della funzione hello */
        void hello(char *msg);

        #endif // Fine blocco #if OPT_HELLO

        #endif /* _HELLO_H_ */ // Header Guard End
        ```
    *   **Spiegazione:**
        *   **Header Guard:** (`#ifndef`/`#define`/`#endif`) previene inclusioni multiple.
        *   **`#include "opt-hello.h"`:** Include il file generato da `config`. **NON CREARE QUESTO FILE MANUALMENTE.** Viene creato in base all'opzione `hello` nel file di configurazione.
        *   **`#if OPT_HELLO ... #endif`:** Rende la dichiarazione del prototipo `void hello(char *msg);` condizionale. Se `OPT_HELLO` non è definito (opzione `hello` non abilitata), il prototipo viene omesso.

3.  **Dichiarare l'Opzione di Configurazione (`conf.kern`)**
    *   **Logica:** Informare il sistema di build dell'esistenza della nuova opzione `hello`.
    *   **Azione:** Modificare `kern/conf/conf.kern`. Aggiungere:
        ```makefile
        # Dichiara l'opzione 'hello'
        # Il testo virgolettato è una descrizione opzionale
        defoption hello         "Enable simple hello world message"
        ```
    *   **Spiegazione:** `defoption` registra l'opzione `hello`. Questo nome verrà usato nei file di configurazione specifici (es. `HELLO_CONF`) e per generare `opt-hello.h`.

4.  **Collegare il File Sorgente all'Opzione (`conf.kern`)**
    *   **Logica:** Dire al sistema di build di compilare `hello.c` *solo se* l'opzione `hello` è abilitata.
    *   **Azione:** Modificare `kern/conf/conf.kern`. Aggiungere (tipicamente vicino ad altri file `main/`):
        ```makefile
        # Associa il file main/hello.c all'opzione 'hello'
        # Verrà compilato solo se 'option hello' è presente nella configurazione
        optfile main/hello.c    hello
        ```
    *   **Spiegazione:** `optfile` lega il sorgente `main/hello.c` (percorso relativo a `kern/`) all'opzione `hello`.

5.  **Chiamare la Funzione da `kmain` (`main.c`)**
    *   **Logica:** Chiamare `hello()` durante l'avvio del kernel, dopo `boot()` (così `kprintf` funziona) ma prima di `menu()`. Rendere la chiamata condizionale usando `OPT_HELLO`.
    *   **Azione:** Modificare `kern/main/main.c`.
        *   Aggiungere gli include necessari all'inizio del file:
            ```c
            #include "opt-hello.h" // Per OPT_HELLO
            #include "hello.h"     // Per il prototipo di hello()
            ```
        *   Trovare la funzione `kmain` e aggiungere la chiamata condizionale:
            ```c
            void
            kmain(char *arguments)
            {
                boot(); // Bootstrap iniziale

                /* ----- MODIFICA HELLO ----- */
                #if OPT_HELLO // Chiama hello() solo se l'opzione è abilitata
                hello((char *) "Hello to PdS Students"); // Passa un messaggio custom
                #endif
                /* --- FINE MODIFICA HELLO --- */

                menu(arguments); // Avvia il menu del kernel

                /* Non dovrebbe arrivare qui */
            }
            ```
    *   **Spiegazione:** Gli `#include` forniscono `OPT_HELLO` e il prototipo. `#if OPT_HELLO` assicura che la chiamata `hello(...)` sia compilata solo quando l'opzione è attiva, prevenendo errori di compilazione in caso contrario.

---

## Parte 4: Buildare (Compilare) il Kernel

*(Segue la procedura della slide 10)*

**Obiettivo:** Compilare il kernel includendo la funzione `hello`.

**Passo-Passo:**

1.  **Creare/Modificare un File di Configurazione**
    *   **Logica:** Creare una configurazione specifica (`HELLO_CONF`) che abiliti l'opzione `hello`.
    *   **Azione:**
        ```bash
        # Vai nella directory delle configurazioni
        cd $HOME/os161/os161-base-2.x/kern/conf

        # Copia una configurazione base (es. DUMBVM)
        cp DUMBVM HELLO_CONF

        # Apri HELLO_CONF con un editor e aggiungi la riga:
        # option hello
        # Salva e chiudi.
        ```

2.  **Generare la Directory di Build**
    *   **Logica:** Usare `./config` per leggere `HELLO_CONF`, validare, creare `compile/HELLO_CONF`, e generare `opt-hello.h` con `#define OPT_HELLO 1`.
    *   **Azione (dalla directory `kern/conf`):**
        ```bash
        ./config HELLO_CONF
        ```
        *Output atteso: Nessun errore, messaggio sulla creazione di `../compile/HELLO_CONF`.*

3.  **Compilare il Kernel**
    *   **Logica:** Spostarsi nella directory di build e usare `bmake` per compilare e installare.
    *   **Azione:**
        ```bash
        # Vai nella directory di build appena creata
        cd ../compile/HELLO_CONF
        # Ora sei in $HOME/os161/os161-base-2.x/kern/compile/HELLO_CONF

        # Analizza le dipendenze
        bmake depend

        # Compila il codice sorgente -> crea 'kernel'
        bmake

        # Installa il kernel in $HOME/os161/root/kernel-HELLO_CONF
        bmake install
        ```
    *   **Output Atteso:** Nessun errore di compilazione. Un file `kernel-HELLO_CONF` viene creato in `$HOME/os161/root`.

---

## Parte 5: Eseguire il Kernel

*(Segue la procedura della slide 11)*

**Obiettivo:** Avviare il kernel `kernel-HELLO_CONF` e vedere il messaggio "Hello".

**Passo-Passo:**

1.  **Navigare alla Directory `root`**
    *   **Azione:**
        ```bash
        cd $HOME/os161/root
        ```

2.  **Eseguire il Kernel nel Simulatore**
    *   **Logica:** Usare `sys161` per lanciare il simulatore specificando il nostro kernel compilato.
    *   **Azione:**
        ```bash
        sys161 kernel-HELLO_CONF
        ```

**Output Atteso:**

Dovresti vedere l'output di boot di OS/161, inclusa la riga aggiunta:

```
... (messaggi di boot iniziali) ...
OS/161 base system version ...
Copyright (c) 2000, ...
Put-your-group-name-here's system version ... (HELLO_CONF #...)

Hello to PdS Students  <--- MESSAGGIO ATTESO!

Device probe...
emu0: Hard disk ...
cpu0: MIPS r2000/r3000 core ...
... (altri messaggi di boot) ...

OS/161 kernel [? for menu]:
```

Se il messaggio non appare, ricontrolla tutti i passaggi della Parte 3 e 4.

---

## Parte 6: Debuggare il Kernel

*(Segue la procedura delle slide 12, 13)*

**Obiettivo:** Usare GDB per controllare l'esecuzione del kernel e ispezionare la funzione `hello`.

**Passo-Passo (Usando GDB TUI):**

1.  **Preparare Due Terminali**
    *   **Azione:** Apri **due** terminali. In **entrambi**, vai a `$HOME/os161/root`:
        ```bash
        # Terminale 1
        cd $HOME/os161/root

        # Terminale 2
        cd $HOME/os161/root
        ```

2.  **Avviare il Kernel in Modalità Debug (Terminale 1)**
    *   **Logica:** Avviare `sys161` con l'opzione `-w` (wait) per attendere il debugger.
    *   **Azione (Terminale 1):**
        ```bash
        sys161 -w kernel-HELLO_CONF
        ```
    *   **Output Atteso (Terminale 1):** `sys161: Waiting for debugger connection on port 16161...`

3.  **Avviare il Debugger (Terminale 2)**
    *   **Logica:** Lanciare GDB (`mips-harvard-os161-gdb`), caricando i simboli dal file kernel e attivando l'interfaccia TUI.
    *   **Azione (Terminale 2):**
        ```bash
        mips-harvard-os161-gdb -tui kernel-HELLO_CONF
        ```
    *   **Output Atteso (Terminale 2):** Avvio di GDB con interfaccia TUI e prompt `(gdb)`.

4.  **Connettere GDB al Simulatore (Terminale 2)**
    *   **Logica:** Dire a GDB di connettersi al processo `sys161` in ascolto sulla porta 16161.
    *   **Azione (Terminale 2, prompt `(gdb)`):**
        ```gdb
        target remote :16161
        ```
    *   **Output Atteso:** Messaggio di connessione riuscita in entrambi i terminali. GDB si ferma alla prima istruzione del kernel.

5.  **Usare il Debugger (Terminale 2)**
    *   **Logica:** Impostare un breakpoint in `hello` e continuare l'esecuzione fino a quel punto.
    *   **Azione (Terminale 2, prompt `(gdb)`):**
        ```gdb
        # Imposta breakpoint alla funzione 'hello'
        b hello

        # Continua l'esecuzione fino al breakpoint
        c
        ```
    *   **Output Atteso:** L'esecuzione nel Terminale 1 avanza, poi si ferma. Nel Terminale 2, GDB si ferma all'inizio della funzione `hello`, mostrando il codice sorgente nella finestra TUI.

6.  **Esaminare e Continuare (Terminale 2)**
    *   **Azione (Terminale 2, prompt `(gdb)`):**
        ```gdb
        # Stampa il valore dell'argomento 'msg'
        p msg

        # Esegui riga per riga (senza entrare in kprintf)
        n
        n

        # Mostra lo stack delle chiamate (chi ha chiamato hello?)
        bt

        # Continua l'esecuzione normale
        c
        ```

7.  **Uscire**
    *   **Azione (Terminale 2):** Al prompt `(gdb)`, digita `q` e conferma con `y`.
    *   **Azione (Terminale 1):** `sys161` dovrebbe terminare. Se rimane in attesa al prompt del kernel, digita `q`.