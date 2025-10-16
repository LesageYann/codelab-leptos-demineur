use leptos::prelude::*;

#[component]
pub fn Game() -> impl IntoView {
    let mut is_game_over = true;
    if is_game_over {
        view! {
      <div class="overlay">
        <div class="overlay-container">
          <div class="message">"Perdu"</div>
          <button on:click=move |_| {is_game_over = false}>
              "Rejouer"
          </button>
        </div>
      </div>
    }.into_any()
    } else {
        view! {}.into_any()
    }
}

