# Guida Completa Progetto: Semantic Correspondence con Visual Foundation Models
## Advanced Machine Learning - Politecnico di Torino

**Versione:** 3.0 - Edizione Integrata Completa con Moduli per Membro  
**Data:** Dicembre 2024  
**Docenti:** Claudia Cuttano (TA)  
**Destinatari:** Team di 4 membri  
**Infrastruttura:** Kaggle Notebooks (2x GPU T4, 30 ore/settimana)  
**Durata stimata:** 4 settimane (1 mese di lavoro parallelo)  
**Ultimo aggiornamento:** Dicembre 2024  

---

## Struttura del Repository GitHub

La struttura del repository è organizzata per massimizzare il parallelismo tra i quattro moduli. Ogni modulo ha tre notebook separati (uno per backbone), e ogni membro del team è responsabile di un modulo specifico replicato su tutti e 3 i backbone:

```
semantic-correspondence/
│
├── README.md                          # Documentazione principale
├── requirements.txt                   # Dipendenze Python
├── .gitignore                         # File da escludere da Git
│
├── data/                              # Dataset (NON pushare su GitHub)
│   ├── spair-71k/                     # SPair-71k dataset
│   ├── pf-pascal/                     # PF-Pascal dataset (cross-val)
│   └── ap-10k/                        # AP-10K dataset (cross-val)
│
├── checkpoints/                       # Modelli salvati e checkpoint
│   ├── dinov2/                        # Checkpoint DINOv2
│   │   ├── modulo1_baseline/
│   │   ├── modulo2_finetuning/
│   │   ├── modulo3_soft_argmax/
│   │   └── modulo4_lora/
│   ├── dinov3/                        # Checkpoint DINOv3
│   │   ├── modulo1_baseline/
│   │   ├── modulo2_finetuning/
│   │   ├── modulo3_soft_argmax/
│   │   └── modulo4_lora/
│   └── sam/                           # Checkpoint SAM
│       ├── modulo1_baseline/
│       ├── modulo2_finetuning/
│       ├── modulo3_soft_argmax/
│       └── modulo4_lora/
│
├── results/                           # Risultati e metriche
│   ├── dinov2/                        # JSON, visualizzazioni DINOv2
│   │   ├── modulo1_baseline/
│   │   ├── modulo2_finetuning/
│   │   ├── modulo3_soft_argmax/
│   │   └── modulo4_lora/
│   ├── dinov3/                        # JSON, visualizzazioni DINOv3
│   │   ├── modulo1_baseline/
│   │   ├── modulo2_finetuning/
│   │   ├── modulo3_soft_argmax/
│   │   └── modulo4_lora/
│   └── sam/                           # JSON, visualizzazioni SAM
│       ├── modulo1_baseline/
│       ├── modulo2_finetuning/
│       ├── modulo3_soft_argmax/
│       └── modulo4_lora/
│
├── src/                               # Codice Python modulare e riutilizzabile
│   ├── __init__.py
│   ├── data/                          # Data loading e preprocessing
│   │   ├── __init__.py
│   │   ├── dataset.py                 # SemanticCorrespondenceDataset
│   │   └── transforms.py              # Trasformazioni geometriche
│   │
│   ├── models/                        # Modelli backbone (astratti, agnostici)
│   │   ├── __init__.py
│   │   ├── dinov2.py                  # Wrapper DINOv2
│   │   ├── dinov3.py                  # Wrapper DINOv3
│   │   ├── sam.py                     # Wrapper SAM
│   │   └── feature_extractors.py      # Estrattori generici di feature
│   │
│   ├── features/                      # Estrazione e fusione feature (Modulo 2)
│   │   ├── __init__.py
│   │   ├── dift.py                    # Estrazione DIFT da Stable Diffusion
│   │   ├── fusion.py                  # Strategie di fusione multimodale
│   │   └── visualization.py           # Visualizzazione PCA feature
│   │
│   ├── matching/                      # Matching e raffinamento geometrico (Modulo 3)
│   │   ├── __init__.py
│   │   ├── argmax_matcher.py          # Argmax semplice
│   │   ├── soft_argmax_matcher.py     # Global soft-argmax
│   │   ├── window_soft_argmax.py      # Window soft-argmax (core Modulo 3)
│   │   └── losses.py                  # Funzioni di perdita (L2, InfoNCE, etc.)
│   │
│   ├── training/                      # Training loop e LoRA/QLoRA (Modulo 4)
│   │   ├── __init__.py
│   │   ├── trainer.py                 # Training loop unificato
│   │   ├── lora_config.py             # Configurazione LoRA/QLoRA
│   │   └── callbacks.py               # Callback per logging e checkpoint
│   │
│   ├── evaluation/                    # Valutazione e metriche
│   │   ├── __init__.py
│   │   ├── metrics.py                 # PCK@T e metriche correlate
│   │   ├── evaluator.py               # Classe Evaluator per tutti i moduli
│   │   └── visualizations.py          # Visualizzazioni qualitative
│   │
│   └── utils/                         # Utility generiche
│       ├── __init__.py
│       ├── config.py                  # Gestione configurazione centralizzata
│       ├── logging.py                 # Logging setup
│       └── helpers.py                 # Funzioni helper
│
├── notebooks/                         # Notebook Kaggle: 3 notebook per modulo
│   │                                  # (uno per ogni backbone)
│   ├── modulo1_baseline_dinov2.ipynb
│   ├── modulo1_baseline_dinov3.ipynb
│   ├── modulo1_baseline_sam.ipynb
│   │
│   ├── modulo2_finetuning_dinov2.ipynb
│   ├── modulo2_finetuning_dinov3.ipynb
│   ├── modulo2_finetuning_sam.ipynb
│   │
│   ├── modulo3_soft_argmax_dinov2.ipynb
│   ├── modulo3_soft_argmax_dinov3.ipynb
│   ├── modulo3_soft_argmax_sam.ipynb
│   │
│   ├── modulo4_lora_dinov2.ipynb
│   ├── modulo4_lora_dinov3.ipynb
│   ├── modulo4_lora_sam.ipynb
│   │
│   └── master_comparison.ipynb        # (Finale) Comparazione tra backbone e moduli
│
└── docs/                              # Documentazione aggiuntiva
    ├── SETUP.md                       # Setup iniziale
    ├── CONTRIBUTING.md                # Linee guida contribuzione
    ├── MODULE_DETAILS.md              # Dettagli tecnici moduli
    └── TROUBLESHOOTING.md             # Troubleshooting
```

