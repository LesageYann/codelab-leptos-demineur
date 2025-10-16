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
- avoid-cloning 
- improve-the-game (vous êtes ici)

## Concept Leptos

## TODO de l'étape `improve-the-game`

Pour pouvoir révélé les cases, nous allons devoir demander le status de la case au serveur.
Or nous pouvons pas avoir un comportement asynchrone dans un event handler. 
Nous allons donc devoir créer un effet qui va écouter les changements d'une signal `last_click_on` et 
faire la requête au serveur à chaque fois que cette signal change.
L'apparence de la case sera gérée par un changement d'état dans le store.

```rust

#[composant]
fn game() -> impl IntoView {
  // [...]
    let last_click_on = RwSignal::new(-1);

    let last_click_result = Resource::new(
        move || last_click_on.get(),
        |position| reveal_from_server(position),
    );

    Effect::new(move || {
        match last_click_result.get() {
            Some(Ok(data)) => {
                last_click_result.set(None);
                let mut rows = state.get().rows;
                data.iter().for_each(|(idx, new_case)| {
                    if new_case.is_mine() {
                        game_status.set(GameStatus::Lost);
                    }

                    rows[*idx].case = new_case.clone();
                });
                console_log(&format!("length {}", data.len()));

                state.rows().set(rows);
            }
            _ => console_log("no data from last click"),
        };
    });

  view! {
    // [...]
   
        <Cell
          case=idx_case.get().case.clone()
          on:click = move |_| {
            console_log(&format!("click on cell {}", idx_case.get().idx));
            // au lieu de faire la requête ici, on met à jour selected_case
            last_click_on.set(idx_case.get().idx as isize);
          }
        />
    // [...]
  }
}
```

## Et la suite ?

A vous de jouer ! Voici quelques idées d'améliorations :
- gérer le clic droit pour poser un drapeau (on:contextmenu)
- agrémenter la fin de partie (victoire / défaite)
- ajouter un counter de mines restantes (via signal ou store)
- pouvoir changer la taille de la grille et le nombre de mines
- pouvoir avoir plusieurs parties en cours (modifier le state côté serveur
  ou passer par une base de données)
