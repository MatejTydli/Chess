use crate::Color;

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

/// represent a piece with type and color
/// doesn't hold any information about their position on board 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}