use crate::Board;
use crate::PieceType;
use crate::Square;
// use crate::MoveGen;

/// Represent a chess move. [crate::Piece] from one [Square] can move to destination [Square].
/// This stores stores info about move.   
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChessMove {
    start: Square,
    dest: Square,
    promo: Option<PieceType>,
}

impl ChessMove {
    /// Create new [ChessMove].
    pub fn new(start: Square, destination: Square, promotion: Option<PieceType>) -> ChessMove {
        ChessMove {
            start,
            dest: destination,
            promo: promotion,
        }
    }

    // pub fn is_valid(&self, board: &Board) -> bool {
    //     for valid_move in MoveGen::new(&board).gen_valid() {
    //         if valid_move == self {
    //             return true;
    //         }
    //     }

    //     false
    // }
}
