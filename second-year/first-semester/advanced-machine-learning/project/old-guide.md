
# Strategie Avanzate di Implementazione per la Corrispondenza Semantica con Modelli Fondazionali Visivi: Un Blueprint di Ricerca per Architetture Efficienti e Consapevoli della Geometria

## 1. Introduzione e Paradigma Architetturale

Il panorama della visione artificiale moderna è stato radicalmente trasformato dall'avvento dei Visual Foundation Models (VFM). La corrispondenza semantica, ovvero il compito di stabilire corrispondenze dense a livello di pixel tra oggetti semanticamente simili ma geometricamente diversi, rappresenta oggi uno dei banchi di prova più critici per valutare la robustezza e la capacità di generalizzazione di questi modelli.

Questo documento costituisce un blueprint tecnico esaustivo per l'implementazione di una pipeline di corrispondenza semantica allo stato dell'arte su infrastruttura Kaggle (GPU NVIDIA T4). Il piano di ricerca è strutturato per un team di quattro membri, adottando una **Strategia a Matrice**: ogni membro è responsabile dello sviluppo di un **Modulo Tecnico specifico**, che dovrà essere applicato e validato su **tre backbone distinti: DINOv2, DINOv3 e SAM (Segment Anything Model)**.

### 1.1 Il Gap Semantico e le Proprietà Emergenti

La sfida centrale è colmare il "gap semantico" (es. mappare la zampa di un cane su quella di un lupo) gestendo variazioni di posa e apparenza. I modelli come DINOv2 e DINOv3, addestrati con obiettivi auto-supervisionati (SSL), e SAM, addestrato sulla segmentazione, offrono rappresentazioni interne potenti ma diverse.

-   **DINOv2/v3:** Ottimizzati per feature globali e locali discriminative.
    
-   **SAM:** Ottimizzato per la sensibilità ai confini e alle parti degli oggetti.
    

Questo progetto mira a confrontare quantitativamente questi paradigmi attraverso quattro stadi evolutivi: Baseline Training-free, Light Fine-tuning, Raffinamento Geometrico e Adattamento Parametro-Efficiente (LoRA).

### 1.2 Organizzazione del Lavoro: Moduli Trasversali

A differenza di un approccio sequenziale, il lavoro è parallelizzato per competenza tecnica. Ogni membro sviluppa il proprio modulo e lo esegue su tutti e tre i backbone.

-   **Membro A (Modulo 1):** Infrastruttura, Dataloader e Baseline Training-Free (Frozen).
    
-   **Membro B (Modulo 2):** Light Fine-tuning degli ultimi layer (Supervisione Keypoint).
    
-   **Membro C (Modulo 3):** Raffinamento Geometrico (Window Soft-Argmax differenziabile).
    
-   **Membro D (Modulo 4):** Fine-tuning Parametro-Efficiente (LoRA/QLoRA) e Valutazione Cross-Dataset.
    

----------

## 2. Modulo 1: Infrastruttura di Valutazione e Baseline Training-Free (Membro A)

Questo modulo costituisce le fondamenta scientifiche dell'intero progetto. L'obiettivo primario è costruire una pipeline di dati robusta per il dataset SPair-71k e stabilire le prestazioni "out-of-the-box" (senza addestramento) di **DINOv2, DINOv3 e SAM**.

### 2.1 L'Ecosistema SPair-71k e la Gestione dei Dati

SPair-71k contiene 70.958 coppie di immagini con variazioni estreme di vista e scala.

-   **Strategia Implementativa per Kaggle:** Il Membro A deve implementare una classe `SemanticCorrespondenceDataset` in PyTorch unica, che sarà usata da tutti gli altri membri.
    
-   **Preprocessing Critico:** Le immagini devono essere ridimensionate a una risoluzione fissa (es. $518 \times 518$ per DINOv2/v3, $1024 \times 1024$ per SAM) mantenendo l'aspect ratio tramite padding. È vitale trasformare le coordinate dei keypoint ground truth $p_{gt}$ applicando la stessa trasformazione affine (scale + pad) usata per l'immagine.
    