---

## Architettura: Moduli per Membro su Tutti i Backbone

**La vera architettura del progetto:**

Invece di avere ogni membro responsabile di un intero backbone, il progetto prevede che **ogni membro sia responsabile di un modulo specifico su TUTTI e 3 i backbone**.

```
Membro A - Modulo 1: Baseline Training-Free
├── modulo1_baseline_dinov2.ipynb
├── modulo1_baseline_dinov3.ipynb
└── modulo1_baseline_sam.ipynb

Membro B - Modulo 2: Light Fine-tuning Last Layers
├── modulo2_finetuning_dinov2.ipynb
├── modulo2_finetuning_dinov3.ipynb
└── modulo2_finetuning_sam.ipynb

Membro C - Modulo 3: Geometric Refinement - Window Soft-Argmax
├── modulo3_soft_argmax_dinov2.ipynb
├── modulo3_soft_argmax_dinov3.ipynb
└── modulo3_soft_argmax_sam.ipynb

Membro D - Modulo 4: LoRA/QLoRA Fine-tuning
├── modulo4_lora_dinov2.ipynb
├── modulo4_lora_dinov3.ipynb
└── modulo4_lora_sam.ipynb
```

Questo significa:
- **Membro A:** Sviluppa l'infrastruttura base (dataloader, metriche, baseline) e lo applica a DINOv2, DINOv3, SAM
- **Membro B:** Implementa il fine-tuning su ultimi layer e lo applica a tutti e 3 i backbone
- **Membro C:** Implementa il window soft-argmax e lo applica a tutti e 3 i backbone
- **Membro D:** Implementa LoRA/QLoRA e lo applica a tutti e 3 i backbone
- **Membro D (coordinatore):** Inoltre coordina e produce il master comparison finale

