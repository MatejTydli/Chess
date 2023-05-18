use crate::ChessMove;
use crate::Color;
use crate::Square;

/// represent type of piece
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Represent a piece with type and color.
/// Doesn't hold any information about their position on the [crate::Board].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    /// Create new [Piece].
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }
}
