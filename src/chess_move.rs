use crate::Board;
use crate::PieceType;
use crate::Square;
use crate::GenMask;
use crate::gen_moves;

/// Represent a chess move, stores info about move (start, destination, promotion).   
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChessMove {
    start: Square,
    pub(crate) dest: Square,
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

    pub fn is_valid(&self, board: &Board) -> bool {
        for valid_move in gen_moves(board, &GenMask::Both) {
            if &valid_move == self {
                return true;
            }
        }
        false
    }

    /*
    , promo: Option<PieceType>, spec_req: fn(&mut Board)
     */
}
