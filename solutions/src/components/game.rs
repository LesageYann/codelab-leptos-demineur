use leptos::prelude::*;

#[component]
pub fn Game() -> impl IntoView {
    let is_game_over = RwSignal::new(true);
    view! {
      {move || {
        if is_game_over.get() {
          view! {
            <div class="overlay">
              <div class="overlay-container">
                <div class="message">"Perdu"</div>
                <button on:click=move |_| {is_game_over.set(false)}>
                    "Rejouer"
                </button>
              </div>
            </div>
          }.into_any()
        } else {
          view! {}.into_any()
        }
      }}
    }
}
