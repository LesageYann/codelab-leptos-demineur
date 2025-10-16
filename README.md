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
- the-grid 
- avoid-cloning (vous êtes ici)
- improve-the-game

## Concept Leptos

Sur un signal get va créer un clone de la valeur. Il est donc important de privilégier les références grâce à `read` et `with` quand c'est possible.
De même un map n'est pas très efficace car il va re-rendre tout le tableau si un élément change.
On utilisera plutôt le composant `<For>`.

## TODO de l'étape `avoid-cloning`

Nous allons juste reprendre la boucle pour rendre la grille : 

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