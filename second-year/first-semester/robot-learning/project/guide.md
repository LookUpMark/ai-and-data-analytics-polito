# Estensione Avanzata per il Sim-to-Real Transfer: Automatic Domain Randomization (ADR)

## 1\. Introduzione e Analisi del Problema

L'apprendimento per rinforzo (RL) in simulazione soffre intrinsecamente del *Reality Gap*: le policy ottimali in un ambiente ideale (MuJoCo) spesso falliscono nel mondo reale a causa di discrepanze fisiche non modellate. La soluzione base implementata nel corso, la *Uniform Domain Randomization* (UDR) sulle masse, è un passo necessario ma insufficiente. La UDR statica presenta un dilemma fondamentale:

  * **Range troppo stretto:** La policy va in *overfitting* sulla simulazione e non generalizza.
  * **Range troppo ampio:** L'agente affronta scenari fisicamente impossibili o contraddittori fin dall'inizio, impedendo la convergenza (*Catastrophic Forgetting*).

Per l'estensione del progetto, propongo l'implementazione dell'**Automatic Domain Randomization (ADR)**. Questa tecnica, resa celebre da OpenAI per il progetto *Dactyl* (risoluzione del Cubo di Rubik con mano robotica), trasforma la selezione dei parametri fisici in un curriculum automatico. L'ambiente diventa progressivamente più difficile (aumentando l'entropia del dominio) solo quando l'agente dimostra competenza, garantendo un apprendimento robusto e continuo.

## 2\. Metodologia Proposta

L'estensione si basa su tre pilastri che elevano il progetto da un semplice tuning a un lavoro di ricerca:

1.  **Algoritmo ADR:** Implementazione di un loop di feedback dove la performance dell'agente (Reward medio) controlla dinamicamente i confini (`bounds`) della randomizzazione.
2.  **Multiphysics Randomization:** Estensione della randomizzazione dalla sola **Massa** a **Smorzamento dei Giunti (Damping)** e **Attrito del Suolo (Friction)**. Questi sono i parametri più critici per la stabilità dell'Hopper.
3.  **Integrazione SB3:** Sviluppo di una `Callback` custom per Stable Baselines3 che gestisce la logica del curriculum senza modificare l'algoritmo di RL sottostante (PPO/SAC).

### Giustificazione Accademica

L'ADR è considerato lo stato dell'arte per il Sim-to-Real "zero-shot". Citando **Akkaya et al. (2019)** [1], l'ADR permette di generare policy che esibiscono "meta-learning emergente", adattandosi a perturbazioni fisiche mai viste durante il training. Inoltre, randomizzare l'attrito è cruciale per la locomozione, come dimostrato da **Tan et al. (2018)** [2] nel lavoro sui quadrupedi (Minitaur).

-----

## 3\. Implementazione Tecnica

Di seguito le modifiche necessarie ai file del progetto.

### 3.1 Modifica all'Ambiente (`custom_hopper.py`)

Dobbiamo modificare la classe `CustomHopper` per mantenere lo "stato" della difficoltà corrente (i delta di randomizzazione) e permettere la modifica a runtime delle proprietà fisiche tramite i binding di MuJoCo.

```python
import numpy as np
from gymnasium.envs.mujoco import MujocoEnv
import gymnasium.utils as utils

class CustomHopper(MujocoEnv, utils.EzPickle):
    def __init__(self, domain=None, **kwargs):
        #... (inizializzazione esistente)...
        MujocoEnv.__init__(self, xml_file, frame_skip, **kwargs)

        # --- ADR SETUP ---
        # Salviamo i valori nominali (backup della fisica originale)
        self.original_masses = np.copy(self.model.body_mass)
        self.original_damping = np.copy(self.model.dof_damping)
        self.original_friction = np.copy(self.model.geom_friction)

        # Stato ADR: definisce l'ampiezza attuale della randomizzazione
        self.adr_state = {
            "mass_range": 0.0,      # % di variazione (es. 0.1 = +/- 10%)
            "damping_range": 0.0,   # % di variazione
            "friction_range": 0.0   # % di variazione
        }
        
        # Iperparametri ADR
        self.adr_step = 0.05        # Step di espansione/contrazione (5%)
        self.max_range = 0.5        # Limite massimo (50% variazione)

    def sample_parameters(self):
        """Genera nuovi parametri fisici basati sulla difficoltà attuale (ADR)"""
        params = {}
        
        # 1. Massa (Variazione percentuale uniforme)
        m_delta = self.adr_state["mass_range"]
        m_scale = self.np_random.uniform(1.0 - m_delta, 1.0 + m_delta, size=self.model.body_mass.shape)
        params["mass"] = self.original_masses * m_scale

        # 2. Damping (Smorzamento giunti)
        d_delta = self.adr_state["damping_range"]
        d_scale = self.np_random.uniform(1.0 - d_delta, 1.0 + d_delta, size=self.model.dof_damping.shape)
        params["damping"] = self.original_damping * d_scale

        # 3. Friction (Attrito geom)
        f_delta = self.adr_state["friction_range"]
        f_scale = self.np_random.uniform(1.0 - f_delta, 1.0 + f_delta, size=self.model.geom_friction.shape)
        params["friction"] = self.original_friction * f_scale
        
        return params

    def set_parameters(self, params):
        """Applica i parametri al motore fisico"""
        self.model.body_mass[:] = params["mass"]
        self.model.dof_damping[:] = params["damping"]
        self.model.geom_friction[:] = params["friction"]

    def update_adr(self, mean_reward, high_th, low_th):
        """Chiamato dalla Callback: Espande o Contrae i range"""
        msg = "stable"
        
        if mean_reward > high_th: # L'agente è bravo -> Aumenta difficoltà
            msg = "expand"
            for k in self.adr_state:
                self.adr_state[k] = min(self.max_range, self.adr_state[k] + self.adr_step)
                
        elif mean_reward < low_th: # L'agente fallisce -> Riduci difficoltà
            msg = "contract"
            for k in self.adr_state:
                self.adr_state[k] = max(0.0, self.adr_state[k] - self.adr_step)
                
        return msg, self.adr_state

    def reset_model(self):
        # Ad ogni reset, campiona una nuova fisica entro i range correnti
        new_params = self.sample_parameters()
        self.set_parameters(new_params)
        
        #... (codice standard di reset: qpos, qvel noise)...
        noise_low = -self._reset_noise_scale
        noise_high = self._reset_noise_scale
        qpos = self.init_qpos + self.np_random.uniform(low=noise_low, high=noise_high, size=self.model.nq)
        qvel = self.init_qvel + self.np_random.uniform(low=noise_low, high=noise_high, size=self.model.nv)
        self.set_state(qpos, qvel)
        return self._get_obs()
```

### 3.2 Implementazione Callback (`train.py`)

La logica di controllo deve risiedere nel processo di training. Creiamo una Callback personalizzata.

```python
from stable_baselines3.common.callbacks import BaseCallback
import numpy as np

class ADRCallback(BaseCallback):
    def __init__(self, check_freq: int, high_th: float, low_th: float, verbose=1):
        super(ADRCallback, self).__init__(verbose)
        self.check_freq = check_freq
        self.high_th = high_th  # Es. 2000: Se supera, rendi più difficile
        self.low_th = low_th    # Es. 1000: Se scende sotto, rendi più facile

    def _on_step(self) -> bool:
        if self.n_calls % self.check_freq == 0:
            # Calcola reward medio dagli ultimi episodi nel buffer
            ep_info = self.model.ep_info_buffer
            if len(ep_info) > 0:
                mean_reward = np.mean([ep['r'] for ep in ep_info])
                
                # Accesso all'ambiente (gestendo il wrapper DummyVecEnv)
                env = self.training_env.envs.unwrapped
                
                # Aggiorna ADR
                status, params = env.update_adr(mean_reward, self.high_th, self.low_th)
                
                # Log su Tensorboard (Cruciale per il report!)
                self.logger.record("adr/mass_range", params["mass_range"])
                self.logger.record("adr/friction_range", params["friction_range"])
                self.logger.record("adr/mean_reward_check", mean_reward)
                
                if self.verbose > 0 and status!= "stable":
                    print(f"ADR Update: {status.upper()} | Reward: {mean_reward:.0f} | Ranges: {params}")
        return True

# Esempio di utilizzo nel main
# model = PPO("MlpPolicy", env,...)
# adr_callback = ADRCallback(check_freq=2048, high_th=2500, low_th=1500)
# model.learn(total_timesteps=1000000, callback=adr_callback)
```

-----

## 4\. Risultati Attesi e Valutazione

Per massimizzare il voto, nel report finale dovrai includere i grafici di Tensorboard generati dalla callback:

1.  **Analisi delle Curve:**

      * Dovresti vedere la curva del Reward salire inizialmente, poi stabilizzarsi.
      * Contemporaneamente, le curve `adr/mass_range` e `adr/friction_range` dovrebbero iniziare a salire (a gradini).
      * **Interpretazione:** Questo dimostra che l'agente sta mantenendo alte performance *nonostante* l'ambiente stia diventando sempre più ostile. È la prova visuale della robustezza.

2.  **Confronto (Ablation Study):**

      * Confronta la policy finale ADR con quella UDR base sul `CustomHopper-target-v0` (il dominio target nascosto). La policy ADR dovrebbe avere una varianza minore e un tasso di successo più alto.

## 5\. Riferimenti Bibliografici Suggeriti

Nel report, cita questi paper per validare l'approccio:

  * **[1] OpenAI et al. (2019).** *"Solving Rubik's Cube with a Robot Hand"*. arXiv preprint. (Fonte dell'algoritmo ADR).
  * **[2] Tan, J., et al. (2018).** *"Sim-to-Real: Learning Agile Locomotion For Quadruped Robots"*. RSS. (Fondamentale per giustificare la randomizzazione di attrito e smorzamento nella locomozione).
  * **[3] Peng, X. B., et al. (2018).** *"Sim-to-Real Transfer of Robotic Control with Dynamics Randomization"*. ICRA. (Panoramica generale sulla randomizzazione dinamica).