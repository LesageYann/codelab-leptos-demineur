use leptos::leptos_dom::logging::console_log;
use leptos::prelude::{ServerFnError, Update, expect_context};
use leptos::server;
use reactive_stores::Store;
use crate::model::board::{Board, Case, CaseState};
use crate::model::server_state::ServerState;

#[derive(Store, Debug, Clone)]
pub struct GameState {
    #[store(key:String = |case| case.get_key())]
    pub rows: Vec<IndexCase>,
}

#[derive(Debug, Store, Clone)]
pub struct IndexCase {
    pub idx: usize,
    pub case: Case,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            rows: vec![],
        }
    }

    async fn generate_grid() -> Vec<IndexCase> {
        Board::new(10)
            .iter()
            .enumerate()
            .map(|(idx, case)| IndexCase { idx, case: case.clone() })
            .collect::<Vec<IndexCase>>()
    }

    pub fn format_cases_to_index_cases(&mut self, data: Vec<Case>) -> Vec<IndexCase>{
       data.iter()
            .enumerate()
            .map(|(idx, case)| IndexCase { idx, case: case.clone() })
            .collect()
    }

}

impl IndexCase {
    pub fn get_key(&self) -> String {
        self.idx.to_string() + &self.case.is_flagged().to_string() + &self.case.is_revealed().to_string()
    }
}

impl Update for IndexCase {
    type Value = IndexCase;

    fn try_maybe_update<U>(&self, fun: impl FnOnce(&mut Self::Value) -> (bool, U)) -> Option<U> {
        let (changed, res) = fun(&mut self.clone());
        if changed {
            Some(res)
        } else {
            None
        }
    }
}

#[server]
pub async fn generate_new_grid() -> Result<Vec<Case>,  ServerFnError> {
    use std::sync::{Arc, Mutex};

    let state = expect_context::<Arc<Mutex<ServerState>>>();
    let mut server_state = state.lock().unwrap();
    server_state.board = Board::new(10);
    Ok([Case::Number(0, CaseState::Hidden); 100].to_vec())
}

#[server]
pub async fn reveal_from_server( position: isize) -> Result<Vec<(usize, Case)>,  ServerFnError> {
    use std::sync::{Arc, Mutex};
    if position < 0 || position as usize >= 100 {
        return Err(ServerFnError::ServerError("Invalid position".into()));
    }

    let state = expect_context::<Arc<Mutex<ServerState>>>();
    let mut server_state = state.lock().unwrap();
    Ok(server_state.board.reveal(position as usize))
}