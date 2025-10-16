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
- let-s-interact (vous êtes ici)
- be-reactive
- use-component-in-component
- first-effect
- first-cell
- the-grid
- avoid-cloning
- improve-the-game

## Concept Leptos

Dans Leptos, le click droit et le click gauche sont gérés avec deux fonctions différentes ayant la même signature :
Un seul paramètre : une closure prenant un événement en paramètre.
Nous verrons plus tard comment lancer des traitements asynchrones dans ces closures.

```rust
use leptos::prelude::*;
#[component]
fn TextButton() -> impl IntoView {
    view! {
        <button 
            on:click=move |_| log::info!("click gauche")
            on:contextmenu=move |ev| {
                ev.prevent_default();
                log::info!("click droit");
            }
        >
            "Click ici"
        </button>
    }
}
```

## TODO de l'étape `first-component`

Changez le bouton `rejouez` pour qu'il change le signal `is_game_over` à `false` quand on clique dessus.

```rust
//src/components/game.rs

//[...]
let mut is_game_over = true; //attention à bien rajouter mut
//[...]
<button on:click=move |_| {is_game_over = false} >
"Rejouer"
</button>
//[...]
```

Si vous ne voyez pas le changement... C'est normal, nous allons corrigez ça dans l'étape suivante !
