use std::sync::{Arc, Mutex, Condvar};
use std::time::Instant;

type TokenAcquirer = dyn Fn() -> Result<(String, Instant), String> + Sync;

enum ManagerState {
    Empty,
    Pending,
    Valid,
}

struct Inner {
    acquirer: Box<TokenAcquirer>,
    token: String,
    delay: Instant,
    state: ManagerState,
}

struct TokenManager {
    inner: Arc<Mutex<Inner>>,
    cv: Condvar,
}

impl TokenManager {
    pub fn new(acquire_token: Box<TokenAcquirer>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                acquirer: acquire_token,
                token: String::new(),
                delay: Instant::now(),
                state: ManagerState::Empty,
            })),
            cv: Condvar::new(),
        }
    }

    pub fn get_token(&self) -> Result<String, String> {
        let mut inner_guard = self.inner.lock().unwrap();

        loop {
            // Controllo lo stato del manager
            match inner_guard.state {
                ManagerState::Empty => {
                    // Cambio lo stato a pending
                    inner_guard.state = ManagerState::Pending;
    
                    // Invoco la funzione per acquisire il token
                    match (inner_guard.acquirer)() {
                        // Risultato valido
                        Ok(t) => {
                            // Memorizzo il token e la sua scadenza
                            inner_guard.token = t.0;
                            inner_guard.delay = t.1;
    
                            // Imposto lo stato a valid
                            inner_guard.state = ManagerState::Valid;

                            // Notifico il cambio di state
                            self.cv.notify_all();

                            // Ritorno una copia del token
                            return Ok(inner_guard.token.clone());
                        },
                        // Errore
                        Err(e) => {
                            // Imposto lo stato ad empty
                            inner_guard.state = ManagerState::Empty;

                            // Notifico il cambio di state
                            self.cv.notify_all();
    
                            // Restituisco l'errore ricevuto
                            return Err(e);
                        }
                    };
                },
                ManagerState::Pending => {
                    // Attendo senza consumare cicli di CPU che lo stato del manager cambi
                    let new_guard = self.cv.wait_while(inner_guard, |guard| {
                        match guard.state {
                            ManagerState::Pending => return true,
                            _ => return false,
                        }
                    }).unwrap();
    
                    inner_guard = new_guard;
                },
                ManagerState::Valid => {
                    let now = Instant::now();
    
                    // Controllo se il token è scaduto
                    if now < inner_guard.delay {
                        // Il token non è scaduto, ne ritorno una copia
                        return Ok(inner_guard.token.clone());
                    } else {
                        // Imposto lo stato a pending
                        inner_guard.state = ManagerState::Pending;
    
                        // Inizio una nuova richiesta di acquisizione
                        match (inner_guard.acquirer)() {
                            Ok(t) => {
                                // Memorizzo il token e la sua scadenza
                                inner_guard.token = t.0;
                                inner_guard.delay = t.1;
        
                                // Cambio lo stato a valid
                                inner_guard.state = ManagerState::Valid;

                                // Notifico il cambio di state
                                self.cv.notify_all();
        
                                return Ok(inner_guard.token.clone());
                            },
                            Err(e) => {
                                inner_guard.state = ManagerState::Empty;

                                // Notifico il cambio di state
                                self.cv.notify_all();
        
                                return Err(e);
                            }
                        };
                    }
                },
            }
        }        
    }

    pub fn try_get_token(&self) -> Option<String> {
        let inner_guard = self.inner.lock().unwrap();

        match inner_guard.state {
            // Se lo stato è valid
            ManagerState::Valid => {
                let now = Instant::now();

                // Se il token non è scaduto
                if now < inner_guard.delay {
                    // Ritorno una copia del token
                    return Some(inner_guard.token.clone());
                } else {
                    // Ritorno None se scaduto
                    return None;
                }
            },
            // Ritorno None in tutti gli altri casi
            _ => return None,
        }
    }
}

fn main() {
    
}
