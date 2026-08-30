# Risposte alle Domande d'Esame

## Domanda di Teoria 1

**Descrivere e confrontare la `Principal Component Analysis (PCA)` e la `Linear Discriminant Analysis (LDA)`, trattando i seguenti aspetti:**

*   **Obiettivi dei due modelli e loro formulazione**
*   **Funzione obiettivo di `training` dei due modelli**
*   **Caratteristiche delle `PCA` `principal components` e delle `LDA` `discriminant directions`**
*   **Come i modelli possono essere impiegati in `classification tasks`**

### Risposta

#### 1. Obiettivi e Formulazione

*   **`Principal Component Analysis (PCA)`**: Una tecnica di riduzione della dimensionalità **`unsupervised`**. Il suo obiettivo è trovare una rappresentazione a dimensionalità inferiore dei dati che catturi la massima **varianza** possibile. La formulazione matematica si basa sulla ricerca di una matrice di proiezione ortogonale $P$ che trasforma i dati originali $x$ in dati a dimensionalità ridotta $y = P^T x$. La soluzione si ottiene tramite la decomposizione agli autovalori della **matrice di covarianza** dei dati $C$, dove le colonne di $P$ sono gli autovettori corrispondenti ai maggiori autovalori.

*   **`Linear Discriminant Analysis (LDA)`**: Una tecnica di riduzione della dimensionalità **`supervised`**, progettata specificamente per la classificazione. Il suo obiettivo è trovare un sottospazio che massimizzi la **separabilità tra le classi**. Questo si ottiene massimizzando il rapporto tra la **`between-class scatter`** ($S_B$), che misura la separazione tra le medie delle classi, e la **`within-class scatter`** ($S_W$), che misura la dispersione dei dati all'interno di ciascuna classe.

#### 2. Funzione Obiettivo di `Training`

*   **`PCA`**: L'obiettivo è trovare una matrice di proiezione $P$ che minimizzi l'**`errore quadratico medio di ricostruzione`** tra i dati originali $x_i$ e le loro ricostruzioni $\hat{x}_i = P P^T x_i$. Questo è matematicamente equivalente a massimizzare la varianza dei dati proiettati, $\frac{1}{N} \sum_i ||y_i - \mu_y||^2$. La soluzione consiste nel selezionare come colonne di $P$ gli $m$ autovettori della matrice di covarianza $C$ associati agli $m$ autovalori più grandi.

*   **`LDA`**: L'obiettivo è trovare una matrice di proiezione $W$ che massimizzi il **`Fisher's discriminant ratio`**:
    
    $$J(W) = \frac{\det(W^T S_B W)}{\det(W^T S_W W)}$$
    
    La soluzione a questo problema di ottimizzazione si ottiene risolvendo il **problema agli autovalori generalizzato**:
    
    $$S_B w = \lambda S_W w$$
    
    Le colonne di $W$ sono gli autovettori generalizzati corrispondenti ai più grandi autovalori.

#### 3. Caratteristiche delle Direzioni

*   **`PCA Principal Components`**: Sono gli **autovettori** della matrice di covarianza dei dati. Per costruzione, sono **`ortogonali`** tra loro, il che significa che catturano direzioni di varianza non correlate. Sono ordinate in base ai loro autovalori corrispondenti: la prima componente principale cattura la massima varianza, la seconda cattura la massima varianza rimanente, e così via. Il loro calcolo è **`unsupervised`**, poiché non utilizza le etichette di classe.

*   **`LDA Discriminant Directions`**: Sono gli **autovettori generalizzati** della matrice $S_W^{-1} S_B$. A differenza della PCA, queste direzioni **`non sono generalmente ortogonali`**. Sono ordinate in base alla loro capacità di separare le classi (cioè, in base ai loro autovalori generalizzati). Il loro calcolo è **`supervised`**, e il numero massimo di direzioni discriminanti che si possono estrarre è $C - 1$, dove $C$ è il numero di classi.

#### 4. Impiego nella Classificazione

*   **`PCA`**: Viene tipicamente utilizzata come passo di **`pre-processing`** per ridurre la dimensionalità. Questo può aiutare a mitigare la "`curse of dimensionality`", ridurre il rumore e accelerare il training dei modelli successivi. Tuttavia, poiché la PCA è `unsupervised`, potrebbe scartare direzioni a bassa varianza che, pur contenendo poca varianza, potrebbero essere cruciali per la separazione delle classi.

*   **`LDA`**: È intrinsecamente un metodo orientato alla classificazione. I dati vengono proiettati sul sottospazio `LDA` e un classificatore viene addestrato su queste nuove `feature` altamente discriminanti. Una `pipeline` comune e molto efficace è **`PCA+LDA`**: prima si applica la PCA per ridurre la dimensionalità e regolarizzare il problema (ad esempio, per evitare che $S_W$ sia singolare), e poi si applica la LDA per trovare il sottospazio ottimale per la classificazione.

---

## Domanda di Teoria 2

**Considerando l'approccio della `Linear Discriminant Analysis (LDA)` per la classificazione binaria e il classificatore `Tied MVG` binario, dettagliare:**

*   **Formulazione del modello, obiettivo di `training` e procedura di `inference` (cioè come impiegare il modello per la classificazione) del classificatore `LDA`**
*   **Assunzioni del modello, obiettivo di `training` e procedura di `inference` del classificatore `Tied MVG`**
*   **La relazione tra i due modelli**
*   **La forma delle `decision rules` dei classificatori binari `LDA` e `Tied MVG`**

### Risposta

#### 1. Classificatore `LDA`

*   **Formulazione e Obiettivo**: Per la classificazione binaria, la `LDA` cerca un singolo vettore di proiezione $w$ che massimizza il `Fisher's discriminant ratio`:
    
    $$J(w) = \frac{w^T S_B w}{w^T S_W w}$$
    
    Questo massimizza la separazione tra le medie delle due classi proiettate, $\tilde{\mu}_1$ e $\tilde{\mu}_2$, minimizzando al contempo la varianza all'interno di ciascuna classe proiettata, $\tilde{\sigma}_1^2$ e $\tilde{\sigma}_2^2$. La soluzione ottimale per il vettore di proiezione è data da:
    
    $$w \propto S_W^{-1}(\mu_1 - \mu_2)$$
*   **`Inference`**: Un nuovo campione $x$ viene classificato proiettandolo sul vettore $w$ per ottenere uno `score` $s = w^T x$. Questo `score` viene quindi confrontato con una soglia $t$, che è tipicamente il punto medio tra le medie delle classi proiettate. Se $s \geq t$, il campione viene assegnato a una classe; altrimenti, viene assegnato all'altra.

#### 2. Classificatore `Tied MVG`

*   **Assunzioni e Obiettivo**: Questo è un modello **generativo** che assume che i dati per ogni classe seguano una distribuzione Gaussiana multivariata. La sua assunzione chiave è che tutte le classi condividano la **`stessa matrice di covarianza`** ($\Sigma_c = \Sigma$ per tutte le classi $c$). Ciò implica che le distribuzioni di classe hanno centri diversi ($\mu_c$) ma la stessa forma e orientamento. L'obiettivo del `training` è trovare le `Maximum Likelihood Estimates (MLE)` per le medie di classe $\mu_c$ e l'unica matrice di covarianza condivisa $\Sigma$, oltre ai `priors` di classe $\pi_c$.
*   **`Inference`**: Utilizza il **teorema di Bayes** per classificare un nuovo campione $x$. La decisione viene presa selezionando la classe $c$ che massimizza la probabilità a posteriori $P(C=c|x)$, che è proporzionale al prodotto della `likelihood` $f(x|C=c)$ e del `prior` $P(C=c)$. 
    
    Utilizziamo il **logaritmo** per stabilità numerica e per trasformare prodotti in somme. La **log posterior probability** è data da:
    
    $$\log P(C=c|x) = -\frac{d}{2}\log(2\pi) - \frac{1}{2}\log|\Sigma| - \frac{1}{2}(x-\mu_c)^T\Sigma^{-1}(x-\mu_c) + \log\pi_c + \text{costante}$$

#### 3. Relazione tra `LDA` e `Tied MVG`

I due modelli sono profondamente connessi. Il confine decisionale per un classificatore `Tied MVG` si trova dove le probabilità a posteriori per due classi sono uguali, ovvero $P(C=1|x) = P(C=0|x)$. Analizzando il rapporto delle `log-posterior-odds`:

$$\log \frac{P(C=1|x)}{P(C=0|x)} = \log \frac{f(x|C=1)}{f(x|C=0)} + \log \frac{P(C=1)}{P(C=0)}$$

I termini quadratici in $x$ (cioè $x^T \Sigma^{-1} x$) presenti nelle `likelihood` Gaussiane sono identici per entrambe le classi a causa della matrice di covarianza condivisa, e quindi si annullano. Ciò si traduce in un `log-likelihood ratio` che è una funzione lineare di $x$, portando a un confine decisionale **`identico`** a quello trovato dalla `LDA`.

#### 4. Forma delle `Decision Rules`

*   **`LDA`**: La `decision rule` è esplicitamente **`lineare`**. Il confine decisionale è un iperpiano nello spazio delle `feature` definito dall'equazione $w^T x - t = 0$, dove $w$ è la direzione discriminante e $t$ è la soglia.
*   **`Tied MVG`**: Anche la `decision rule` è **`lineare`**. L'assunzione di una matrice di covarianza condivisa è precisamente ciò che semplifica il `log-likelihood ratio` a una funzione lineare di $x$, risultando in un confine decisionale lineare. La `LDA` può quindi essere vista come un caso particolare del classificatore `Tied MVG`.

---

## Domanda di Teoria 3

**Descrivere in dettaglio il classificatore `Multivariate Gaussian (MVG)`, trattando i seguenti aspetti:**

*   **Assunzioni del modello**
*   **Stima dei parametri del modello**
*   **Come il modello può essere impiegato per eseguire l'"`inference`" (cioè classificare un campione di test) per problemi sia `multi-class` che binari**
*   **La forma delle `decision rules` per problemi binari**
*   **Varianti del modello `Naive Bayes` e `Tied Covariance`, focalizzandosi su:**
    *   **Differenze con il modello standard (non vincolato) in termini di assunzioni e `decision rules`**
    *   **Vantaggi e limitazioni rispetto al modello non vincolato**

### Risposta

#### 1. Assunzioni del Modello

Il classificatore **`Multivariate Gaussian (MVG)`** è un modello **generativo**. La sua assunzione fondamentale è che i dati delle `feature` per ogni classe $c$ siano estratti da una distinta **distribuzione Gaussiana multivariata**, $\mathcal{N}(\mu_c, \Sigma_c)$. Questo significa che ogni classe è modellata come un iper-ellissoide nello spazio delle `feature`, caratterizzato da un proprio **vettore delle medie** $\mu_c$ (il centro dell'ellissoide) e da una propria **matrice di covarianza** $\Sigma_c$ (che ne definisce la forma, la dimensione e l'orientamento).

