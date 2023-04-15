use crate::Board;
//use crate::Timer;

/// represent a player or color of their pieces
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black
}

/// Game struct is for managing board and timer at once
/// 
pub struct Game {
    board: Board,
    turn: Color
    // timer: Timer,
}