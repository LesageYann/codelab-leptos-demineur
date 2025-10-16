# codelab-leptos-demineur
Un codelab pas à pas pour apprendre à faire un front Rust avec Leptos.
Le but est d'exposer les concepts clefs du framework Leptos.
Il est conseillé d'avoir déjà des bases en Rust.
Dans le cas contraire, la lecture du livre Rust est conseillée : https://doc.rust-lang.org/book/.

## Instructions d'utilisation

Le projet est organisé en deux parties : 
- Le dossier solutions contient une solution aux exercices.
- Le Readme contient, à chaque étape, un concept leptos et un exercice à réaliser

Les différents exercices sont matérialisés par des tags git dont voici la liste ordonnée : 
- instructions (commencez ici)
- init 
- first-component 
- let-s-interact
- be-reactive 
- use-component-in-component 
- first-effect (vous êtes ici)
- first-cell
- the-grid
- avoid-cloning
- improve-the-game

## Concept Leptos

Un effect est une fonction qui s'exécute automatiquement lorsque les signaux qu'elle utilise changent.
Le tracking des dépendances est :
- automatique: vous n'avez pas besoin de déclarer les dépendances
- dynamique: les dépendances sont recalculées à chaque exécution de l'effect. 
si vous avez une branche conditionnelle, les dépendances peuvent changer à chaque exécution.

Pour éviter la triche de joueurs indélicats, la génération de la grille de démineur doit se faire côté serveur.
Leptos propose un moyen simple de faire des requêtes HTTP de manière transparente :

```rust
#[server]
pub async fn server_function() -> Result<Vec<Case>,  ServerFnError> {
    // tout le code ici est exécuté côté serveur
}

#[component]
pub fn Game() -> impl IntoView {
    let new_grid = Resource::new(move || game_status, |refresh| server_function(refresh));
}
```

Vous pouvez aussi voir l'apparition d'une nouvelle structure : Resource. Cette structure est un signal asynchrone.
Elle possède une fonction de refetch.



## TODO de l'étape `first-effect`

Dans cette étape, nous allons initialiser une grille de démineur.
Pour ce faire, nous allons utiliser un modèle simple qui est fournit dans le dossier `solutions/src/model`. 
Ce modèle contient trois parties :
- board: une structure pour décrire une grille de démineur, elle sera stocker côté serveur
- board_store: une structure qui réorganise la donnée pour le front. 
Si vous jetez un oeil au code, vous verrez que cette structure utilise des macros `server`.
Pour ce qui est des stores, il s'agit d'une structure qui dérive de la macro store. 
- server_state: une structure de données globale pour le serveur. 

Il faudra rajouter la déclaration du module dans `lib.rs`
```rust
// src/lib.rs
pub mod model;
```
Il nous faudra aussi rajouter un state partagé pour le serveur.

```rust
// /src/main.rs
// [...]
async fn main() {
    // [...]
    use std::sync::{Arc, Mutex};
    use demineur::model::board::Board;
    use demineur::model::server_state::ServerState;
    // [...]
    let app_state = Arc::new(Mutex::new(ServerState { board: Board::new(10) }));

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                move || provide_context(app_state.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            }
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);
    // [...]
}
```

Nous allons maintenant préparer notre resource qui va appeler une fonction serveur pour initialiser la grille.

```rust
// src/components/game.rs
pub fn Game() -> impl IntoView {
    // [...]
    let game_status = RwSignal::new(GameStatus::New);
    async fn reset_grid(game_status: RwSignal<GameStatus>) -> Option<Vec<Case>> {
        if game_status.get() != GameStatus::New {
            return None;
        }
        game_status.set(GameStatus::Playing);
        match generate_new_grid().await {
            Ok(data) => Some(data),
            Err(err) => {
                console_log(&format!("Error generating new grid: {:?}", err));
                None
            }
        }
    }

    let new_grid = Resource::new(move || game_status, |refresh| reset_grid(refresh));
    // [...]
}
```

Et enfin, notre effet : 

```rust
// src/components/game.rs
#[component]
pub fn Game() -> impl IntoView {
    // [...]
    Effect::new(move || {
        match game_status.get() {
            GameStatus::New => new_grid.refetch(),
            _ => {}
        };
    });
    
    view! {
        // [...]
    }
}
```