#### 2. Stima dei Parametri del Modello

I parametri del modello ($\{\mu_c, \Sigma_c, \pi_c\}$ per tutte le classi) sono stimati dai dati di `training` etichettati utilizzando la **`Maximum Likelihood Estimation (MLE)`**. Per ogni classe $c$, le stime sono calcolate come segue:

*   **Prior di classe**: $\hat{\pi}_c = \frac{N_c}{N}$, dove $N_c$ è il numero di campioni della classe $c$ e $N$ è il numero totale di campioni. Questa è la frequenza relativa della classe.
*   **Media di classe**: $\hat{\mu}_c = \frac{1}{N_c} \sum_{i: y_i = c} x_i$. Questa è semplicemente la media campionaria dei vettori di `feature` appartenenti alla classe $c$.
*   **Matrice di covarianza di classe**: $\hat{\Sigma}_c = \frac{1}{N_c} \sum_{i: y_i = c} (x_i - \hat{\mu}_c)(x_i - \hat{\mu}_c)^T$. Questa è la matrice di covarianza campionaria per la classe $c$.

Queste stime massimizzano la `log-likelihood` dei dati di `training` osservati.

#### 3. `Inference`

Sia per problemi binari che `multi-class`, l'`inference` si basa sul **teorema di Bayes**. Per classificare un nuovo campione $x$, si calcola la probabilità a posteriori per ogni classe $c$ e si sceglie la classe che la massimizza:

$$\text{Classe Predetta} = \arg\max_c P(C=c|x) = \arg\max_c P(C=c) f(x|C=c)$$

Dove $P(C=c)$ è il `prior` di classe (stimato come $\hat{\pi}_c$) e $f(x|C=c)$ è la `likelihood` del campione data la classe, calcolata usando la PDF Gaussiana $\mathcal{N}(x | \hat{\mu}_c, \hat{\Sigma}_c)$. Per stabilità numerica e convenienza, i calcoli vengono spesso eseguiti nello spazio delle `log-probabilities`:

$$\log P(C=c|x) \propto \log f(x|C=c) + \log P(C=c)$$

#### 4. Forma delle `Decision Rules` per Problemi Binari

Il confine decisionale si trova dove le probabilità a posteriori delle due classi sono uguali, cioè $P(C=1|x) = P(C=0|x)$. Questo equivale a confrontare il `log-likelihood ratio` con una soglia dipendente dai `priors`:

$$\log \frac{f(x|C=1)}{f(x|C=0)} > \log \frac{P(C=0)}{P(C=1)}$$

Per il modello `MVG` standard, le matrici di covarianza $\Sigma_1$ e $\Sigma_2$ sono diverse. Il `log-likelihood ratio` conterrà termini quadratici in $x$ (del tipo $x^T A x$), il che significa che il confine decisionale è una funzione **`quadratica`** di $x$. Geometricamente, questo corrisponde a forme come iper-paraboloidi o iper-ellissoidi.

#### 5. Varianti `Naive Bayes` e `Tied Covariance`

*   **`Naive Bayes Gaussian Classifier`:**
    *   **Assunzioni e `Rules`**: Fa l'assunzione forte che le `feature` siano **`condizionatamente indipendenti`** data la classe. Questo vincola ogni matrice di covarianza $\Sigma_c$ ad essere **`diagonale`**. Il confine decisionale rimane **`quadratico`**, ma le forme quadratiche risultanti sono allineate con gli assi delle `feature`.
    *   **`Pros & Cons`**: **Vantaggio**: Riduce drasticamente il numero di parametri da stimare (da $O(D^2)$ a $O(D)$ per covarianza), rendendolo molto efficiente e robusto contro l'`overfitting`, specialmente con dati ad alta dimensionalità. **Svantaggio**: L'assunzione di indipendenza è spesso una semplificazione eccessiva della realtà e può portare a una perdita di `performance` se le `feature` sono correlate.

*   **`Tied Covariance Gaussian Classifier`:**
    *   **Assunzioni e `Rules`**: Assume che tutte le classi condividano la **`stessa matrice di covarianza`** ($\Sigma_c = \Sigma$ per ogni $c$). Poiché i termini quadratici nel `log-likelihood ratio` si annullano, il confine decisionale si semplifica e diventa **`lineare`**.
    *   **`Pros & Cons`**: **Vantaggio**: Agisce come una forma di regolarizzazione, offrendo un buon compromesso tra il modello completo e quello `Naive Bayes`. È più stabile del modello completo quando i dati sono limitati. **Svantaggio**: L'assunzione di covarianza condivisa può essere troppo restrittiva se le classi hanno distribuzioni con forme e orientamenti naturalmente diversi.

---

## Domanda di Teoria 4

**Descrivere il modello di `binary logistic regression` per la classificazione, trattando i seguenti aspetti:**

*   **`Classification rule` del modello di `binary logistic regression`**
*   **Interpretazione probabilistica del modello e del `classification score`**
*   **Stima dei parametri del modello e possibili interpretazioni della `training objective function`**

**Sia la `logistic regression` che le `Support Vector Machines (SVM)` possono essere interpretate come approcci di `risk minimization`.**

*   **Confrontare le `objective functions` dei due modelli**
*   **Spiegare possibili approcci per ottenere `non-linear decision functions` con questi due classificatori**

### Risposta

#### 1. `Binary Logistic Regression Model`

*   **`Classification Rule`**: La regressione logistica è un classificatore **discriminativo** che apprende un **`confine decisionale lineare`**. Un nuovo campione $x$ viene classificato in base al segno di uno `score` lineare $s = w^T x + b$. La regola è: assegna alla classe 1 se $s > 0$, altrimenti alla classe 0.

*   **Interpretazione Probabilistica**: Il modello assume che il `log-posterior-odds ratio` sia una funzione lineare delle `feature`: 

$$\log \frac{P(C=1|x)}{P(C=0|x)} = w^T x + b$$

Di conseguenza, la probabilità a posteriori della classe positiva viene modellata direttamente passando lo `score` $s$ attraverso la **funzione sigmoide**: 

$$P(C=1|x) = \sigma(w^T x + b) = \frac{1}{1 + e^{-(w^T x + b)}}$$

Lo `score` $s$ stesso è quindi interpretabile come il `log-odds` della classe positiva.

*   **Stima dei Parametri**: I parametri ($w, b$) sono stimati massimizzando la `conditional log-likelihood` sui dati di `training`. Questo è equivalente a minimizzare la **`negative log-likelihood`**, nota anche come **`binary cross-entropy loss`**:

$$J(w, b) = - \sum_{i=1}^{N} \left[ y_i \log(\sigma(w^T x_i + b)) + (1-y_i) \log(1-\sigma(w^T x_i + b)) \right]$$

dove $y_i \in \{0, 1\}$ sono le etichette. Questa funzione obiettivo è **convessa**, garantendo un minimo globale, ma non ha una soluzione in forma chiusa e deve essere ottimizzata con metodi iterativi come la discesa del gradiente. Per prevenire l'`overfitting` con dati linearmente separabili, si aggiunge tipicamente un termine di **regolarizzazione L2**, $\frac{\lambda}{2} ||w||^2$.

#### 2. Confronto con le `SVM`

*   **`Objective Functions`:**
    *   **`Logistic Regression`**: Minimizza la **`logistic loss`** (o `cross-entropy`). Questa `loss` è **liscia (differenziabile ovunque)** e assegna una penalità a tutti i campioni, anche a quelli classificati correttamente con alta confidenza. Incoraggia il modello a produrre probabilità a posteriori accurate.
    *   **`SVM`**: Minimizza la **`hinge loss`**, $\max(0, 1 - y_i(w^T x_i + b))$. Questa `loss` è **zero** per i punti classificati correttamente che si trovano al di fuori del `margin` ($y_i s_i \ge 1$). Penalizza solo i punti che violano il `margin` (i `support vectors`), il che porta a una soluzione **`sparsa`** in cui solo i `support vectors` definiscono il confine.

*   **`Non-linear Decision Functions`:**
    *   **`Logistic Regression`**: Ottiene la non linearità tramite l'**`espansione delle feature`**. Si creano manualmente nuove `feature` non lineari (es. $x_1^2, x_1 x_2$) e si addestra un modello lineare su questo spazio espanso. Questo produce un confine non lineare nello spazio originale, ma può essere computazionalmente costoso e soggetto a `overfitting`.
    *   **`SVM`**: Ottiene la non linearità in modo più efficiente e potente con il **`kernel trick`**. Questa tecnica permette alla `SVM` di operare in uno spazio delle `feature` a dimensionalità potenzialmente infinita, sostituendo i `prodotti scalari` ($x_i^T x_j$) con una `kernel function` (es. `polynomial` o `RBF kernel`). Questo evita il calcolo esplicito della trasformazione delle `feature`, rendendo l'apprendimento di confini complessi computazionalmente fattibile.


---

## Domanda di Teoria 5

**Descrivere il classificatore `Support Vector Machine (SVM)`, trattando i seguenti aspetti:**

*   **`Classification rule` di `SVM` e interpretazione dello `SVM score`**
*   **Il concetto di `margin`**
*   **Formulazione primale (sia come programmazione quadratica convessa vincolata che con `hinge loss`) e duale della `objective function`, e relazione tra le soluzioni primale e duale**
*   **`SVM` per la classificazione non lineare**

### Risposta

#### 1. `SVM Classification Rule` e `Score Interpretation`

*   **`Rule`**: Una `SVM` lineare classifica un campione $x$ in base al segno di uno `score` lineare $s = w^T x + b$. Il confine decisionale è l'iperpiano definito da $s = 0$. Se $s > 0$, il campione viene assegnato a una classe; se $s < 0$, all'altra.
*   **`Score Interpretation`**: Lo `score` $s$ non è una probabilità. Rappresenta la **`distanza geometrica con segno`** del campione dall'iperpiano decisionale, normalizzata dalla norma del vettore dei pesi $||w||$. La grandezza dello `score` indica la "confidenza" della classificazione in termini di distanza dal confine.

#### 2. Il Concetto di `Margin`

Il **`margin`** è il "corridoio" o la "strada" che separa le due classi, centrato sull'iperpiano decisionale. Le `SVM` sono **`maximum margin classifiers`**: il loro obiettivo primario è trovare l'iperpiano che massimizza l'ampiezza di questo `margin`. L'idea è che un `margin` più ampio corrisponda a una migliore capacità di generalizzazione su dati non visti, poiché il confine è il più lontano possibile da entrambi i `cluster` di dati. I punti dati che si trovano esattamente sui bordi del `margin` sono chiamati **`support vectors`** e sono gli unici punti che definiscono la posizione e l'orientamento dell'iperpiano.

#### 3. Formulazioni Primale e Duale