**Vantaggi di questa architettura:**
1. Ogni membro diventa esperto di un modulo specifico
2. Ogni membro valida il proprio codice su 3 backbone diversi
3. Esecuzione massimamente parallela: mentre A fa Modulo 1, B/C/D rimangono in attesa ma coordinano
4. Codice riusabile: una volta implementato Modulo 1, viene eseguito 3 volte (per ogni backbone)

---

## Notebook Separati: Strategia di Parallelizzazione

Il progetto utilizza **12 notebook Kaggle separati** (3 per modulo × 4 moduli), ognuno contenente un modulo specifico per un backbone specifico. Questi notebook possono essere eseguiti in parallelo.

### Modulo 1 - Baseline Training-Free (Membro A)

Membro A crea 3 notebook (uno per backbone):

#### `modulo1_baseline_dinov2.ipynb`
- **Durata:** 2-3 ore su T4
- **GPU required:** 1x T4 sufficiente
- **Output:**
  - Dataloader PyTorch (`SemanticCorrespondenceDataset`)
  - Metriche PCK@T per DINOv2 (congelato)
  - File JSON con risultati baseline DINOv2
  - Visualizzazioni (grafici PCK@0.05, @0.1, @0.2)
  - Checkpoint: feature DINOv2 estratte

**Struttura:**
```
1. Importazioni e setup
2. Caricamento dataset SPair-71k
3. Implementazione SemanticCorrespondenceDataset (in src/data/)
4. Caricamento backbone DINOv2
5. Estrazione feature congelate
6. Implementazione metrica PCK
7. Valutazione quantitativa per DINOv2
8. Salvataggio risultati in results/dinov2/modulo1_baseline/
9. Visualizzazioni per-category e globali
10. Salvataggio checkpoint in checkpoints/dinov2/modulo1_baseline/
```

#### `modulo1_baseline_dinov3.ipynb`
- Identico al precedente, ma con DINOv3 come backbone

#### `modulo1_baseline_sam.ipynb`
- Identico al precedente, ma con SAM come backbone (+ upsampling delle feature)

**Deliverable Modulo 1 (Membro A):**
1. Classe PyTorch `SemanticCorrespondenceDataset` in `src/data/dataset.py`
2. Funzione `compute_pck()` in `src/evaluation/metrics.py`
3. 3 notebook completati (uno per backbone)
4. 3 file JSON con baseline results per ogni backbone
5. 3 set di visualizzazioni
6. Codice condiviso in `src/` pulito e riusabile

---

### Modulo 2 - Light Fine-tuning Last Layers (Membro B)

Membro B crea 3 notebook (uno per backbone), riusando dataloader e metriche dal Modulo 1:

#### `modulo2_finetuning_dinov2.ipynb`
- **Durata:** 2-3 ore su T4
- **GPU required:** 1x T4 sufficiente
- **Dipendenze:** Codice da `src/` (dataloader, metriche da Modulo 1)
- **Output:**
  - Modello DINOv2 fine-tuned su ultimi N layer
  - Metriche PCK@T per DINOv2 fine-tuned
  - Comparazione quantitativa: frozen vs fine-tuned
  - Ablation study: impatto di fine-tuning con N=1,2,4,8 layer scongelati
  - File JSON con risultati
  - Visualizzazioni comparazione

