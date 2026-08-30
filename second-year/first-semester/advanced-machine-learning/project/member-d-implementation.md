# Apply Grid Search for hyperparameters tuning
from sklearn.model_selection import GridSearchCV

# Definizione della griglia di parametri da testare
# Nota: usiamo 'model__' come prefisso perché il modello è dentro una Pipeline chiamata 'model'
param_grid = {
    'model__n_estimators': [100, 200, 300],    # Numero di alberi
    'model__max_depth': [None, 10, 20],        # Profondità massima degli alberi
    'model__min_samples_split': [2, 5, 10]     # Numero minimo di campioni per dividere un nodo
}

# Creazione dell'oggetto GridSearchCV
grid_search = GridSearchCV(
    estimator=pipeline,       # La tua pipeline esistente
    param_grid=param_grid,    # I parametri da provare
    cv=5,                     # Cross-validation a 5 fold (più robusta)
    n_jobs=-1,                # Usa tutti i processori disponibili
    verbose=1,                # Mostra il progresso
    scoring='r2'              # Ottimizza per R2 score
)

# Esecuzione della ricerca (può richiedere qualche minuto)
print("Inizio Grid Search...")
grid_search.fit(X_train, y_train)

# Risultati
print(f"\nMigliori parametri trovati: {grid_search.best_params_}")
print(f"Miglior score (CV): {grid_search.best_score_:.4f}")

# Valutazione finale sul Test Set con il miglior modello
best_model = grid_search.best_estimator_
test_score = best_model.score(X_test, y_test)
print(f"Score finale sul Test Set: {test_score:.4f}")