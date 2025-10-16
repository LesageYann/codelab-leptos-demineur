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
- use-component-in-component (vous êtes ici)
- first-effect
- first-cell
- the-grid
- avoid-cloning
- improve-the-game

## Concept Leptos

Pas concept cette fois-ci, mais un peu de refactoring pour utiliser un composant dans un autre.

## TODO de l'étape `use-component-in-component`

Transformons notre boolean en un enum pour plus de clarté.

```rust
//src/components/mod.rs

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameStatus {
    Playing,
    New,
    Lost,
}
```

On découpe game.rs en extrayant la partie overlay.

```rust
// src/components/game.rs
use leptos::prelude::*;
use crate::components::GameStatus;

#[component]
pub fn GameOverOverlay(state: RwSignal<GameStatus>) -> impl IntoView {
    view! {
        {move ||
             if *state.read() == GameStatus::Lost {
                 view! {
                     <div class="overlay">
                        <div class="overlay-container">
                            <div class="message">"Perdu"</div>
                            <button
                                on:click=move |_| {
                                    state.set(GameStatus::New);
                                }
                            >
                                "Rejouer"
                            </button>
                        </div>
                     </div>
                 }.into_any()
             } else {
                 view! {}.into_any()
             }
        }
    }
}
```

Et on l'utilise dans game.rs

```rust
// src/components/game.rs
#[component]
pub fn Game() -> impl IntoView {
    let game_status = RwSignal::new(GameStatus::Lost);
    view! {
      <GameOverOverlay state=game_status />
    }
}
```