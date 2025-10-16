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
- first-component (vous êtes ici)
- let-s-interact
- be-reactive
- use-component-in-component
- first-effect
- first-cell
- the-grid
- avoid-cloning
- improve-the-game

## Concept Leptos

Dans Leptos, la déclaration d'un composant repose sur deux choses :
 - la macro `#[component]`
 - une fonction qui retourne un `impl IntoView`

Pour facilité la seconde partie, Leptos fournit la macro `view!` qui va retourner un élément `impl IntoView` depuis un RSX (comme JSX, mais en Rust).
Regardons un exemple simple

```rust
use leptos::prelude::*;
#[component]
fn Game() -> impl IntoView {
    view! {
        <div>
            <h1>"Bonjour tout le monde"</h1>
            <p>"Ceci est un paragraphe de notre composant"</p>
        </div>
    }
}
```

On peut faire varier l'affichage du composant en fonction de variables ou de paramètres avec des if else
```rust
use leptos::prelude::*;
#[component]
fn HelloWorld() -> impl IntoView {
    let is_a_balrog = true;
    if  is_a_balrog {
      view! {<p>fuyez ! pauvres fous !</p>}.into_any()  
    } else {
        view! {
            <div>
                <h1>"Bonjour tout le monde"</h1>
                <p>"Ceci est un paragraphe de notre composant"</p>
            </div>
        }.into_any()
    }
}
```

## TODO de l'étape `first-component`

Pour notre première étape, nous allons créer un composant `Game` qui affichera un message de fin de partie et un bouton pour relancer une partie.

Voici comment procéder pas à pas :

Créons un dossier components dans src et un fichier mod.rs dans ce dossier.
Ce dossier contiendra les composants de notre application.
Créeons ensuite un fichier game.rs dans ce dossier et n'oublions pas de le rattacher au mod.rs.

```rust
// src/components/game.rs

use leptos::prelude::*;
#[component]
fn Game() -> impl IntoView {
    view! {
      <div class="overlay">
        <div class="overlay-container">
          <div class="message">"Perdu"</div>
          <button>"Rejouer"</button>
        </div>
      </div>
    }
}
```

On peut faire varier l'affichage du composant en fonction de variables ou de paramètres avec des if else
```rust
use leptos::prelude::*;
#[component]
fn Game() -> impl IntoView {
    let is_game_over = true;
    if is_game_over {
        view! {
      <div class="overlay">
        <div class="overlay-container">
          <div class="message">"Perdu"</div>
          <button >
              "Rejouer"
          </button>
        </div>
      </div>
    }.into_any()
    } else {
        view! {}.into_any()
    }
}
```


Ajoutons notre composant dans la home page : 
```Rust 
// src/app.rs
use crate::components::game::Game;

// [...]
#[component]
fn HomePage() -> impl IntoView {

    view! {
        <Game/>
    }
}
```