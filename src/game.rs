use crate::level::*;

pub enum GameCommand {
    Up,
    Down,
    Left,
    Right,

    Undo,
    Quit,
}

pub struct Game {
    pub level: Level,
}