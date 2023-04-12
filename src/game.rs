use crate::board;
use crate::timer;

pub enum Color {
    White,
    Black
}

pub struct Game {
    board: board::Board,
    // timer: Timer,
}