*   **`Primal Formulation`**: L'obiettivo è trovare i parametri $w$ e $b$ che minimizzano $\frac{1}{2} ||w||^2$ (che è equivalente a massimizzare il `margin` $\frac{1}{||w||}$) soggetto ai vincoli $y_i (w^T x_i + b) \ge 1$ per tutti i campioni $i$. Per dati non separabili, la formulazione **`soft-margin`** introduce delle `slack variables` $\xi_i$ e un termine di penalità $C \sum_i \xi_i$ per gestire le violazioni del `margin`. Questo equivale a minimizzare una funzione di costo che include la **`hinge loss`**.
*   **`Dual Formulation`**: Derivata dalla formulazione primale tramite ottimizzazione lagrangiana, massimizza una funzione obiettivo rispetto ai moltiplicatori di Lagrange $\alpha_i$: 

$$W(\alpha) = \sum_i \alpha_i - \frac{1}{2} \sum_{i,j} \alpha_i \alpha_j y_i y_j x_i^T x_j$$

soggetto a $\sum_i \alpha_i y_i = 0$ e $0 \le \alpha_i \le C$. La caratteristica fondamentale è che la `dual objective function` dipende solo dai **`prodotti scalari`** ($x_i^T x_j$) dei campioni di `training`.

**Relazione**: I problemi primale e duale portano alla stessa soluzione ottimale. Le condizioni di Karush-Kuhn-Tucker (KKT) collegano le due formulazioni e rivelano che il vettore dei pesi ottimale 

$$w = \sum_i \alpha_i y_i x_i$$

è una combinazione lineare solo dei **`support vectors`** (i campioni per cui $\alpha_i > 0$). Questo rende la soluzione della `SVM` **sparsa**, poiché dipende solo da un sottoinsieme dei dati di `training`.

#### 4. `SVM` per la Classificazione Non Lineare

Le `SVM` gestiscono la classificazione non lineare in modo molto efficace tramite il **`kernel trick`**. Poiché la formulazione duale dipende solo dai prodotti scalari, possiamo sostituire il prodotto scalare standard $x_i^T x_j$ con una **`kernel function`** non lineare $k(x_i, x_j)$. Questo equivale a mappare implicitamente i dati in uno spazio a dimensionalità superiore dove diventano linearmente separabili, senza mai dover calcolare esplicitamente la trasformazione. Kernel comuni includono:
*   **`Polynomial kernel`**: $k(x_i, x_j) = (x_i^T x_j + c)^d$
*   **`RBF (Gaussian) kernel`**: $k(x_i, x_j) = \exp(-\gamma ||x_i - x_j||^2)$

Questo permette alle `SVM` di apprendere confini decisionali estremamente complessi e non lineari in modo computazionalmente efficiente.

---

## Domanda di Teoria 6

**Descrivere i `Gaussian Mixture Models (GMM)` nel contesto della `density estimation` e della `pattern classification`, trattando i seguenti aspetti:**

*   **Definizione del modello, interpretazione dei parametri del modello e formulazione del `GMM` come `latent variable model`**
*   **Stima dei parametri del modello**
*   **Come il modello può essere utilizzato per risolvere problemi di classificazione, inclusi i `open-set classification tasks`**
*   **Potenziali `issues` dei `GMM`, possibili modi per affrontare questi `issues`, e possibili `variations` del modello**

### Risposta

#### 1. Definizione e Formulazione del `GMM`

*   **Definizione**: Un **`Gaussian Mixture Model (GMM)`** è un modello di densità probabilistica che assume che i dati osservati siano generati da una **somma pesata di $K$ distribuzioni Gaussiane** (o componenti). La sua funzione di densità di probabilità (PDF) è:

$$p(x) = \sum_{k=1}^{K} w_k \mathcal{N}(x | \mu_k, \Sigma_k)$$
*   **Parametri e Visione Latente**: Il modello è definito dai **pesi** ($w_k$, con $\sum w_k = 1$), dalle **medie** ($\mu_k$) e dalle **covarianze** ($\Sigma_k$) di ciascuna delle $K$ componenti. Può essere interpretato come un **`latent variable model`**: per ogni punto $x_i$, esiste una variabile latente (nascosta) $z_i$ che indica da quale delle $K$ componenti il punto è stato generato. Questo fornisce un "`soft clustering`" dei dati, dove ogni punto ha una probabilità di appartenere a ciascuna componente.

#### 2. Stima dei Parametri

Poiché le assegnazioni alle componenti ($z_i$) sono latenti, la stima dei parametri non può essere fatta direttamente. Si utilizza l'algoritmo **`Expectation-Maximization (EM)`**, un metodo iterativo che alterna due passi fino a convergenza:

*   **`E-step` (Expectation)**: In questo passo, si calcola la probabilità a posteriori (chiamata **`responsibility`**) che ogni punto $x_i$ appartenga a ciascuna componente $k$, dati i parametri correnti del modello. La formula per la `responsibility` $\gamma_{ik}$ è:

$$\gamma_{ik} = P(Z_i=k | X_i=x_i, \theta^{(t)}) = \frac{w_k^{(t)} \mathcal{N}(x_i|\mu_k^{(t)}, \Sigma_k^{(t)})}{\sum_{j=1}^{K} w_j^{(t)} \mathcal{N}(x_i|\mu_j^{(t)}, \Sigma_j^{(t)})}$$

Questa `responsibility` rappresenta l'assegnazione "soft" di ogni punto a ogni `cluster`.

*   **`M-step` (Maximization)**: In questo passo, si aggiornano i parametri del modello ($w_k, \mu_k, \Sigma_k$) per massimizzare la `log-likelihood` attesa dei dati completi (dati osservati + variabili latenti). Gli aggiornamenti utilizzano le `responsibilities` calcolate nell'E-step come pesi:
    *   Pesi: 

    $$ w_k^{\text{new}} = \frac{1}{N}\sum_{i=1}^N \gamma_{ik} $$

    *   Medie: 

    $$ \mu_k^{\text{new}} = \frac{\sum_i \gamma_{ik} x_i}{\sum_i \gamma_{ik}} $$

    *   Covarianze: 

    $$ \Sigma_k^{\text{new}} = \frac{\sum_i \gamma_{ik} (x_i - \mu_k^{\text{new}})(x_i - \mu_k^{\text{new}})^T}{\sum_i \gamma_{ik}} $$

Il processo si ripete fino a quando la `log-likelihood` dei dati non smette di aumentare significativamente.

#### 3. `GMM` per la Classificazione

*   **Classificazione Standard**: In un approccio generativo, si addestra un `GMM` separato per ogni classe $c$ per modellare la sua `class-conditional density` $f(x|C=c)$. Per classificare un nuovo campione $x$, si calcola la `likelihood` di $x$ sotto il `GMM` di ogni classe. Quindi, si applica il **teorema di Bayes**, combinando la `likelihood` con il `prior` di classe $P(C=c)$, e si assegna il campione alla classe con la più alta probabilità a posteriori.
*   **`Open-set Classification`**: I `GMM` sono molto utili per i `task` di `open-set`. Si può addestrare un `GMM` aggiuntivo su un `dataset` di campioni noti per essere "`unknown`" (cioè non appartenenti a nessuna delle classi target). Quando arriva un nuovo campione, la sua `likelihood` viene valutata rispetto a tutti i modelli di classe e al modello `unknown`. Se la `likelihood` è massima per il modello `unknown`, il campione viene rigettato, permettendo al sistema di gestire input imprevisti.

#### 4. Problematiche, Soluzioni e Varianti

**Problematiche Principali:**

*   **`Singularities`**: L'algoritmo EM può portare a soluzioni degeneri in cui una componente Gaussiana "collassa" su un singolo punto dati, causando una matrice di covarianza singolare e una `likelihood` infinita. Questo rende il modello instabile.
*   **`Local Optima`**: L'algoritmo EM garantisce solo la convergenza a un massimo locale della `log-likelihood`, non a quello globale. La soluzione finale dipende fortemente dall'inizializzazione dei parametri.
*   **`Model Selection` (Scelta di K)**: Determinare il numero ottimale di componenti $K$ è un problema complesso. Un $K$ troppo alto porta a `overfitting`, mentre un $K$ troppo basso porta a `underfitting`.

**Soluzioni e Strategie:**

*   **Regolarizzazione contro `Singularities`**: Per evitare covarianze singolari, si può aggiungere una piccola costante alla diagonale della matrice di covarianza durante l'aggiornamento, assicurando che rimanga sempre invertibile.
*   **Miglioramento dell'Inizializzazione**: Per mitigare il problema dei massimi locali, si possono usare strategie di inizializzazione intelligenti come il `K-means clustering` per trovare i centri iniziali, o eseguire l'algoritmo più volte con `multiple random restarts` e scegliere la soluzione con la `likelihood` finale più alta.
*   **Selezione del Numero di Componenti**: Per scegliere $K$, si possono usare criteri informativi come `AIC` o `BIC`, che penalizzano la complessità del modello, oppure tecniche di `cross-validation` per valutare le `performance` di generalizzazione del modello su dati non visti.

**Varianti del Modello:**

I `Gaussian Mixture Models` possono essere adattati a diverse situazioni pratiche attraverso specifiche assunzioni sulle matrici di covarianza, che permettono di bilanciare la complessità del modello con la quantità di dati disponibili e le caratteristiche del problema.

**`Diagonal GMMs`** rappresentano una variante in cui si assume che le matrici di covarianza $\Sigma_k$ siano diagonali, ovvero $\Sigma_k = \text{diag}(\sigma_{k,1}^2, \sigma_{k,2}^2, \ldots, \sigma_{k,D}^2)$. Questa assunzione implica **indipendenza condizionale** tra le feature dato il cluster, il che significa che le variabili non sono correlate all'interno di ciascuna componente gaussiana. Il principale vantaggio di questa variante è la drastica riduzione del numero di parametri da stimare, passando da $O(D^2)$ a $O(D)$ parametri per ogni matrice di covarianza. Questo rende il modello computazionalmente più efficiente e numericamente più stabile, particolarmente adatto per dataset ad alta dimensionalità dove la stima di matrici di covarianza complete potrebbe essere problematica. Tuttavia, l'assunzione di indipendenza condizionale può essere troppo restrittiva in molti casi reali, portando a possibile underfitting quando esistono correlazioni significative tra le feature.

**`Tied GMMs`** adottano un approccio diverso, assumendo che tutte le componenti del modello condividano la stessa matrice di covarianza ($\Sigma_k = \Sigma$ per tutti i $k$). In questa configurazione, i cluster mantengono la stessa forma e orientamento nello spazio delle feature, differendo solo per posizione (media) e peso relativo. Questa variante agisce come una forma di regolarizzazione naturale, particolarmente utile quando i dati di training sono scarsi o quando si vuole prevenire l'overfitting. La condivisione della matrice di covarianza riduce significativamente il numero di parametri da stimare e aumenta la stabilità numerica del modello, riducendo il rischio di singolarità nelle matrici. Tuttavia, questa assunzione può essere limitante quando i cluster naturali nei dati hanno forme o orientamenti molto diversi, potenzialmente portando a underfitting in situazioni dove la flessibilità nella forma dei cluster è essenziale per una buona modellazione dei dati.

---

## Domanda di Teoria 7

