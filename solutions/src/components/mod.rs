pub mod game;
mod game_over_overlay;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameStatus {
    Playing,
    New,
    Lost,
}