use crate::game;
use crate::Square;

pub struct Board {
    pos: [[Square; 8]; 8],
    history: Vec<Board>,
    turn: game::Color,
}

impl Board {
    // pub fn deafult() -> Board {

    // }

    // pub fn empty() -> Board {

    // }

    // pub fn from_str(FEN_str: &str) -> Board {

    // }

    // pub fn is_valid() -> bool {

    // }

    // pub fn is_check() -> bool {

    // }

    // pub fn is_check_mate() -> bool {

    // }

    // pub fn is_draw() -> bool {

    // }
}