**Struttura:**
```
1. Importazioni e setup, caricamento dataloader da src/
2. Caricamento DINOv2 e congelamento backbone
3. Ciclo su N = 1, 2, 4, 8 layer scongelati
4. Per ogni N:
   - Scongela ultimi N layer
   - Configurazione optimizer (learning rate basso)
   - Training loop su SPair-71k
   - Valutazione su validation set
   - Salvataggio modello e metriche
5. Ablation study: tabella e grafici
6. Salvataggio risultati in results/dinov2/modulo2_finetuning/
7. Salvataggio modelli in checkpoints/dinov2/modulo2_finetuning/
8. Visualizzazioni comparazione frozen vs fine-tuned
```

#### `modulo2_finetuning_dinov3.ipynb`
- Identico, ma con DINOv3

#### `modulo2_finetuning_sam.ipynb`
- Identico, ma con SAM

**Deliverable Modulo 2 (Membro B):**
1. Classe `Trainer` in `src/training/trainer.py` per fine-tuning unificato
2. Configurazione optimizer e scheduler in `src/training/`
3. 3 notebook completati (uno per backbone)
4. 3 file JSON con fine-tuning results per ogni backbone
5. 3 set di checkpoint modello fine-tuned
6. 3 set di visualizzazioni ablation study
7. Codice in `src/training/` riusabile

---

### Modulo 3 - Geometric Refinement: Window Soft-Argmax (Membro C)

Membro C crea 3 notebook (uno per backbone), riusando modello fine-tuned dal Modulo 2:

#### `modulo3_soft_argmax_dinov2.ipynb`
- **Durata:** 2-3 ore su T4
- **GPU required:** 1x T4 sufficiente
- **Dipendenze:** Modello fine-tuned dal Modulo 2, dataloader da Modulo 1
- **Output:**
  - Implementazione Window Soft-Argmax differenziabile
  - Metriche PCK@T con window soft-argmax
  - Ablation study: argmax vs soft-argmax vs window soft-argmax
  - Ablation study: window size R = 1, 2, 3, 5
  - File JSON con risultati
  - Visualizzazioni mappe similarità qualitative

**Struttura:**
```
1. Importazioni e setup
2. Caricamento modello DINOv2 fine-tuned dal Modulo 2
3. Caricamento dataloader da src/
4. Implementazione Argmax baseline matcher
5. Implementazione Global Soft-Argmax matcher
6. Implementazione Window Soft-Argmax matcher (core)
7. Implementazione funzioni di perdita (L2 coordinate, InfoNCE, softmax)
8. Ablation study: test matching strategy (argmax vs soft vs window soft)
9. Ablation study: test window size R = 1, 2, 3, 5
10. Valutazione quantitativa per ogni combinazione
11. Salvataggio matcher e configurazione in checkpoints/dinov2/modulo3_soft_argmax/
12. Salvataggio risultati in results/dinov2/modulo3_soft_argmax/
13. Visualizzazioni qualitative (heatmap similarità, predizioni corrette/errate)
```

#### `modulo3_soft_argmax_dinov3.ipynb`
- Identico, ma con DINOv3

#### `modulo3_soft_argmax_sam.ipynb`
- Identico, ma con SAM

**Deliverable Modulo 3 (Membro C):**
1. Classe `WindowSoftArgmaxMatcher` in `src/matching/window_soft_argmax.py`
2. Implementazione loss functions in `src/matching/losses.py`
3. 3 notebook completati (uno per backbone)
4. 3 file JSON con soft-argmax results per ogni backbone
5. 3 set di matcher checkpoint
6. 3 set di visualizzazioni ablation study
7. Codice in `src/matching/` riusabile

---

### Modulo 4 - LoRA/QLoRA Fine-tuning (Membro D)

Membro D crea 3 notebook (uno per backbone), integrando tutti i componenti:

