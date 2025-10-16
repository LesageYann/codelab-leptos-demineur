use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use crate::components::game_over_overlay::GameOverOverlay;
use crate::components::GameStatus;
use crate::model::board::Case;
use crate::model::board_store::generate_new_grid;

#[component]
pub fn Game() -> impl IntoView {
    let game_status = RwSignal::new(GameStatus::Lost);
    let game_status = RwSignal::new(GameStatus::New);

    async fn reset_grid(game_status: RwSignal<GameStatus>) -> Option<Vec<Case>> {
        if game_status.get() != GameStatus::New {
            return None;
        }
        game_status.set(GameStatus::Playing);
        match generate_new_grid().await {
            Ok(data) => Some(data),
            Err(err) => {
                console_log(&format!("Error generating new grid: {:?}", err));
                None
            }
        }
    }

    let new_grid = Resource::new(move || game_status, |refresh| reset_grid(refresh));

    Effect::new(move || {
        match game_status.get() {
            GameStatus::New => new_grid.refetch(),
            _ => {}
        };
    });
    view! {
      <GameOverOverlay state=game_status />
    }
}
