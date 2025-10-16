pub mod game;
mod game_over_overlay;
mod cell;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameStatus {
    Playing,
    New,
    Lost,
}