**Descrivere il modello di `binary logistic regression` per la classificazione, trattando i seguenti aspetti:**

*   **`Classification rule` del modello**
*   **Interpretazione probabilistica del modello e del suo `classification score`**
*   **Stima dei parametri del modello e possibili interpretazioni della `training objective function`**
*   **Come il modello può essere esteso per eseguire una `non-linear classification`**
*   **Come il modello può essere esteso e applicato per affrontare `score calibration issues`**

### Risposta

#### 1. `Classification Rule`

La `binary logistic regression` è un **classificatore discriminativo** che apprende un **`confine decisionale lineare`**. Un nuovo campione $x$ viene classificato in base al segno di uno `score` lineare $s = w^T x + b$. La `classification rule` è:

*   Assegna alla **Classe 1** se $s > 0$.
*   Assegna alla **Classe 0** se $s < 0$.

L'iperpiano definito da $w^T x + b = 0$ separa le due classi nello spazio delle `feature`.

#### 2. Interpretazione Probabilistica

Il modello assume che il `log-odds` (o `log-posterior ratio`) sia lineare nelle `feature`:

$$\log\frac{P(C=1|x)}{P(C=0|x)} = w^T x + b$$

Da questa assunzione, si deriva che la **probabilità a posteriori** della classe positiva è modellata direttamente passando lo `score` lineare $s$ attraverso la **funzione sigmoide**, $\sigma(s) = \frac{1}{1 + e^{-s}}$:
*   $P(C=1|x) = \sigma(w^T x + b)$. Questo fornisce un output probabilistico diretto (un valore tra 0 e 1).
*   Lo `score` $s$ stesso rappresenta il **`log-odds ratio`**, dove valori positivi indicano una maggiore probabilità per la Classe 1.

#### 3. Stima dei Parametri e `Objective Function`

I parametri ($w, b$) sono stimati minimizzando la **`negative log-likelihood`** (nota anche come **`binary cross-entropy loss`**) sui dati di `training`:

$$J(w, b) = - \sum_{i=1}^{N} \left[ y_i \log(\sigma(w^T x_i + b)) + (1-y_i) \log(1-\sigma(w^T x_i + b)) \right]$$

*   **Interpretazione MLE**: Questa `objective function` corrisponde alla **`Maximum Likelihood Estimation (MLE)`**. Trova i parametri che massimizzano la probabilità di osservare le etichette di `training` date le `feature`.
*   **Interpretazione `Risk Minimization`**: La `cross-entropy loss` può essere vista come una `loss function` che penalizza le predizioni errate. Minimizzare la somma di questa `loss` su tutto il `dataset` è un'istanza di **`Empirical Risk Minimization`**.
*   **Ottimizzazione**: La funzione è **`convessa`** ma non ha una soluzione in forma chiusa, quindi viene minimizzata con algoritmi iterativi come la discesa del gradiente.

#### 4. Estensione alla `Non-linear Classification`

La `logistic regression` standard è lineare. Per apprendere `non-linear boundaries`, si utilizza la tecnica dell'**`feature expansion`**. Questo processo comporta la creazione di nuove `feature` applicando trasformazioni non lineari a quelle originali (es. termini polinomiali come $x_1^2, x_1x_2$, o altre funzioni base). Il modello apprende un `linear boundary` in questo spazio espanso ad alta dimensionalità, che corrisponde a un **`non-linear boundary`** nello spazio delle `feature` originale. Questo approccio aumenta la potenza del modello, ma richiede attenzione per evitare l'`overfitting`, solitamente tramite regolarizzazione.

#### 5. Applicazione alla `Score Calibration`

Poiché il suo `training objective` (minimizzare la `cross-entropy`) incoraggia intrinsecamente il modello a produrre output che sono probabilità a posteriori accurate, la `logistic regression` è uno strumento eccellente per la **`score calibration`**. Viene utilizzata per calibrare gli `score` grezzi e non probabilistici di altri modelli (come le `SVM` o i `GMM`). Questo processo, noto come **`Platt Scaling`**, funziona come segue:

1.  Si addestra un `primary model` (es. `SVM`) e si generano i suoi `score` su un `calibration set` (un `dataset` separato, come il `validation set`).
2.  Si addestra un modello di `logistic regression` dove l'unica `feature` di input è lo `score` generato dal `primary model`. La `logistic regression` impara quindi una trasformazione monotona (tipicamente affine, $s_{\text{cal}} = as + b$) che mappa gli `uncalibrated scores` a `well-calibrated posterior probabilities` (o `log-likelihood ratios`).

Questo approccio preserva l'ordine degli `score` originali (la `discriminative power` del modello) ma ne corregge l'interpretazione probabilistica, rendendoli affidabili per il `decision-making` bayesiano.

---

## Domanda di Teoria 8

**Descrivere il modello generativo multinomiale per la classificazione, trattando i seguenti aspetti:**

*   **Il tipo di dati per cui è adatto e le sue assunzioni principali.**
*   **Come vengono stimati i parametri del modello usando la `Maximum Likelihood Estimation (MLE)`.**
*   **L'approssimazione di `Naive Bayes` per gestire attributi multipli.**
*   **Il "`zero probability problem`" e come può essere affrontato.**

### Risposta

#### 1. Adeguatezza dei Dati e Assunzioni

Il modello generativo multinomiale è specificamente progettato per `classification tasks` che coinvolgono **`dati discreti`** o **`categorici`**. È particolarmente comune in applicazioni di **Natural Language Processing (NLP)**, come la classificazione di testi, dove le `features` sono tipicamente i conteggi di parole da un vocabolario predefinito (il cosiddetto modello "`Bag-of-Words`").

*   **Assunzione Principale**: Il modello assume che i campioni di `training` siano **`independent and identically distributed (i.i.d.)`**. Questo significa che ogni documento (o campione) è considerato un'estrazione indipendente dalla stessa `underlying probability distribution`, e l'ordine delle parole al suo interno viene ignorato.

#### 2. Stima dei Parametri (`MLE`)

I parametri del modello sono le probabilità condizionali $\pi_{c,j} = P(X_{\text{word}}=j | C=c)$, che rappresentano la probabilità che una parola scelta a caso da un documento della classe $c$ sia la parola $j$ del vocabolario. Questi parametri vengono stimati utilizzando la **`Maximum Likelihood Estimation (MLE)`**.

*   **Obiettivo**: L'obiettivo è trovare il set di parametri $\pi_c$ per ogni classe $c$ che massimizza la `log-likelihood` dei dati di `training` osservati.
*   **Soluzione**: La stima `MLE` per $\pi_{c,j}$ è la **`frequenza relativa`** della parola $j$ all'interno di tutti i documenti della classe $c$:
    $$\pi_{c,j}^{\text{ML}} = \frac{N_{c,j}}{N_c} = \frac{\text{Conteggio totale della parola } j \text{ nella classe } c}{\text{Conteggio totale di tutte le parole nella classe } c}$$
    dove $N_{c,j}$ è il conteggio della parola $j$ nella classe $c$, e $N_c$ è il numero totale di parole (token) in tutti i documenti della classe $c$.

#### 3. `Naive Bayes` per Attributi Multipli

Quando si ha a che fare con più `features` discrete (come nel caso di un vocabolario di parole), modellare la loro probabilità congiunta è computazionalmente intrattabile a causa della **`curse of dimensionality`**. L'approssimazione di **`Naive Bayes`** risolve questo problema introducendo una forte (ma efficace) assunzione di indipendenza:

*   **Assunzione**: Tutte le `features` (parole) sono **`condizionatamente indipendenti`** data la classe.
*   **Semplificazione**: Questa assunzione permette di fattorizzare la `joint conditional probability` di un intero documento in un prodotto delle probabilità delle singole parole:

$$P(x | C=c) \approx \prod_{j=1}^{D} P(x_j | C=c)$$

Questo rende il modello computazionalmente efficiente e robusto contro la `data sparsity`, poiché la probabilità di ogni parola può essere stimata in modo indipendente.

#### 4. Il "`zero probability problem`"

Un problema critico, noto come **`zero-frequency problem`**, sorge se una parola presente in un documento di test non è mai stata osservata nei dati di `training` per una particolare classe. In questo caso, la sua probabilità `MLE` sarebbe 0. A causa della natura moltiplicativa del calcolo della `likelihood` (specialmente in `Naive Bayes`), questo forzerebbe l'intera `class likelihood` a diventare 0, rendendo impossibile la classificazione per quella classe.

*   **Soluzione**: Questo problema viene risolto tramite lo **`smoothing`**. La tecnica più comune è l'**`additive smoothing`** (o `Laplace smoothing`), che consiste nell'aggiungere un piccolo **`pseudo-conteggio`** $\alpha > 0$ (tipicamente $\alpha=1$) a ogni conteggio di parola:

$$\pi_{c,j}^{\text{smoothed}} = \frac{N_{c,j} + \alpha}{N_c + m \alpha}$$

dove $m$ è la dimensione del vocabolario. Questo garantisce che nessuna probabilità sia mai esattamente zero. L'applicazione di `pseudo-counts` è matematicamente equivalente a eseguire una stima a **`Maximum A Posteriori (MAP)`** con una `Dirichlet prior` sui parametri, che formalizza l'idea di incorporare una credenza a priori che tutte le parole abbiano una probabilità non nulla di apparire.

---

## Domanda di Teoria 9

**Discutere i principi delle decisioni bayesiane e della valutazione robusta dei modelli, trattando:**

*   **Le limitazioni dell'accuratezza come `evaluation metric`.**
*   **Il ruolo della `confusion matrix` e delle `prior-independent metrics (FPR, FNR)`.**
*   **Il concetto di `Bayes Risk` e la `Detection Cost Function (DCF)` per problemi binari.**
*   **Come visualizzare le `classifier performance` attraverso tutte le `decision thresholds`.**

### Risposta

#### 1. Limitazioni dell'Accuratezza

Mentre è intuitiva, l'accuratezza (la proporzione di predizioni corrette) è spesso una metrica di valutazione **inadeguata e fuorviante** perché:
*   È altamente sensibile allo **`squilibrio delle classi`** (`class imbalance`). Un classificatore può raggiungere un'`high accuracy` su un `imbalanced dataset` semplicemente predicendo sempre la `majority class`, anche se non possiede alcun `real discriminative power`.
*   Tratta tutti gli errori come **ugualmente costosi**, il che è raramente vero nelle applicazioni del mondo reale (es. un `false negative` in una diagnosi medica è molto più grave di un `false positive`).
*   Dipende dalle **`prior`** del `evaluation set`, che potrebbero non corrispondere alle `prior` della `target application`, rendendola una stima inaffidabile delle `future performance`.

#### 2. `Confusion Matrix` e `Prior-Independent Metrics`

La **`confusion matrix`** fornisce un `detailed breakdown` delle `performance` di un classificatore, tabulando i `True Positives (TP)`, `True Negatives (TN)`, `False Positives (FP)` e `False Negatives (FN)`. Da questa matrice, possiamo derivare metriche **robuste e indipendenti dai `prior`**:
*   **`False Positive Rate (FPR)`**: La proporzione di negativi effettivi erroneamente classificati come positivi. 

