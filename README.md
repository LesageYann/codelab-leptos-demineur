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
- first-effect
- first-cell (vous êtes ici)
- the-grid
- avoid-cloning
- improve-the-game

## Concept Leptos

Pour cette étape, le nouveau concept est l'utilisation de class conditionnelles dans les composants.

```rust
#[component]
fn MySpecialButton() -> impl IntoView {
    let mon_test = true;
    view! {
        <button
            class="my-button"
            class:colored=move || mon_test
        >
        </button>
    }
}
```


## TODO de l'étape `first-cell`

Notre cellule à plusieurs états possibles : 
- cachée
- marquée (drapeau)
- game over (mine cliquée)
- révélée (vide, nombre, mine)

Pour les trois premiers états, nous pouvons les représenter comme des boutons. 
Bien que la mine pourrait ne pas être cliquable, elle contient le même texte que les autres états.
Pour le dernier état, nous allons utiliser une div.

Ce qui donne le code suivant : 

```rust
use crate::model::board::Case;
use leptos::prelude::*;

#[component]
pub fn Cell(case: Case) -> impl IntoView {
    if case.is_revealed() && !case.is_mine() {
        view! {
            <div
                class="cell square revealed"
            >
            {case.get_mines_around()}
            </div>
        }.into_any()
    } else {
        view! {
            <button
                class="cell square"
                class:revealed=move || case.is_revealed()
                class:mine=move || case.is_revealed() && case.is_mine()
                class:flag=move || case.is_flagged()
            />
        }.into_any()
    }
}
```
Notez que le `if else` n'est pas dans un `view! { ... }`. La réactivité sera donc gérée par le composant parent.