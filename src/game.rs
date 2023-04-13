use crate::board;
use crate::timer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black
}

pub struct Game {
    board: board::Board,
    // timer: Timer,
}