//! The moust interesting and the moust complex, part of the library.

use crate::Board;
use crate::ChessMove;
use crate::Color;

/// Masks represents differnet types of generating moves via gen_move() function.
#[derive(Clone, Copy, PartialEq)]
pub enum GenMask {
    White,
    Black,
    Both,
}

impl GenMask {
    /// Checks if [Color] is not filtered out.
    pub fn is_eq(&self, other: Color) -> bool {
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
pub fn gen_moves(board: &Board, color: &GenMask) -> Vec<ChessMove> {
    todo!()
}

/// Create valid moves from [Board], but not take checks in to account. Depends on [GenMask].
fn gen_moves_raw(board: &mut Board, color: &GenMask) -> Vec<ChessMove> {
    let mut moves = Vec::new();

    for place in board.iter() {
        if let Some(piece) = place {
            match piece.piece_type {
                crate::PieceType::Pawn => {
                    
                }
                crate::PieceType::Knight => todo!(),
                crate::PieceType::Bishop => todo!(),
                crate::PieceType::Rook => todo!(),
                crate::PieceType::Queen => todo!(),
                crate::PieceType::King => todo!(),
            }
        }
    }

    moves
}
