use leptos::prelude::*;
use crate::components::game_over_overlay::GameOverOverlay;
use crate::components::GameStatus;

#[component]
pub fn Game() -> impl IntoView {
    let game_status = RwSignal::new(GameStatus::Lost);
    view! {
      <GameOverOverlay state=game_status />
    }
}