#### `modulo4_lora_dinov2.ipynb`
- **Durata:** 3-4 ore su T4
- **GPU required:** 2x T4 ideale (ma possibile con 1x con ottimizzazioni)
- **Dipendenze:** Tutto (dataloader M1, matcher M3, modello base M2)
- **Output:**
  - Modello DINOv2 fine-tuned con LoRA/QLoRA rank 8-16
  - Metriche PCK@T con LoRA fine-tuning su SPair-71k
  - Metriche PCK@T cross-dataset (PF-Pascal, AP-10K)
  - Analisi memoria: peak VRAM, tempo training
  - Comparazione LoRA vs full fine-tuning (se tempo permette)
  - File JSON con risultati
  - Checkpoint LoRA adapter

**Struttura:**
```
1. Importazioni e setup
2. Caricamento dataloader da src/
3. Caricamento matcher dal Modulo 3
4. Setup BitsAndBytes per quantizzazione 4-bit
5. Caricamento DINOv2 con quantizzazione
6. Configurazione PEFT LoRA (rank 8-16, target modules: attention, MLP)
7. Implementazione training loop unificato con LoRA
8. Fine-tuning su SPair-71k training set
9. Valutazione su SPair-71k validation set
10. Valutazione cross-dataset (PF-Pascal, AP-10K)
11. Analisi memoria e tempo di training
12. Se tempo permette: comparazione LoRA vs full fine-tuning
13. Salvataggio checkpoint LoRA in checkpoints/dinov2/modulo4_lora/
14. Salvataggio risultati in results/dinov2/modulo4_lora/
15. Visualizzazioni (grafici performance, memory usage, speed analysis)
```

#### `modulo4_lora_dinov3.ipynb`
- Identico, ma con DINOv3

#### `modulo4_lora_sam.ipynb`
- Identico, ma con SAM

**Deliverable Modulo 4 (Membro D):**
1. Configurazione LoRA in `src/training/lora_config.py`
2. Trainer aggiornato per LoRA in `src/training/trainer.py`
3. 3 notebook completati (uno per backbone)
4. 3 file JSON con LoRA results per ogni backbone
5. 3 set di checkpoint LoRA adapter
6. 3 set di visualizzazioni cross-dataset evaluation
7. Analisi memoria e speed per ogni backbone
8. Codice in `src/training/` aggiornato per LoRA

---

### Master Comparison: `master_comparison.ipynb` (Membro D - Coordinatore)

**Durata:** 2-3 ore

**Eseguito:** DOPO che tutti e 12 i notebook sono completati

**Output principale:**
- Comparazione quantitativa: DINOv2 vs DINOv3 vs SAM su ogni modulo
- Visualizzazioni comparative finali
- Analisi: quale backbone è migliore? Per quale modulo?
- Analisi: quale modulo contribuisce più al miglioramento?
- Report finale integrato con raccomandazioni

**Struttura:**
```
1. Caricamento risultati JSON da tutti e 12 i notebook
   - results/dinov2/modulo1_baseline/, modulo2_finetuning/, modulo3_soft_argmax/, modulo4_lora/
   - results/dinov3/modulo1_baseline/, modulo2_finetuning/, modulo3_soft_argmax/, modulo4_lora/
   - results/sam/modulo1_baseline/, modulo2_finetuning/, modulo3_soft_argmax/, modulo4_lora/

2. Creazione tabelle comparative:
   - Tabella 1: PCK@0.05, @0.1, @0.2 - Modulo 1 per ogni backbone
   - Tabella 2: PCK@0.1 - Impatto fine-tuning (Modulo 2) per ogni backbone
   - Tabella 3: PCK@0.1 - Impatto soft-argmax vs argmax (Modulo 3) per ogni backbone
   - Tabella 4: PCK@0.1 - Impatto LoRA (Modulo 4) per ogni backbone
   - Tabella 5: PCK@0.1 cross-dataset (PF-Pascal, AP-10K) per ogni backbone
   - Tabella 6: Memory e time analysis per ogni backbone

3. Visualizzazioni comparative:
   - Grafici a barre: backbone comparison per ogni modulo
   - Grafici a linee: progression attraverso i moduli per ogni backbone
   - Heatmaps: performance per categoria e backbone
   - Scatter plot: memory vs accuracy trade-off
   - Grafici speed vs performance per ogni backbone

4. Analisi ablative:
   - Quale modulo contribuisce più al miglioramento per ogni backbone?
   - Quale backbone beneficia più di quale modulo?
   - Quale tecnica (fine-tuning, soft-argmax, LoRA) è più importante?

5. Conclusioni e raccomandazioni:
   - Per accuracy massima: usa [backbone X]
   - Per memoria minima: usa [backbone Y]
   - Per velocità massima: usa [backbone Z]
   - Compromesso migliore: usa [backbone W]

6. Salvataggio report finale e visualizzazioni in results/master_comparison/
```