$$FPR = \frac{FP}{FP + TN}$$

*   **`False Negative Rate (FNR)`**: La proporzione di positivi effettivi erroneamente classificati come negativi. 

$$FNR = \frac{FN}{FN + TP}$$

Questi tassi sono intrinseci alle `performance` del classificatore e non sono influenzati dalla distribuzione delle classi nel `evaluation set`, permettendo una stima più affidabile delle `performance` future.

#### 3. `Bayes Risk` e `DCF`

Le decisioni ottimali dovrebbero mirare a minimizzare il **`Bayes Risk`**, che è il **costo atteso minimo** sull'intera `application population`, tenendo conto sia dei `priors` di applicazione che dei costi degli errori. Per un problema binario, questo è quantificato dalla **`Detection Cost Function (DCF)`**.
*   **`Un-normalized DCF`**: 

$$DCF_u(\pi_T, C_{FN}, C_{FP}) = \pi_T C_{FN} P_{fn} + (1 - \pi_T) C_{FP} P_{fp}$$

dove $\pi_T$ è il `prior` della classe positiva, e $C_{FN}, C_{FP}$ sono i costi dei `false negatives` e `false positives`.
*   **`Normalized DCF`**: La `DCF` viene normalizzata rispetto al costo di un sistema "`dummy`" che predice sempre l'`outcome` meno costoso. Una $\text{DCF} < 1$ indica che il classificatore è migliore del `dummy system`, mentre $\text{DCF} = 0$ rappresenta una classificazione perfetta.

#### 4. Visualizzazione delle `Performance`

Per valutare le `performance` di un classificatore su **tutte le possibili soglie di decisione**, si utilizzano grafici specifici:
*   **`Receiver Operating Characteristic (ROC) Curve`**: Mostra il `True Positive Rate` $(TPR = 1 - FNR)$ contro il `False Positive Rate (FPR)`. Il punto ideale è l'angolo in alto a sinistra $(TPR = 1, FPR = 0)$. L'**`Area Under the Curve (AUC)`** riassume la `discriminative power` complessiva del classificatore.
*   **`Detection Error Trade-off (DET) Curve`**: Mostra il `FNR` contro il `FPR` su una **scala di probabilità** (es. `normal deviate`). Questa scala espande le regioni a basso errore, rendendola più efficace della `ROC` per visualizzare le differenze di `performance` tra classificatori ad alta accuratezza. Il punto ideale è l'angolo in basso a sinistra.

---

## Domanda di Teoria 10

**Spiegare i concetti di `score calibration` e `fusion`, trattando:**

*   **Il problema dei `mis-calibrated scores` e il suo impatto sul `decision-making`.**
*   **La differenza tra `minimum DCF` e `actual DCF`.**
*   **Metodi comuni per lo `score calibration`.**
*   **La motivazione e un approccio comune per lo `score-level fusion`.**

### Risposta

#### 1. `Mis-calibrated Scores`

Molti classificatori (in particolare le `SVM`, ma anche i `GMM` o altri modelli le cui assunzioni non corrispondono perfettamente ai dati) producono `raw scores` che **non sono ben calibrati**. Questo significa che gli `score` non rappresentano accuratamente vere **probabilità a posteriori** o **`Log-Likelihood Ratios (LLR)`**. Applicare la soglia decisionale teorica di Bayes ($t = -\log(\frac{\pi_{eff}}{1-\pi_{eff}})$) a questi `score` non calibrati è **subottimale**, perché la soglia si basa su assunzioni probabilistiche che gli `score` non soddisfano. Ciò porta a un costo (`Bayes Risk`) più alto del necessario per l'applicazione.

#### 2. `Minimum DCF` vs. `Actual DCF`

