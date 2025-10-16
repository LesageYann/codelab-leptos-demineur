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
- first-cell 
- the-grid (vous êtes ici)
- avoid-cloning
- improve-the-game

## Concept Leptos

## TODO de l'étape `the-grid`

On va lier le chargement de la grille à notre store pour afficher la grille ensuite.

```rust
// src/components/game.rs

#[component]
pub fn Game(case: Case) -> impl IntoView {
    let state = Store::new(GameState::new());
    // [...]
    let new_grid = Resource::new(move || game_status, |refresh| reset_grid(refresh));

    Effect::new(move || {
        match new_grid.get() {
            Some(Some(data)) => {
                new_grid.set(None);
                let formatted_cases = state.get().format_cases_to_index_cases(data);
                state.rows().set(formatted_cases);
            }
            _ => {}
        };
    });
    // [...]
}
```

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
// src/components/game.rs

#[component]
pub fn Game(case: Case) -> impl IntoView {
    // [...]

    view! {
        <div class="square">
            <div class="grid" style="grid-template-columns: repeat(10, 1fr); grid-template-rows: repeat(10, 1fr); ">
                { state.rows()
                    .get()
                    .iter()
                    .enumerate()
                    .map(|(idx, case)| {
                        let case_cloned = case.clone();
                        view! {
                          <Cell
                            case=case_cloned
                            on:click = move |_| {
                              console_log(&format!("click on cell {}", idx));
                            }
                          />
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
        <GameOverOverlay state=game_status /> 
    }
}
```