---

## Modulo 1 - Baseline Training-Free (Membro A)

### Obiettivi e Rationale del Modulo 1

Il Modulo 1 è il fondamento dell'intero progetto e ha tre obiettivi critici:

1. **Infrastruttura di dati robusta**: Un dataloader PyTorch che carica SPair-71k, gestisce risoluzioni e aspect ratio, trasforma keypoint coerentemente. (Implementato una sola volta in `src/`, riusato da tutti gli altri moduli)

2. **Metrica di valutazione PCK**: Implementazione della metrica PCK@T con tutti i dettagli algoritmici. (Implementato una sola volta in `src/`, riusato da tutti)

3. **Baseline quantitativo per ogni backbone**: Per ogni backbone (DINOv2, DINOv3, SAM), stabilire il baseline congelato (senza fine-tuning).

### Preprocessing e Gestione dei Dati

[Identico al documento originale - risoluzione 518x518, padding per aspect ratio, trasformazione coerente keypoint]

### Baseline: Ogni Notebook Estrae il Suo Backbone

- **modulo1_baseline_dinov2.ipynb:** DINOv2 congelato → PCK@T
- **modulo1_baseline_dinov3.ipynb:** DINOv3 congelato → PCK@T
- **modulo1_baseline_sam.ipynb:** SAM congelato (con upsampling) → PCK@T

Questo produce il primo set di dati comparativi: quale backbone naturalmente cattura meglio la corrispondenza semantica senza alcun fine-tuning?

### Implementazione della Metrica PCK

[Identico al documento originale - formula PCK@T, normalization, thresholds]

### Deliverable Specifici del Modulo 1 (Membro A)

1. Classe PyTorch `SemanticCorrespondenceDataset` in `src/data/dataset.py` (condivisa)
2. Funzione `compute_pck()` in `src/evaluation/metrics.py` (condivisa)
3. Classe `Evaluator` in `src/evaluation/evaluator.py` (condivisa)
4. 3 notebook completati:
   - `modulo1_baseline_dinov2.ipynb`
   - `modulo1_baseline_dinov3.ipynb`
   - `modulo1_baseline_sam.ipynb`
5. 3 file JSON con baseline results
6. 3 set di visualizzazioni (PCK@T per ogni backbone)
7. Checkpoint feature estratte per ogni backbone

---

## Modulo 2 - Light Fine-tuning Last Layers (Membro B)

### Obiettivi del Modulo 2

Adattare gli ultimi N layer di ogni backbone al task di semantic correspondence via keypoint supervision da SPair-71k, senza fine-tuning completo (che sarebbe troppo costoso).

### Procedure per Ogni Backbone

**modulo2_finetuning_dinov2.ipynb:**
- Carica modello DINOv2 base
- Ciclo su N = 1, 2, 4, 8 layer scongelati (a partire dal top)
- Per ogni N: fine-tuning con learning rate basso (1e-5 a 5e-5) su SPair-71k
- Salva modello fine-tuned per ogni N
- Valutazione e comparazione

**modulo2_finetuning_dinov3.ipynb:**
- Identico, ma su DINOv3

