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
- be-reactive (vous êtes ici)
- use-component-in-component
- first-effect
- first-cell
- the-grid
- avoid-cloning
- improve-the-game

## Concept Leptos

Leptos ne réagit au changement des données que si celles-ci sont des signaux.
Il y a deux façons de créer des signaux :
- avec la fonction `signal`.
- avec la fonction `RwSignal::new`.

```rust
let (names, set_names) = signal(Vec::new());
if names.get().is_empty() {
    set_names(vec!["Alice".to_string()]);
}

let rw_names = RwSignal::new(Vec::new());
if rw_names.get().is_empty() {
    rw_names.set(vec!["Alice".to_string()]);
}
```

il est à noter que les signaux ne seront reactifs que si on les utilise dans la macro `view!` ou s'ils sont passé en props à un composant.

## TODO de l'étape `be-reactive`

Changez le bouton `rejouez` pour qu'il change le signal `is_game_over` à `false` quand on clique dessus.

```rust
//src/components/game.rs

//[...]
let is_game_over = RwSignal::new(true); //attention le mut à de nouveau disparu ! 
view ! { {move ||{ // comme dit plus haut, il faut une closure pour être dans un contexte réactif
    if is_game_over.get() {
    //[...]
    <button on:click=move |_| {is_game_over.set(false)} >
      "Rejouer"
    </button>
    //[...]
}}}
```