L'impatto della `mis-calibration` si misura confrontando due valori della `Detection Cost Function (DCF)`:
*   **`Minimum DCF (minDCF)`**: È il **costo più basso possibile** che il classificatore può raggiungere per una data applicazione. Viene trovato cercando empiricamente la **soglia ottimale** sugli `score` del `validation set`. Misura la `potenza discriminativa intrinseca` del classificatore, assumendo una calibrazione perfetta.
*   **`Actual DCF (actDCF)`**: È il `DCF` ottenuto applicando la **soglia teorica di Bayes** (derivata dai `priors` e dai costi dell'applicazione) direttamente ai `raw scores`.

Il `gap`, **`actDCF - minDCF`**, è la **`calibration loss`**. Un `gap` ampio indica una cattiva calibrazione e una perdita di `performance` dovuta a decisioni non ottimali.

#### 3. `Score Calibration Methods`

L'obiettivo della calibrazione è apprendere una **funzione monotona** $f$ che trasforma i `raw scores` $s$ in `scores` calibrati $s_{\text{cal}} = f(s)$ che siano `well-calibrated LLRs`. Ciò consente di applicare efficacemente la soglia teorica di Bayes. I metodi comuni includono:
*   **`Prior-Weighted Logistic Regression (Platt Scaling)`**: Un metodo parametrico che apprende una trasformazione affine $s_{\text{cal}} = a \cdot s + b$. È semplice e robusto, ma assume una relazione lineare nello spazio dei `log-odds`.
*   **`Isotonic Regression`**: Un metodo non parametrico che trova la migliore trasformazione monotona non lineare. È più flessibile ma può facilmente sovradattarsi (`overfitting`) con `calibration set` di piccole dimensioni.

#### 4. `Score-Level Fusion`

**Motivazione**: Classificatori diversi (es. un `GMM` e una `SVM`) spesso catturano informazioni complementari dai dati. Combinando i loro `output scores`, un sistema fuso può ottenere `performance` superiori a quelle di qualsiasi singolo classificatore.

**`Weighted Fusion`**: Un approccio comune è calcolare una combinazione lineare pesata degli `score` provenienti da sistemi multipli. Questo viene tipicamente implementato addestrando un modello di **`logistic regression`**:

1.  Gli `score` $s_1, s_2, ..., s_K$ dei singoli classificatori formano un `feature vector` $s = [s_1, s_2, ..., s_K]$.
2.  Un modello di `logistic regression` viene addestrato su un `calibration set` per apprendere i pesi ottimali ($\alpha$) e il `bias` ($\gamma$) per combinare gli `score`: $s_{\text{fused}} = \alpha^T s + \gamma$.

Questo approccio non solo **combina** gli `score` in modo ottimale, ma li **calibra** anche simultaneamente, producendo uno `score` fuso che è un `well-calibrated LLR`.

---

## Domanda di Teoria 11

**Descrivere l'approccio di riduzione della dimensionalità `Principal Component Analysis (PCA)`, concentrandosi su:**

*   **L'obiettivo del modello e la sua formulazione**
*   **La funzione obiettivo di `training`**
*   **La relazione tra la distribuzione dei dati e la soluzione `PCA`**
*   **Considerazioni pratiche nell'impiego della `PCA`**

### Risposta

#### 1. Obiettivo e Formulazione

L'obiettivo della **`Principal Component Analysis (PCA)`** è quello di trasformare i dati da uno spazio delle `feature` ad alta dimensionalità a un nuovo spazio a dimensionalità inferiore, cercando di preservare la maggior quantità di **informazione** possibile. Questa informazione viene quantificata come la **`varianza`** dei dati.

La formulazione del modello si basa sulla ricerca di una matrice di proiezione $P$ (di dimensione $n \times m$, dove $n$ è la dimensionalità originale e $m$ quella ridotta), le cui colonne sono vettori di base ortonormali. La proiezione di un campione $x$ nel nuovo spazio è data da $y = P^T (x - \mu)$, dove $\mu$ è la media dei dati, mentre la sua ricostruzione approssimata nello spazio originale è $\hat{x} = P y + \mu$.

#### 2. Funzione Obiettivo di `Training`

La funzione obiettivo che la `PCA` ottimizza è la **`minimizzazione dell'errore quadratico medio di ricostruzione`**:

$$\min_P \frac{1}{K} \sum_{i=1}^K ||x_i - \hat{x}_i||^2$$

dove $K$ è il numero di campioni. Questo criterio assicura che la distanza tra i punti originali e le loro ricostruzioni sia la più piccola possibile. Minimizzare l'errore di ricostruzione è matematicamente **`equivalente a massimizzare la varianza`** dei dati proiettati $y$. In altre parole, la `PCA` trova le direzioni lungo le quali i dati si disperdono maggiormente.

#### 3. Relazione tra Distribuzione dei Dati e Soluzione `PCA`

La soluzione della `PCA` è intrinsecamente legata alla distribuzione dei dati, descritta dalla loro **matrice di covarianza** $C = \frac{1}{K} \sum_i (x_i - \mu)(x_i - \mu)^T$. Le direzioni ottimali che formano la matrice di proiezione $P$ sono gli **`autovettori`** della matrice di covarianza $C$ associati agli $m$ **`autovalori`** più grandi.

*   Gli **`autovettori`** (le `componenti principali`) rappresentano le direzioni di massima varianza nello spazio dei dati.
*   Gli **`autovalori`** corrispondenti quantificano la quantità di varianza catturata da ciascuna `componente principale`.

In sostanza, la `PCA` allinea il nuovo sistema di coordinate con gli assi principali dell'ellissoide di covarianza dei dati.

#### 4. Considerazioni Pratiche

*   **`Centratura dei Dati`**: È un passaggio **cruciale**. I dati devono essere **`centrati rispetto alla media`** $(x_i - \mu)$ prima di applicare la `PCA`. Se i dati non sono centrati, la prima `componente principale` catturerà semplicemente la posizione media dei dati nello spazio, anziché la direzione di massima varianza.
*   **Selezione di `m` (Numero di Componenti)**: La scelta della dimensionalità `target m` può essere guidata da due approcci comuni:
    1.  **`Percentuale di Varianza Conservata`**: Si sceglie il `m` più piccolo tale che la somma degli `m` autovalori più grandi sia una frazione significativa (es. 95%) della varianza totale (la somma di tutti gli autovalori).
    2.  **`Cross-Validation`**: Se la `PCA` è un passo di `pre-processing` per un classificatore, si può trattare `m` come un iperparametro da ottimizzare su un `validation set` per massimizzare le `performance` del classificatore.
*   **`Complessità Computazionale`**: Per dati con un numero di `feature` molto elevato ($n > K$), calcolare la matrice di covarianza `n x n` può essere proibitivo. In questi casi, si possono usare metodi alternativi come la **`Truncated SVD`**, che calcola solo le `componenti principali` più importanti in modo più efficiente.

---

## Domanda di Teoria 12

**Descrivere l'approccio di riduzione della dimensionalità `Linear Discriminant Analysis (LDA)`, concentrandosi su:**

*   **L'obiettivo del modello e la sua formulazione**
*   **La funzione obiettivo di `training`**
*   **La relazione tra la distribuzione dei dati e la soluzione `LDA`**
*   **Considerazioni pratiche nell'impiego della `LDA`**

### Risposta

#### 1. Obiettivo e Formulazione

L'obiettivo della **`Linear Discriminant Analysis (LDA)`** è trovare un sottospazio a dimensionalità inferiore che **massimizzi la separabilità tra le classi**. A differenza della `PCA`, la `LDA` è un metodo **`supervised`** che utilizza le etichette di classe per trovare le direzioni che meglio discriminano i dati.

La formulazione del modello si basa sulla ricerca di una matrice di proiezione $W$ che trasforma i dati originali $x$ in uno spazio a dimensionalità inferiore $y = W^T x$. Questa proiezione è ottimizzata per massimizzare il rapporto tra la `between-class scatter` (la varianza delle medie di classe) e la `within-class scatter` (la varianza all'interno di ciascuna classe).

#### 2. Funzione Obiettivo di `Training`

La funzione obiettivo che la `LDA` ottimizza è il **`criterio di Fisher`**, che è il rapporto tra la `between-class scatter matrix` ($S_B$) e la `within-class scatter matrix` ($S_W$) nello spazio proiettato:

$$ J(W) = \frac{\det(W^T S_B W)}{\det(W^T S_W W)} $$

La massimizzazione di questo rapporto porta a un **problema agli autovalori generalizzato**: 

$$ S_B w = \lambda S_W w $$

Le colonne della matrice di proiezione ottimale $W$ sono gli autovettori di $S_W^{-1} S_B$ corrispondenti agli autovalori più grandi.

#### 3. Relazione tra Distribuzione dei Dati e Soluzione `LDA`

La soluzione della `LDA` dipende direttamente dalla distribuzione dei dati all'interno di ogni classe e tra le classi, catturata dalle matrici di `scatter`:

*   **`Within-class scatter matrix` ($S_W$)**: È la somma delle matrici di covarianza di ciascuna classe. Misura quanto sono sparsi i dati all'interno di ciascuna classe.
    
    $$S_W = \sum_{c=1}^{C} \sum_{i: x_i \in c} (x_i - \mu_c)(x_i - \mu_c)^T$$
    
    dove $\mu_c$ è la media della classe $c$.

*   **`Between-class scatter matrix` ($S_B$)**: Misura la separazione tra le medie delle diverse classi rispetto alla media globale dei dati.
    
    $$S_B = \sum_{c=1}^{C} N_c (\mu_c - \mu)(\mu_c - \mu)^T$$
    
    dove $\mu$ è la media globale di tutti i dati e $N_c$ è il numero di campioni della classe $c$.

La `LDA` trova le direzioni che rendono i cluster di classe il più compatti possibile (minimizzando $S_W$) e allo stesso tempo il più distanti possibile l'uno dall'altro (massimizzando $S_B$). Il numero massimo di direzioni discriminanti che la `LDA` può trovare è $C-1$, dove $C$ è il numero di classi.

#### 4. Considerazioni Pratiche

*   **`Assunzioni sui Dati`**: La `LDA` assume implicitamente che i dati di ogni classe seguano una distribuzione approssimativamente Gaussiana con strutture di covarianza simili. Funziona al meglio quando le classi sono ben separate e hanno una forma simile a un'ellissoide.
*   **`Singolarità di Sw`**: La matrice `within-class scatter` $S_W$ può diventare singolare (non invertibile) se il numero di `feature` è molto più grande del numero di campioni. Questo è un problema comune nei `dataset` ad alta dimensionalità.
*   **`Pipeline PCA+LDA`**: Una soluzione comune al problema della singolarità è applicare prima la `PCA` per ridurre la dimensionality e de-correlare le `feature`. Successivamente, la `LDA` viene applicata sui dati trasformati dalla `PCA` per trovare le direzioni più discriminanti. Questa `pipeline` `PCA+LDA` è molto potente e robusta.
*   **`Relazione con il Classificatore Gaussiano`**: La `LDA` è strettamente correlata al classificatore `Tied Covariance Gaussian`. Entrambi i modelli portano a `decision boundaries` lineari, poiché l'assunzione di una covarianza condivisa nel modello Gaussiano è matematicamente equivalente all'obiettivo di massimizzazione del rapporto di `scatter` della `LDA`.

---

## Risposte alle Domande sul Progetto

### Progetto - `question example 1`

**Spiegare, alla luce delle caratteristiche dei classificatori e delle caratteristiche dei `dataset` del progetto:**

**1. Le `relative performance` dei modelli `MVG`, `Tied MVG` e `GMM`.**
**2. Le `relative performance` delle `linear` e `non-linear SVM`.**

### Risposta

#### 1. `Generative Models` (`MVG`, `Tied MVG`, `GMM`):

*   **`MVG` vs. `Tied MVG`:** Il classificatore standard **`Multivariate Gaussian (MVG)`**, che modella ogni classe con una media e una `full covariance matrix` separate, ha costantemente superato il modello **`Tied MVG`**. L'assunzione principale del modello `Tied MVG` è che entrambe le classi condividano una `single covariance matrix`. Le sue più scarse `performance` suggeriscono che questa assunzione non è valida per il nostro `dataset`; le due classi hanno probabilmente varianze e `feature correlations` differenti, e la flessibilità del modello `full MVG` nel catturare queste `distinct distributions` è cruciale per migliori `performance`.
*   **`GMM` vs. `MVG`:** Il **`Gaussian Mixture Model (GMM)`**, in particolare con un `optimized number of components` e `diagonal covariance`, ha superato significativamente il `single-component MVG model`. Ciò indica che l'`underlying data distribution` per almeno una delle classi non è una semplice `unimodal Gaussian`. La capacità del `GMM` di modellare più complesse, `multi-modal distributions` gli ha permesso di catturare la `data's structure` in modo più accurato, portando a `classification performances` superiori.

#### 2. `SVM Models` (`Linear` vs. `Non-linear`):

*   **`Linear` vs. `Non-linear SVM`:** La **`linear SVM`** ha fornito una `reasonable baseline`, ma è stata significativamente superata dalle `SVM` con `non-linear kernels`. Questa è una forte evidenza che il `dataset` **`non è linearly separable`** e che è necessario un `decision boundary` più complesso.
*   **`Polynomial` vs. `RBF SVM`:** Tra i `non-linear kernels`, la **`RBF SVM`** ha ottenuto le `best performance`, superando la `polynomial kernel SVM`. Sebbene il `polynomial kernel` abbia confermato il `benefit` di andare oltre un `linear model`, la maggiore flessibilità del `RBF kernel` nel creare altamente complessi, `localized decision boundaries` si è rivelata la `best match` per la struttura intricata del `project dataset`.

---

### Progetto - `question example 2`

**Spiegare le `relative performance` sul `project validation set` dei diversi `SVM kernels` (inclusi i modelli lineari), alla luce delle caratteristiche del `kernel` e delle caratteristiche del `dataset`. Analizzare brevemente gli effetti della `regularization` sulle `model performance`.**

### Risposta

Le `performance` dei diversi `SVM kernels` sul `project dataset` rivelano una chiara gerarchia, che è direttamente legata alla complessità del `decision boundary`.

*   **`Linear SVM`:** Questo modello apprende un semplice iperpiano come `decision boundary`. Le sue `performance` sono state le più basse tra le `SVM` testate, il che indica fortemente che le due classi nel nostro `dataset` **`non sono linearly separable`**. Serve come un'utile `baseline` ma è insufficiente per questo `task`.
*   **`Polynomial SVM`:** Utilizzando un `polynomial kernel`, la `SVM` può apprendere un `non-linear`, `curved decision boundary`. Questo modello ha mostrato un `significant improvement` rispetto alla `linear SVM`, confermando che un approccio `non-linear` è necessario.
*   **`RBF SVM`:** L'`RBF kernel` è il più flessibile, capace di creare `complex`, `non-linear boundaries` di forma arbitraria. Questo modello ha costantemente ottenuto le **`best performance`** di tutte le `SVM`. La sua superiorità implica che l'`optimal decision boundary` è altamente complesso e non ben approssimato da forme più semplici come linee o parabole.

**Effetti della `Regularization`:**
La `Regularization`, controllata dall'iperparametro $C$, gestisce il `trade-off` tra la massimizzazione del `margin` e la minimizzazione degli errori di classificazione sui dati di `training`.
*   **$C$ molto basso** (`strong regularization`): Causa `underfitting` poiché il modello è eccessivamente vincolato e non riesce a catturare i `pattern` nei dati.
*   **$C$ molto alto** (`weak regularization`): Rischia `overfitting` sui `training data`, anche se spesso migliora la `score calibration` permettendo al modello di adattarsi meglio.
*   **$C$ intermedio**: Le `performance` ottimali si ottengono con valori intermedi di $C$, che bilanciano efficacemente il `fitting` dei `training data` con il mantenimento di un `margin` ampio per una buona generalizzazione.

---

### Progetto - `question example 3`

**Considerare i classificatori `SVM` e `logistic regression`. Alla luce delle caratteristiche dei `datasets` e dei classificatori, spiegare il `gap` tra `minimum` e `actual DCF` per ciascun modello e, se necessario, il metodo che avete impiegato per ridurre questo `gap` per il `project dataset`. Analizzare anche gli effetti della `regularization` sull'`miscalibration error` per entrambi i modelli.**

### Risposta

Il `gap` tra `minimum DCF` e `actual DCF` è un indicatore diretto della **`qualità della calibrazione degli score`**.

*   **`Logistic Regression`:** Questo modello produce intrinsecamente `score` ben calibrati. Il suo `obiettivo di training` (minimizzare la `negative log-likelihood`) incoraggia direttamente gli output del modello a essere accurate probabilità a posteriori. Di conseguenza, il `gap` tra `minDCF` e `actDCF` era consistentemente piccolo.

*   **`SVM`:** Al contrario, l'obiettivo della `SVM` è massimizzare il `margin`, non produrre probabilità. I suoi `score` rappresentano la distanza con segno dall'iperpiano di decisione. Questi `score` sono generalmente **`mal calibrati`**, risultando in un **`gap ampio`** tra `minDCF` e `actDCF`. Applicare la soglia teorica a questi `score` grezzi porta a decisioni subottimali.

**Metodo per Ridurre il `Gap` (Calibrazione):**
Per affrontare la cattiva calibrazione della `SVM`, abbiamo addestrato un `prior-weighted logistic regression` sul `validation set`, utilizzando i `raw score` della `SVM` come input. Questa tecnica, chiamata `Platt Scaling`, apprende una funzione monotona semplice che mappa gli `score` non calibrati della `SVM` a `log-likelihood ratios` ben calibrati. Dopo questo passaggio, l'`actDCF` della `SVM` calibrata è diventato molto più vicino al suo `minDCF`, riducendo significativamente l'`errore di calibrazione` e rendendo gli output del classificatore affidabili per il `decision-making`.

**Effetti della `Regularization` sulla `Miscalibrazione`:**
*   Per la **`SVM`**, la `regularization` (parametro $C$) ha un effetto notevole. Abbiamo osservato che all'aumentare di $C$, il modello cercava di classificare correttamente i `training point`, e il suo `actDCF` tendeva ad avvicinarsi al suo `minDCF`, migliorando la calibrazione.
*   Per la **`Logistic Regression`**, la `regularization` (parametro $\lambda$) serve principalmente a prevenire l'`overfitting`. Mentre valori estremi potevano danneggiare le `performance`, il suo effetto sulla calibrazione già buona era meno pronunciato rispetto alla `SVM`.

---

### Progetto - `question example 4`

**Given the following functions (assume these functions are already implemented unless specified):**

*   `trainPCA`: trains a `PCA` model
*   `applyPCA`: applies a `PCA` model to some data
*   `trainClassifier(D, L)`: trains a given classifier from the `data matrix D` and the `label vector L`; returns an object containing the trained `model parameters`
*   `scoreClassifier(clsModel, D)`: computes the `array of scores` for classifier `clsModel` (as returned by the function `trainClassifier`) for the `samples` in `data matrix D`
*   `evaluateScores(S, L)`: computes a `performance metric` (e.g. `minimum DCF`) over the `score array S` with `label vector L`

**a) Provide a possible `signature` and an `implementation` of the functions `trainPCA` and `applyPCA`, briefly explaining also the function parameters and the return value.**
**b) Using these functions, write the `Python code` to:**
*   **Train the classifier on a `training set`, optimizing the `PCA dimension` with respect to the provided `metric function` using a `single-fold cross-validation approach`**
*   **Evaluate its `performance` on an `evaluation set`.**

Assume that you have at your disposal a `training set`, already divided in `model training data (DTR, LTR)` and `validation data (DVAL, LVAL)`, and an `evaluation set (DTE, LTE)`. `DTR`, `DVAL` and `DTE` are `data matrices`, with `samples` organized as `column vectors`, whereas `LTR`, `LVAL` and `LTE` are `arrays` containing the corresponding `labels`. To select the `PCA dimension m` consider all possible values of `m` that are compatible with the dimension of the `feature vectors`. Assume that the classifier is `invariant to affine transformations`, that it does not include `hyper-parameters` to tune, and that `PCA` is the `only kind of pre-processing` to analyze.

### Risposta

#### a) `Signature` e `Implementation` di `trainPCA` e `applyPCA`

```python
import numpy

def trainPCA(D, m):
    """
    Trains a PCA model by finding the top m principal components.
    
    Args:
        D (numpy.ndarray): The training data matrix, with samples as columns.
        m (int): The number of principal components to retain.
        
    Returns:
        tuple: A tuple containing:
            - P (numpy.ndarray): The projection matrix (m eigenvectors).
            - mu (numpy.ndarray): The mean vector of the training data.
    """
    mu = D.mean(1).reshape((D.shape[0], 1))
    DC = D - mu
    C = numpy.dot(DC, DC.T) / D.shape[1]
    s, U = numpy.linalg.eigh(C)
    P = U[:, ::-1][:, 0:m]
    return P, mu

def applyPCA(P, mu, D):
    """
    Applies a trained PCA model to project data.
    
    Args:
        P (numpy.ndarray): The projection matrix from trainPCA.
        mu (numpy.ndarray): The mean vector from the training data.
        D (numpy.ndarray): The data matrix to project, with samples as columns.
        
    Returns:
        numpy.ndarray: The projected data matrix.
    """
    DC = D - mu
    DP = numpy.dot(P.T, DC)
    return DP
```

#### b) `Code` per `PCA Optimization` e `Evaluation`

```python
best_m = -1
best_metric_val = float('inf')  # Assuming lower is better (e.g., minDCF)
best_P = None
best_mu = None

# --- Hyper-parameter (m) selection using validation set ---
for m in range(1, DTR.shape[0] + 1):
    # Train PCA on the training set
    P, mu = trainPCA(DTR, m)
    
    # Project both training and validation sets
    DTR_p = applyPCA(P, mu, DTR)
    DVAL_p = applyPCA(P, mu, DVAL)
    
    # Train classifier on projected training data
    clsModel = trainClassifier(DTR_p, LTR)
    
    # Score on projected validation data
    S_val = scoreClassifier(clsModel, DVAL_p)
    
    # Evaluate performance
    current_metric = evaluateScores(S_val, LVAL)
    
    # Check if this m is the best so far
    if current_metric < best_metric_val:
        best_metric_val = current_metric
        best_m = m
        best_P = P
        best_mu = mu

print(f"Optimal PCA dimension is m={best_m} with validation metric={best_metric_val}")

# --- Final model training and evaluation ---
# Train a new classifier on the full training data (DTR) projected with the optimal PCA
DTR_p_best = applyPCA(best_P, best_mu, DTR)
final_model = trainClassifier(DTR_p_best, LTR)

# Project the evaluation data (DTE) with the optimal PCA model
DTE_p_best = applyPCA(best_P, best_mu, DTE)

# Score the final model on the projected evaluation data
S_test = scoreClassifier(final_model, DTE_p_best)

# Compute final performance on the evaluation set
final_metric = evaluateScores(S_test, LTE)
print(f"Performance on evaluation set: {final_metric}")
```

---

### Progetto - `question example 5`

**You are given the following functions (assume these functions are already implemented unless specified):**

*   `trainRBFKernelSVM(D, L, C, gamma)`: trains an `SVM model` with an `RBF kernel` with `hyper-parameter gamma` and returns an object containing the trained `model information`; `D` is the `training data matrix`, `L` is the corresponding `label array`, and `C` is the `SVM cost-vs-margin trade-off coefficient`
*   `scoreRBFKernelSVM(svmModel, D)`: computes the `classification scores` for `samples` in the `data matrix D` for an `SVM model svmModel` (as returned by the function `trainRBFKernelSVM`) and returns an `array of scores`
*   `evaluateScores(S, L)`: computes an `evaluation metric` (e.g. `minimum DCF`) over the `array of scores S` with associated `array of labels L`

**Write the `Python code` to train and apply an `SVM classifier`. In particular, the `code` should**

*   **Train an `SVM classifier`, optimizing the `value` of the `hyper-parameters` with respect to the `metric function evaluateScores` using a `single-fold cross-validation approach`.**
*   **Evaluate the selected `SVM model` on the `evaluation data`, using the provided `metric`.**

Write an `implementation` of `scoreRBFKernelSVM(svmModel, D)`. Assume that `svmModel` is an object with the following fields: `sv`, `alpha`, `labels`, `gamma`. You can assume that you have at your disposal a function `RBFKernel(D1, D2, gamma)` that returns the `matrix of kernel values k(x, y)` for all pairs of `samples x, y` of `2-D sample matrices D1, D2`.

### Risposta

#### `Code` per `SVM Optimization` e `Evaluation`

```python
C_values = [1e-3, 1e-2, 1e-1, 1.0]
gamma_values = [1e-3, 1e-2, 1e-1]

best_C = None
best_gamma = None
best_metric_val = float('inf') # Assuming lower metric is better (e.g., minDCF)

# --- Hyper-parameter tuning using validation set ---
for C in C_values:
    for gamma in gamma_values:
        # Train the SVM model on the training data
        svmModel = trainRBFKernelSVM(DTR, LTR, C, gamma)
        
        # Compute scores on the validation data
        S_val = scoreRBFKernelSVM(svmModel, DVAL)
        
        # Evaluate the performance
        current_metric = evaluateScores(S_val, LVAL)
        
        # Update the best hyperparameters if performance improved
        if current_metric < best_metric_val:
            best_metric_val = current_metric
            best_C = C
            best_gamma = gamma

# --- Final model evaluation on test set ---
final_svm_model = trainRBFKernelSVM(DTR, LTR, best_C, best_gamma)
S_test = scoreRBFKernelSVM(final_svm_model, DTE)
final_performance = evaluateScores(S_test, LTE)
```

#### `Implementation` di `scoreRBFKernelSVM`

```python
import numpy

def scoreRBFKernelSVM(svmModel, D):
    """
    Computes classification scores for an RBF SVM model.
    The score is calculated as f(x) = sum(alpha_i * y_i * K(sv_i, x)).
    
    Args:
        svmModel (object): An SVM model with fields: sv, alpha, labels, gamma.
        D (numpy.ndarray): The data matrix to score (features x samples).
    
    Returns:
        numpy.ndarray: A 1-D array of scores.
    """
    # Create the kernel matrix between support vectors (from training) and new data D
    # sv shape: (num_features, num_sv)
    # D shape: (num_features, num_test_samples)
    # kernel_matrix shape: (num_sv, num_test_samples)
    kernel_matrix = RBFKernel(svmModel.sv, D, svmModel.gamma)
    
    # The dual solution weights are alpha_i * y_i for each support vector
    # alpha shape: (num_sv,)
    # labels shape: (num_sv,)
    # weighted_alphas shape: (num_sv,)
    weighted_alphas = svmModel.alpha * svmModel.labels
    
    # Compute the final scores by summing the weighted kernel values for each test sample
    # dot product of (1, num_sv) with (num_sv, num_test_samples) -> (1, num_test_samples)
    scores = numpy.dot(weighted_alphas, kernel_matrix)
    
    return scores.flatten()
```

---

### Progetto - `question example 6`

**Consider a `binary classification problem`, with classes labeled as 1 and 0, respectively. Let `(DTR, LTR)`, `(DVAL, LVAL)` represent a `labeled training set` and a `labeled validation set`. Let also `DTE` represent the `dataset matrix` containing the `samples` that our `application` should `classify`. Write a `Python code fragment` that:**

1.  **trains a `calibrated binary classifier`**
2.  **performs `inference` (i.e. computes predicted `labels`) on the `evaluation data`**

**You can assume that the following functions have been defined:**

*   `trainClassifier(D, L)`: train a `non-calibrated classification model` (e.g., an `SVM` or an `LDA classifier`) on the `training matrix D` with associated `labels array L`, and return a `python object` containing the trained `model`
*   `scoreClassifier(model, D)`: compute the `non-calibrated classification scores` for `model model` for the `samples` in `data matrix D` and return a `1-D array of scores`
*   `trainCalibrationModel(S, L, prior)`: train a `calibration model` on the `1-D array of scores S`, with associated `array of labels L`, for a `binary application` with `prior prior` for `class 1`, and return a `python object` containing the trained `model`
*   `applyCalibrationModel(calModel, S)`: apply the `calibration model calModel` to the `1-D array of scores S`, and return a `1-D array of calibrated scores`

NOTE: assume that the `target application` is characterized by an `effective prior p` for `class 1`. You are not required to tune the `calibration model hyper-parameter prior`, but you can assume that the `calibration model` can be trained using the `target application prior p`.

### Risposta

```python
import numpy

# Assume DTR, LTR, DVAL, LVAL, DTE are pre-defined.
# Assume p (prior for class 1) is a known float.

# --- 1. Train the calibrated binary classifier ---

# First, train the base (non-calibrated) classifier on the training set.
# The validation set (DVAL, LVAL) is kept separate for calibration.
base_model = trainClassifier(DTR, LTR)

# Second, generate scores on the validation set using the base model.
scores_for_calibration = scoreClassifier(base_model, DVAL)

# Third, train the calibration model. It learns to map the raw scores
# to calibrated log-likelihood ratios, using the validation scores,
# their true labels, and the application prior.
calibration_model = trainCalibrationModel(scores_for_calibration, LVAL, p)

# The "calibrated classifier" is the combination of base_model and calibration_model.

# --- 2. Perform inference on the evaluation data ---

# First, get the raw scores for the evaluation data (DTE) from the base model.
raw_scores_test = scoreClassifier(base_model, DTE)

# Second, apply the trained calibration model to these raw scores to get
# calibrated log-likelihood ratios.
calibrated_scores_test = applyCalibrationModel(calibration_model, raw_scores_test)

# Finally, compute the predicted labels. For calibrated scores (log-likelihood ratios),
# the optimal decision threshold for a given application is derived from the prior.
# The threshold is -numpy.log(p / (1 - p)).
threshold = -numpy.log(p / (1 - p))

# Assign label 1 if score > threshold, otherwise 0.
predicted_labels = (calibrated_scores_test > threshold).astype(int)

# `predicted_labels` now contains the final class predictions for the DTE samples.
```

---

### Progetto - `question example 7`

**Given the following functions (assume these functions are already implemented unless specified):**

*   `trainPCA`, `applyPCA`, `trainLDA`, `applyLDA`
*   `evaluateScores(S, L)`: computes a `performance metric`

**1. Provide possible `signatures` (prototype) for these functions, briefly explaining the function parameters and the return value.**
**2. Using these functions, write a short `Python program` to train and apply an `LDA binary classifier` with `PCA pre-processing`. The `program` should employ the provided data to train the `model` and to select an `optimal value` for the `PCA dimensionality`.**

### Risposta

#### 1. `Function Signatures` (`Prototypes`)

```python
def trainPCA(D, m):
    """
    Trains a PCA model.
    Args:
        D (numpy.ndarray): Training data matrix (features x samples).
        m (int): The number of principal components to keep.
    Returns:
        tuple: The projection matrix (P) and the mean vector (mu).
    """
    pass

def applyPCA(P, mu, D):
    """
    Applies a trained PCA model to project data.
    Args:
        P (numpy.ndarray): Projection matrix from trainPCA.
        mu (numpy.ndarray): Mean vector from the training data.
        D (numpy.ndarray): Data matrix to project (features x samples).
    Returns:
        numpy.ndarray: The projected data matrix.
    """
    pass

def trainLDA(D, L, m_lda=1):
    """
    Trains an LDA model.
    Args:
        D (numpy.ndarray): Training data matrix (features x samples).
        L (numpy.ndarray): Training labels array.
        m_lda (int): Number of discriminant directions (for binary case, it's 1).
    Returns:
        numpy.ndarray: The LDA projection matrix (W).
    """
    pass

def applyLDA(W, D):
    """
    Applies a trained LDA model to project data, yielding scores.
    Args:
        W (numpy.ndarray): LDA projection matrix from trainLDA.
        D (numpy.ndarray): Data matrix to project (features x samples).
    Returns:
        numpy.ndarray: The 1-D array of classification scores.
    """
    pass

def evaluateScores(S, L):
    """
    Computes a performance metric for given scores and labels.
    Args:
        S (numpy.ndarray): 1-D array of classification scores.
        L (numpy.ndarray): 1-D array of true labels.
    Returns:
        float: The computed performance metric (e.g., minDCF).
    """
    pass
```

#### 2. `Python Program` per `PCA+LDA` con Selezione della Dimensionalità

```python
best_pca_dim = -1
best_metric = float('inf')  # Assuming lower metric is better (e.g., minDCF)

# Iterate through possible PCA dimensions
for m in range(1, DTR.shape[0] + 1):
    # Train PCA on the training set
    P, mu = trainPCA(DTR, m)

    # Apply the same PCA transformation to both training and validation sets
    DTR_pca = applyPCA(P, mu, DTR)
    DVAL_pca = applyPCA(P, mu, DVAL)

    # Train LDA on the PCA-projected training data
    # For a binary problem, we only seek 1 discriminant direction.
    W_lda = trainLDA(DTR_pca, LTR, 1)

    # Compute scores by applying LDA to the PCA-projected validation data
    S_val = applyLDA(W_lda, DVAL_pca)

    # Evaluate the performance for this value of m
    current_metric = evaluateScores(S_val, LVAL)

    # Update the best PCA dimension if performance improved
    if current_metric < best_metric:
        best_metric = current_metric
        best_pca_dim = m
```

---

### Progetto - `question example 8` (`Variation` di `PQ4`: `LDA` per Riduzione della Dimensionalità)

**Given a `multi-class classification problem`, consider the following functions:**

*   `trainLDA`: trains an `LDA model` for `dimensionality reduction`.
*   `applyLDA`: applies a trained `LDA model` to project data.
*   `trainClassifier(D_reduced, L)`: trains a `generic classifier` on the `reduced-dimension data`.
*   `scoreClassifier(clsModel, D_reduced)`: computes `classification scores` using the `classifier`.
*   `evaluateScores(S, L)`: computes a `performance metric` (e.g., `multi-class accuracy`).

**a) Provide a possible `signature` and an `implementation` for the functions `trainLDA` and `applyLDA` for `dimensionality reduction`, briefly explaining the function parameters and the return value.**
**b) Using these functions, write the `Python code` to train a `classifier`, optimizing the `dimensionality of LDA m` with a `single-fold validation approach`. Assume the `number of classes C` is known.**

### Risposta

#### a) `Signature` e `Implementation` di `trainLDA` e `applyLDA`

```python
import numpy
import scipy.linalg

def trainLDA(D, L, m):
    """
    Trains an LDA model for dimensionality reduction by computing the
    m directions that maximize the ratio of between-class to within-class scatter.
    
    Args:
        D (numpy.ndarray): The training data matrix (features x samples).
        L (numpy.ndarray): The training labels vector.
        m (int): The number of discriminant directions to retain.
        
    Returns:
        numpy.ndarray: The LDA projection matrix W of shape (num_features, m).
    """
    # 1. Calculate scatter matrices with optimized single loop
    mu_total = D.mean(1).reshape(D.shape[0], 1)
    SW = numpy.zeros((D.shape[0], D.shape[0]))
    SB = numpy.zeros((D.shape[0], D.shape[0]))
    
    # Single loop to calculate both SW and SB
    for i in numpy.unique(L):
        D_class = D[:, L == i]
        mu_class = D_class.mean(1).reshape(D.shape[0], 1)
        nc = D_class.shape[1]
        
        # Within-class scatter for this class
        centered_data = D_class - mu_class
        SW += centered_data @ centered_data.T
        
        # Between-class scatter for this class
        diff_mean = mu_class - mu_total
        SB += nc * (diff_mean @ diff_mean.T)
        
    # 2. Solve the generalized eigenvalue problem: SB * v = lambda * SW * v
    # We use scipy.linalg.eigh which solves A*x = lambda*B*x
    # Note: eigh returns eigenvalues in ascending order.
    eigenvalues, eigenvectors = scipy.linalg.eigh(SB, SW)
    
    # 3. Select the m eigenvectors with the largest eigenvalues
    # We flip the order to get descending eigenvalues.
    W = eigenvectors[:, ::-1][:, 0:m]
    
    return W

def applyLDA(W, D):
    """
    Applies a trained LDA model to project data into a lower-dimensional space.
    
    Args:
        W (numpy.ndarray): The LDA projection matrix from trainLDA.
        D (numpy.ndarray): The data matrix to project (features x samples).
        
    Returns:
        numpy.ndarray: The projected data matrix with m features.
    """
    # Project the data: D_projected = W^T * D
    return numpy.dot(W.T, D)
```

#### b) `Code` per `LDA Dimensionality Optimization` e `Evaluation`

```python
best_m_lda = -1
best_metric_val = -float('inf') # Assuming higher metric is better (e.g., accuracy)

# The maximum number of LDA dimensions is C-1.
max_lda_dims = C - 1

# Iterate through possible LDA dimensions
for m_lda in range(1, max_lda_dims + 1):
    # 1. Train LDA for dimensionality reduction on the training set
    lda_reducer_model_W = trainLDA(DTR, LTR, m_lda)
    
    # 2. Project both training and validation sets
    DTR_reduced = applyLDA(lda_reducer_model_W, DTR)
    DVAL_reduced = applyLDA(lda_reducer_model_W, DVAL)
    
    # 3. Train a generic classifier on the reduced-dimension training data
    classifier = trainClassifier(DTR_reduced, LTR)
    
    # 4. Score the classifier on the reduced-dimension validation data
    S_val = scoreClassifier(classifier, DVAL_reduced)
    
    # 5. Evaluate the performance
    current_metric = evaluateScores(S_val, LVAL)
    
    # 6. Check if this m_lda is the best so far
    if current_metric > best_metric_val:
        best_metric_val = current_metric
        best_m_lda = m_lda
```

---

### Progetto - `question example 9` (`Variation` di `PQ5`: `Logistic Regression`)

**You are given the following functions (assume these functions are already implemented unless specified):**

*   `trainLogisticRegression(D, L, lambda_reg)`: trains a `binary logistic regression model`.
*   `scoreLogisticRegression(lrModel, D)`: computes `classification scores` for the `model`.
*   `evaluateScores(S, L)`: computes an `evaluation metric` (e.g. `minimum DCF`).

**Write the `Python code` to train and apply a `Logistic Regression classifier`, optimizing the `hyper-parameter lambda_reg` using `single-fold cross-validation`. Then, evaluate the selected `model` on the `test data`. Additionally, provide an `implementation` for `scoreLogisticRegression`, assuming `lrModel` is an object with fields `w` (`weights`) and `b` (`bias`).**

### Risposta

#### `Code` per `Logistic Regression Optimization` e `Evaluation`

```python
lambda_values = [1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1.0]

best_lambda = None
best_metric_val = float('inf') # Assuming lower metric is better (e.g., minDCF)

# --- Hyper-parameter tuning using validation set ---
for l in lambda_values:
    # Train the logistic regression model on the training data
    lrModel = trainLogisticRegression(DTR, LTR, l)
    
    # Compute scores on the validation data
    S_val = scoreLogisticRegression(lrModel, DVAL)
    
    # Evaluate the performance
    current_metric = evaluateScores(S_val, LVAL)
    
    # Update the best hyperparameter if performance improved
    if current_metric < best_metric_val:
        best_metric_val = current_metric
        best_lambda = l
```

#### `Implementation` di `scoreLogisticRegression`

```python
import numpy

def scoreLogisticRegression(lrModel, D):
    w = lrModel.w
    b = lrModel.b
    
    # s = w^T * D + b
    scores = numpy.dot(w.T, D) + b
    
    # Return a flat 1-D array of scores
    return scores.flatten()
```