**modulo2_finetuning_sam.ipynb:**
- Identico, ma su SAM (con gestione feature upsampling)

### Ablation Study

Per ogni backbone, Membro B produce ablation study:
- Frozen baseline (da Modulo 1)
- Fine-tuned con 1 layer scongelato
- Fine-tuned con 2 layer scongelati
- Fine-tuned con 4 layer scongelati
- Fine-tuned con 8 layer scongelati

**Domande risposte:**
- Quale N è ottimale per questo backbone?
- Quanto fine-tuning aiuta?

### Deliverable Modulo 2 (Membro B)

Per ogni backbone:
1. Modello backbone fine-tuned (salvato in checkpoints/)
2. Metriche PCK@T per ogni N
3. Tabella comparazione frozen vs fine-tuned
4. Ablation study completo
5. File JSON con risultati
6. Visualizzazioni

---

## Modulo 3 - Geometric Refinement: Window Soft-Argmax (Membro C)

### Obiettivi del Modulo 3

Sostituire argmax (discreto, non differenziabile) con window soft-argmax differenziabile per ottenere:
- Sub-pixel accuracy (predizioni continue, non discrete)
- Robustezza a rumore nelle mappe di similarità

### Implementazione Window Soft-Argmax

[Identico al documento originale - algoritmo, formula, justification]

### Ablation Studies per Ogni Backbone

Per ogni backbone, Membro C produce ablation studies:

1. **Matching strategy:**
   - Argmax semplice (baseline)
   - Global soft-argmax (su tutta la mappa)
   - Window soft-argmax (su finestra intorno al picco)

2. **Window size R:**
   - R = 1 (finestra 3×3)
   - R = 2 (finestra 5×5)
   - R = 3 (finestra 7×7)
   - R = 5 (finestra 11×11)

**Domande risposte:**
- Il soft-argmax migliora PCK?
- Quale R è ottimale per questo backbone?

### Deliverable Modulo 3 (Membro C)

Per ogni backbone:
1. Implementazione WindowSoftArgmaxMatcher (in src/matching/)
2. Metriche PCK@T per ogni matching strategy
3. Metriche PCK@T per ogni window size
4. Tabella comparazione completa
5. File JSON con risultati
6. Visualizzazioni qualitative (heatmap, predizioni)

---

## Modulo 4 - LoRA/QLoRA Fine-tuning (Membro D)

### Obiettivi del Modulo 4

Applicare LoRA (Low-Rank Adaptation) per fine-tuning efficiente in memoria, permettendo un adattamento più profondo del modello rispetto al Modulo 2, senza esaurire memoria Kaggle.

### Configurazione LoRA/QLoRA per Ogni Backbone

**modulo4_lora_dinov2.ipynb:**
- DINOv2-Base con LoRA rank 8-16
- Quantizzazione 4-bit con BitsAndBytes
- Target modules: attention (q, v) e MLP (fc1, fc2)
- Fine-tuning con window soft-argmax matcher + InfoNCE loss
- Evaluation su SPair-71k, PF-Pascal, AP-10K

**modulo4_lora_dinov3.ipynb:**
- Identico, ma su DINOv3

**modulo4_lora_sam.ipynb:**
- SAM encoder con LoRA
- Gestione feature upsampling appropriato

### Cross-Dataset Evaluation

Ogni notebook valuta il modello LoRA fine-tuned su:
- **SPair-71k** (dataset di training)
- **PF-Pascal** (new domain, benchmark classico)
- **AP-10K** (new domain, animali diversi)

Questo mostra quale backbone generalizza meglio a nuovi domini.

### Deliverable Modulo 4 (Membro D)

Per ogni backbone:
1. Modello fine-tuned con LoRA
2. Metriche PCK@T su SPair-71k
3. Metriche PCK@T cross-dataset (PF-Pascal, AP-10K)
4. Analisi memoria: peak VRAM, average VRAM, tempo training
5. Analisi speed: tempo inference per immagine
6. Checkpoint LoRA adapter (leggero, facile da condividere)
7. File JSON con risultati
8. Visualizzazioni (PCK comparison, memory vs accuracy, speed analysis)