### 2.2 Baseline: Estrazione Feature Frozen (Zero-Shot)

Per ogni backbone, il Membro A deve implementare l'estrazione delle feature e il calcolo della similarità senza alcun addestramento.

1.  **DINOv2 & DINOv3:** Estrarre le feature map (tipicamente dalle Key o dai Value dell'ultimo layer di attenzione).
    
2.  **SAM:** Utilizzare l'Image Encoder. Poiché l'output di SAM è a bassa risoluzione ($64 \times 64$), è necessario implementare un upsampling bilineare per confrontarlo equamente con i keypoint.
    
3.  **Matching:** Calcolare la similarità del coseno tra il descrittore del keypoint sorgente e tutti i descrittori target, selezionando il massimo (Argmax).
    

### 2.3 Metrica PCK (Percentage of Correct Keypoints)

Il Membro A implementa la metrica ufficiale PCK@$\alpha$. Un keypoint predetto è corretto se:

$$dist(\hat{p}, p_{gt}) < \alpha \cdot \max(H_{bbox}, W_{bbox})$$

Si devono produrre report per $\alpha \in \{0.01, 0.05, 0.1\}$.

### Deliverables del Modulo 1 (Membro A)

1.  **Universal Dataloader:** Codice PyTorch riusabile in `src/data`.
    
2.  **3 Notebook Baseline:** `modulo1_baseline_dinov2`, `modulo1_baseline_dinov3`, `modulo1_baseline_sam`.
    
3.  **Report Baseline:** Tabella comparativa PCK@0.1 dei tre modelli congelati.
    

----------

## 3. Modulo 2: Light Fine-tuning degli Ultimi Layer (Membro B)

Mentre il Modulo 1 valuta i modelli congelati, il Modulo 2 implementa lo step 2 del progetto: adattare i modelli al task specifico sbloccando e addestrando solo gli ultimi layer del backbone.

### 3.1 Strategia di Fine-Tuning Parziale

Il fine-tuning completo è costoso. Il Membro B esplora l'efficacia del "Light Fine-tuning" su tutti e tre i backbone.

-   **Metodologia:** Congelare tutto il backbone eccetto gli ultimi $N$ blocchi Transformer.
    
-   **Ablation Study:** Testare $N \in \{1, 2, 4, 8\}$. L'ipotesi è che sbloccare troppi layer porti a overfitting su SPair-71k (dato il dataset limitato), mentre sbloccarne troppi pochi non permetta l'adattamento geometrico.
    

### 3.2 Supervisione tramite Keypoint

Poiché l'obiettivo è la corrispondenza, il modello non viene addestrato con una loss di classificazione, ma per minimizzare la distanza tra il keypoint predetto e il ground truth.

-   Per rendere questo processo addestrabile, è necessario un meccanismo per retropropagare l'errore dalla coordinata predetta ai pesi del modello. Sebbene il Soft-Argmax completo sia parte del Modulo 3, il Membro B può implementare una versione semplificata o utilizzare una loss sulle feature map (es. massimizzare la similarità nella posizione GT) per guidare il fine-tuning.
    

### Deliverables del Modulo 2 (Membro B)

1.  **Classe Trainer Unificata:** Script di training che accetta il backbone come parametro.
    
2.  **3 Notebook Fine-tuning:** Esecuzione del training su DINOv2, DINOv3, SAM.
    
3.  **Analisi Ablativa:** Grafici che mostrano PCK@0.1 in funzione del numero di layer sbloccati per ciascun backbone.
    

----------

## 4. Modulo 3: Raffinamento Geometrico e Window Soft-Argmax (Membro C)

Il matching standard (Argmax) è discreto e non differenziabile. Il Modulo 3 (Step 3 del progetto) mira a sostituirlo con il **Window Soft-argmax**, migliorando la precisione sub-pixel e la robustezza al rumore per tutti e tre i backbone.

### 4.1 Window Soft-Argmax: Teoria e Implementazione

Il Membro C deve implementare un modulo PyTorch differenziabile che:

1.  **Argmax Grossolano:** Identifica il picco di similarità $k_{peak}$ nella feature map.
    
2.  **Windowing:** Isola una finestra $W$ di raggio $R$ attorno a $k_{peak}$.
    
3.  Soft-Argmax Locale: Calcola il centro di massa (speranza matematica) all'interno della finestra:
    
    $$\hat{p} = \sum_{u,v \in W} \text{Softmax}(\beta \cdot S(u, v)) \cdot [u, v]$$
    

### 4.2 Studio di Ablazione sui Parametri

Il modulo deve determinare la configurazione geometrica ottimale per ogni backbone.

-   **Raggio $R$:** Testare finestre di dimensioni $3\times3$ ($R=1$), $5\times5$ ($R=2$), etc. Una finestra troppo piccola non cattura abbastanza contesto; una troppo grande introduce rumore.
    
-   **Temperatura $\beta$:** Controlla l'acutezza della Softmax.
    

### 4.3 Funzioni di Perdita (Dense Objectives)

Per supportare il fine-tuning avanzato, il Membro C implementa anche le Loss Functions:

-   **Coordinate Regression Loss (L2):** $\|\hat{p} - p_{gt}\|_2$.
    
-   **Dense InfoNCE Loss:** Per penalizzare le similarità alte in posizioni errate (utile per distinguere parti simmetriche come occhio sx/dx).
    

### Deliverables del Modulo 3 (Membro C)

1.  **Matcher Differenziabile:** Modulo `WindowSoftArgmax` in `src/matching`.
    
2.  **3 Notebook Soft-Argmax:** Applicazione del matcher ai modelli (sia frozen che fine-tuned dal Modulo 2).
    
3.  **Report Geometrico:** Tabella che mostra il guadagno di PCK ottenuto sostituendo Argmax con Window Soft-Argmax su ciascun backbone.
    

----------

## 5. Modulo 4: Fine-Tuning Parametro-Efficiente (LoRA/QLoRA) (Membro D)

Questo modulo rappresenta l'estensione obbligatoria e la soluzione ai vincoli hardware. Il Membro D applica **LoRA (Low-Rank Adaptation)** a tutti e tre i backbone per permettere un adattamento profondo senza esaurire la memoria della GPU T4.

### 5.1 LoRA e QLoRA su T4

Invece di sbloccare gli ultimi layer (Modulo 2), LoRA inietta matrici di rango ridotto $A$ e $B$ in tutti i layer di attenzione ($W' = W + BA$).

-   **QLoRA (Quantized LoRA):** Per modelli giganti (es. DINOv2-Giant o SAM-H), il Membro D utilizzerà la quantizzazione 4-bit (NF4) per i pesi congelati, mantenendo gli adapter in 16-bit.
    
-   **Target Modules:** Applicare LoRA alle proiezioni `q_proj`, `v_proj` (e idealmente anche ai layer MLP) di DINOv2, DINOv3 e SAM.
    

### 5.2 Valutazione Cross-Dataset (Generalizzazione)

Come estensione del progetto (Step 4), il Membro D valuterà i modelli addestrati con LoRA su dataset non visti durante il training, per testare la generalizzazione del dominio:

-   **PF-Pascal:** Dataset storico per la corrispondenza semantica.
    
-   **AP-10K:** Dataset di pose animali (se disponibile/applicabile).
    

### Deliverables del Modulo 4 (Membro D)

1.  **Wrapper PEFT:** Configurazione LoRA in `src/training`.
    
2.  **3 Notebook LoRA:** Training e validazione di DINOv2-LoRA, DINOv3-LoRA, SAM-LoRA.
    
3.  **Analisi Efficienza:** Report sull'uso della VRAM e tempi di training rispetto al Light Fine-tuning.
    
4.  **Master Comparison (Coordinamento):** Il Membro D aggrega i risultati di tutti i moduli nel notebook finale `master_comparison.ipynb`.
    

----------

## 6. Integrazione e Flusso di Lavoro

I quattro moduli convergono in una pipeline unificata.

1.  **Setup (Settimana 1):** Membro A rilascia il `SemanticCorrespondenceDataset`.
    
2.  **Esecuzione Parallela (Settimane 2-3):**
    
    -   Membro A esegue le baseline su 3 backbone.
        
    -   Membro B esegue il light fine-tuning su 3 backbone.
        
    -   Membro C sviluppa il soft-argmax e lo testa sui checkpoint di A e B.
        
    -   Membro D prepara l'infrastruttura LoRA e la applica ai 3 backbone.
        
3.  **Consolidamento (Settimana 4):**
    
    -   Tutti i risultati JSON vengono raccolti.
        
    -   Il Membro D esegue il `master_comparison.ipynb`.
        

### Ottimizzazioni Specifiche per Kaggle

-   **Persistenza:** Salvare i checkpoint e i JSON dei risultati esternamente o come output del dataset Kaggle per evitare perdite al timeout della sessione.
    
-   **Codice Condiviso:** Tutto il codice core (dataset, modelli, loss) risiede nella cartella `src/` e viene importato nei 12 notebook. Non duplicare il codice delle classi nei notebook.
    

## 7. Analisi dei Risultati Attesi e "Deep Insights"

### 7.1 DINOv3 vs DINOv2

Ci si aspetta che DINOv3 (con Gram Anchoring) mostri prestazioni baseline (Modulo 1) superiori a DINOv2. Tuttavia, l'adattamento con LoRA (Modulo 4) potrebbe ridurre questo divario, dimostrando che un modello meno recente ma ben adattato può competere con l'SOTA generico.

### 7.2 SAM per la Corrispondenza Semantica

SAM eccelle nei confini, ma potrebbe fallire nella coerenza semantica tra classi diverse (es. gatto vs cane). I moduli 2 e 4 sono critici per iniettare questa semantica in SAM. Se SAM-LoRA raggiunge alte prestazioni, dimostrerà che le feature di segmentazione sono una base valida per la corrispondenza se correttamente adattate.

### 7.3 Impatto del Window Soft-Argmax

L'uso del Window Soft-Argmax (Modulo 3) dovrebbe garantire un boost immediato di PCK (specialmente a soglie severe come $\alpha=0.01$) su tutti i backbone, indipendentemente dal training, evidenziando l'importanza del raffinamento geometrico post-processing.

Questa struttura garantisce che ogni membro del team contribuisca in modo equo e tecnicamente rilevante, coprendo l'intero spettro delle moderne tecniche di Computer Vision (Baseline, Fine-tuning, Geometria, Efficienza).

## 8. Conclusione e Direzioni Future

Questa struttura progettuale garantisce che tutti i 4 membri abbiano contributi tecnici distinti e di alto valore che si integrano in una pipeline degna di pubblicazione. Sfruttando QLoRA, il team aggira le limitazioni hardware dell'ambiente universitario/Kaggle impegnandosi al contempo con l'avanguardia della ricerca sull'AI efficiente. La combinazione di raffinamento geometrico (Soft-argmax) e apprendimento parametro-efficiente rappresenta l'attuale metodologia allo stato dell'arte per adattare i Foundation Models generalizzati a compiti specifici e sensibili allo spazio.

----------

### Riferimenti Chiave per l'Implementazione

-   **SPair-71k & PCK:** Min et al. 9, "Hyperpixel Flow".
    
-   **DINOv2:** Oquab et al..4
    
-   **DINOv3:** Simeoni et al..1
    
-   **DIFT:** Tang et al..6
    
-   **Window Soft-argmax / Geo-Awareness:** Zhang et al..23
    
-   **LoRA/QLoRA:** Hu et al. 1, Dettmers et al..2
    

Questo documento fornisce un piano completo, tecnicamente solido ed equamente distribuito per il successo del progetto universitario.
