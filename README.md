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
- init (vous êtes ici)
- first-component
- let-s-interact
- be-reactive
- use-component-in-component
- first-effect
- first-cell
- the-grid
- avoid-cloning
- improve-the-game

## Concept Leptos

Leptos propose trois quatres méthodes principales pour démarrer un projet :
- une crate front seule. Le rendu ne pourrat être que client-side-rendering
- un starter actix + leptos. 
Actix est un serveur rust.
Il servira de back-for-front. 
Le rendu sera par defaut en server-side-rendering.
- un starter axum + leptos. Le principe est identique au précédent mais avec axum en backend
- un starter axum + leptos en workspaces séparés.
On aura un serveur axum à part et un front en CSR.

Nous allons choisir la troisième solution pour ce tutoriel.
Le projet généré se composera de 3 fichiers sources : 
- app.rs : la racine de notre future page web. Elle contient :
    - la structure de notre page avec le head et body dans le composant `shell`
    - un exemple de router dans le composant `App`
    - Un troisième composant orienté html
- main.rs : le serveur axum. 
- lib.rs : De la glue entre app.rs et main.rs. Nous n'y toucherons pas

## TODO de l'étape `init`

Si vous êtes à l'aise avec Rust, sentez-vous libre de changer de starter. 
Si vous avez un soucis de connexion, copier la solution. 
Elle contient un dossier vendor avec toutes les dépendances pour une installation hors ligne.
De même, si vous ne souhaitez pas faire le css, vous pouvez copier le style présent dans la solution.

```bash
cargo install cargo-leptos --locked
cargo leptos new --git https://github.com/leptos-rs/start-axum --name demineur
cd demineur 
```

On va tout de suite ajouter les dépendances dont nous aurons besoin

```bash
cargo add reactive_stores rand 
cargo add getrandom -F wasm_js
cargo add serde -F derive
```

Reactive_stores est une librairie de store réactif.
Et serde permettra d'envoyer des données entre axum et leptos.
Rand nous permettra de générer aléatoirement les mines.
Getrandom est une dépendance de rand mais nous allons devoir préciser quelle version utiliser pour le webassembly.
Dans `.cargo/config.toml`, ajoutez

```toml
[target.'cfg(target_arch = "wasm32")']
rustflags = ["--cfg", "getrandom_backend=\"wasm_js\""]
```

Enfin, vous pouvez lancer le projet pour tester l'installation


```bash
cargo leptos watch
```