---

## Integrazione del Sistema e Reporting

### Convergenza dei Moduli

I 4 moduli vengono completati sequenzialmente, ma **ogni membro lavora su un modulo specifico su TUTTI e 3 i backbone in parallelo**:

```
Timeline Parallela:

Settimana 1:
- Membro A: Completa Modulo 1
  - modulo1_baseline_dinov2.ipynb
  - modulo1_baseline_dinov3.ipynb
  - modulo1_baseline_sam.ipynb

Settimana 2:
- Membro B: Completa Modulo 2 (su tutti e 3 i backbone in parallelo)
  - modulo2_finetuning_dinov2.ipynb
  - modulo2_finetuning_dinov3.ipynb
  - modulo2_finetuning_sam.ipynb

Settimana 3:
- Membro C: Completa Modulo 3 (su tutti e 3 i backbone in parallelo)
  - modulo3_soft_argmax_dinov2.ipynb
  - modulo3_soft_argmax_dinov3.ipynb
  - modulo3_soft_argmax_sam.ipynb

Settimana 3-4:
- Membro D: Completa Modulo 4 (su tutti e 3 i backbone in parallelo)
  - modulo4_lora_dinov2.ipynb
  - modulo4_lora_dinov3.ipynb
  - modulo4_lora_sam.ipynb

Settimana 4:
- Membro D: Esegue master_comparison.ipynb e produce report finale
```

### Struttura del Report Finale

Il report finale (20-30 pagine) contiene:

1. **Executive Summary**: 
   - Quale backbone è migliore?
   - Quale modulo contribuisce più al miglioramento?

2. **Introduzione e Motivazione**: [Identico documento originale]

3. **Related Work**: [Identico documento originale]

4. **Metodologia**: Sezioni separate per ogni modulo:
   - Modulo 1: Dataset, preprocessing, dataloader, metrica PCK
   - Modulo 2: Fine-tuning ultimi N layer
   - Modulo 3: Window soft-argmax
   - Modulo 4: LoRA/QLoRA

5. **Risultati e Esperimenti**: Tabelle e grafici comparativi

| Modulo | DINOv2 | DINOv3 | SAM | Miglior Miglioria |
|--------|--------|--------|-----|------------------|
| Baseline (M1) | XX.X% | YY.Y% | ZZ.Z% | - |
| +Fine-tuning (M2) | +AA.A% | +BB.B% | +CC.C% | ? |
| +Soft-argmax (M3) | +DD.D% | +EE.E% | +FF.F% | ? |
| +LoRA (M4) | +GG.G% | +HH.H% | +II.I% | ? |
| Cross-dataset | XX.X% | YY.Y% | ZZ.Z% | ? |

6. **Visualizzazioni Comparative:**
   - Grafici a barre: backbone comparison per ogni modulo
   - Grafici a linee: progression dei moduli per ogni backbone
   - Heatmaps: performance per categoria
   - Scatter plot: memory vs accuracy

7. **Ablation Study:**
   - Quale modulo è più importante?
   - Quale backbone beneficia più di quale modulo?

8. **Conclusioni e Raccomandazioni**

9. **Riferimenti**

---

## Conclusione

Con questa **architettura modulo-per-membro su tutti i backbone**, il team può:

1. **Parallelizzare realmente:** Membro A non blocca B, B non blocca C, etc.
2. **Sviluppare expertise:** Ogni membro diventa esperto del suo modulo
3. **Riusare codice:** Infrastruttura base creata una volta, usata 3 volte
4. **Confrontare scientificamente:** Valutazione sistematica di DINOv2 vs DINOv3 vs SAM

**Timeline:** 4 settimane, parallelismo reale, consegna finale completa e rigorosa.