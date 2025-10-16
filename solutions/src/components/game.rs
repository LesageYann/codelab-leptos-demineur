use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use reactive_stores::Store;
use crate::components::cell::Cell;
use crate::components::game_over_overlay::GameOverOverlay;
use crate::components::GameStatus;
use crate::model::board::Case;
use crate::model::board_store::{generate_new_grid, GameState, GameStateStoreFields};

#[component]
pub fn Game() -> impl IntoView {
    let state = Store::new(GameState::new());
    let game_status = RwSignal::new(GameStatus::New);

    async fn reset_grid(game_status: RwSignal<GameStatus>) -> Option<Vec<Case>> {
        console_log("Resetting grid");
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
        match new_grid.get() {
            Some(Some(data)) => {
                new_grid.set(None);
                let formatted_cases = state.get().format_cases_to_index_cases(data);
                state.rows().set(formatted_cases);
            }
            _ => {}
        };
    });

    Effect::new(move || {
        match game_status.get() {
            GameStatus::New => new_grid.refetch(),
            _ => {}
        };
    });
    view! {
        <div class="square">
            <div class="grid" style="grid-template-columns: repeat(10, 1fr); grid-template-rows: repeat(10, 1fr); ">
               {move || state.rows()
                   .get()
                    .iter()
                    .enumerate()
                    .map(|(idx, case)| {
                        let case_cloned = case.clone();
                        view! {
                          <Cell
                            case=case_cloned.case
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
