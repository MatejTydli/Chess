//! The moust interesting and the moust complex, part of the library.

use crate::Board;
use crate::ChessMove;
use crate::Color;
use crate::Square;

/// Masks represents differnet types of generating moves via gen_move() function.
#[derive(Clone, Copy, PartialEq)]
pub enum GenMask {
    White,
    Black,
    Both,
}

impl GenMask {
    /// Checks if [Color] is not filtered out.
    fn compare(&self, other: Color) -> bool {
        if *self == GenMask::Both
            || (*self == GenMask::White && other == Color::White)
            || (*self == GenMask::Black && other == Color::Black)
        {
            return true;
        }

        false
    }
}

/// Create valid moves from [Board], depends on [GenMask].
pub fn gen_moves(board: &Board, mask: &GenMask) -> Vec<ChessMove> {
    todo!()
}

/// Create valid moves from [Board], but not take checks in to account. Depends on [GenMask].
fn gen_moves_raw(board: &mut Board, mask: &GenMask) -> Vec<ChessMove> {
    let mut moves = Vec::new();

    for u in 0..64usize {
        let square = Square(u);
        if let Some(piece) = board.get(square) {
            if mask.compare(piece.color) {
                match piece.piece_type {
                    crate::PieceType::Pawn => {
                        // pawn move forward normal
                        // if let Some(target) = board.get(square.up(1, None).unwrap().dest) {

                        // }
                        // pawn move forward double

                        // pawn takes

                        // pawn takes en passatns
                    }
                    crate::PieceType::Knight => todo!(),
                    crate::PieceType::Bishop => todo!(),
                    crate::PieceType::Rook => todo!(),
                    crate::PieceType::Queen => todo!(),
                    crate::PieceType::King => todo!(),
                }
            }
        }
    }

    moves
}
