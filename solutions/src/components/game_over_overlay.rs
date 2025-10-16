use leptos::prelude::*;
use crate::components::GameStatus;

#[component]
pub fn GameOverOverlay(state: RwSignal<GameStatus>) -> impl IntoView {
    view! {
        {move ||
             if *state.read() == GameStatus::Lost {
                 view! {
                     <div class="overlay">
                        <div class="overlay-container">
                            <div class="message">"Perdu"</div>
                            <button
                                on:click=move |_| {
                                    state.set(GameStatus::New);
                                }
                            >
                                "Rejouer"
                            </button>
                        </div>
                     </div>
                 }.into_any()
             } else {
                 view! {}.into_any()
             }
        }
    }
}