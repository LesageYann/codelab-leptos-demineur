use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use reactive_stores::Store;
use crate::components::cell::Cell;
use crate::components::game_over_overlay::GameOverOverlay;
use crate::components::GameStatus;
use crate::model::board::Case;
use crate::model::board_store::{generate_new_grid, reveal_from_server, GameState, GameStateStoreFields};

#[component]
pub fn Game() -> impl IntoView {
    let state = Store::new(GameState::new());
    let game_status = RwSignal::new(GameStatus::New);
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
              <For
                    each=move ||  state.rows()
                    key=|idx_case| idx_case.get().get_key()
                    let:idx_case
                >
                    <Cell
                        case=idx_case.get().case.clone()
                        on:click = move |_| {
                            last_click_on.set(idx_case.get().idx as isize);
                        }
                    />
                </For>
            </div>
        </div>
        <GameOverOverlay state=game_status /> 
    }
}
