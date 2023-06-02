use crate::Board;
use crate::Color;
use crate::GenMask;
use crate::Piece;
use crate::PieceType;
use crate::Square;

/// Represent a chess move, stores info about move (start, destination, promotion).   
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChessMove {
    pub(crate) start: Square,
    pub(crate) dest: Square,
    pub(crate) promo: Option<PieceType>,
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

    /// Check if move is valid to current position on board.
    pub fn is_valid(&self, board: &Board) -> bool {
        board.gen_moves(&GenMask::Both).contains(self)
    }

    /// Pre-function to square move functions.
    /// Switches perspective of piece by piece color.
    fn create(
        board: &Board,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
        white: fn(&Square, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str>,
        black: fn(&Square, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str>,
    ) -> Result<ChessMove, &'static str> {
        if let Some(p) = piece {
            let sq = board.get_square(piece).unwrap();
            if p.color == Color::White {
                white(&sq, mul, promo)
            } else {
                black(&sq, mul, promo)
            }
        } else {
            Err("Piece not found on the board.")
        }
    }

    /// Shortcut function for creating moves on board.
    pub(crate) fn up(
        board: &Board,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        ChessMove::create(board, piece, mul, promo, Square::up, Square::down)
    }

    /// Shortcut function for creating moves on board.
    pub(crate) fn down(
        board: &Board,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        ChessMove::create(board, piece, mul, promo, Square::down, Square::up)
    }

    /// Shortcut function for creating moves on board.
    pub(crate) fn right(
        board: &Board,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        ChessMove::create(board, piece, mul, promo, Square::right, Square::left)
    }

    /// Shortcut function for creating moves on board.
    pub(crate) fn left(
        board: &Board,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        ChessMove::create(board, piece, mul, promo, Square::left, Square::right)
    }

    /// Shortcut function for creating moves on board.
    pub(crate) fn up_right(
        board: &Board,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        ChessMove::create(
            board,
            piece,
            mul,
            promo,
            Square::up_right,
            Square::down_left,
        )
    }

    /// Shortcut function for creating moves on board.
    pub(crate) fn up_left(
        board: &Board,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        ChessMove::create(
            board,
            piece,
            mul,
            promo,
            Square::up_left,
            Square::down_right,
        )
    }

    /// Shortcut function for creating moves on board.
    pub(crate) fn down_right(
        board: &Board,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        ChessMove::create(
            board,
            piece,
            mul,
            promo,
            Square::down_right,
            Square::up_left,
        )
    }

    /// Shortcut function for creating moves on board.
    pub(crate) fn down_left(
        board: &Board,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        ChessMove::create(
            board,
            piece,
            mul,
            promo,
            Square::down_left,
            Square::up_right,
        )
    }
}
