use crate::ChessMove;
use crate::Color;
use crate::File;
use crate::Piece;
use crate::PieceType;
use crate::Rank;
use crate::Square;

/// [Board] stores position and history of position.
/// Position is represent by array of [Option]<[Piece]>, with unchangeable size of 8x8.
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pos: [[Option<Piece>; 8]; 8],
    history: Vec<[[Option<Piece>; 8]; 8]>,
    turn: Color,
}

impl Board {
    /// setup [Board] position to deaufult chess position
    pub fn deafult() -> Board {
        let army = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        let mut board = Board::empty(Color::White);

        for i in 0..8 {
            *board.get_mut(Square::new(
                Rank::try_from_usize(0).unwrap(),
                File::try_from_usize(i).unwrap(),
            )) = Some(Piece {
                piece_type: army[i],
                color: Color::White,
            });

            *board.get_mut(Square::new(
                Rank::try_from_usize(7).unwrap(),
                File::try_from_usize(i).unwrap(),
            )) = Some(Piece {
                piece_type: army[i],
                color: Color::Black,
            });
        }

        board
    }

    /// return empty [Board] (invalid for starting [crate::Game]) use for building chess position
    pub fn empty(turn: Color) -> Self {
        let pos = [[None; 8]; 8];

        Board {
            pos,
            history: Vec::new(),
            turn,
        }
    }

    /// Place(or replace) piece on board
    pub fn place_piece(&mut self, index: Square, piece: Option<Piece>) -> &mut Board {
        *self.get_mut(index) = piece;
        self
    }

    /// Remove piece on board
    pub fn remove_piece(&mut self, index: Square) -> &mut Board {
        *self.get_mut(index) = None;
        self
    }

    /// get [`Option<Square>`] from reference on piece
    pub fn get_square(&self, piece: &Option<Piece>) -> Option<Square> {
        for i in 0..64usize {
            let sq = Square(i);
            if std::ptr::eq(self.get(sq), piece) {
                return Some(sq);
            }
        }
        None
    }

    /// get reference to specific [`Option<Piece>`] on the [Board]
    pub fn get(&self, index: Square) -> &Option<Piece> {
        &self.pos[index.0 / 8][index.0 % 8]
    }

    /// get mutable reference to specific [Option<Piece>] on the [Board]
    pub(crate) fn get_mut(&mut self, index: Square) -> &mut Option<Piece> {
        &mut self.pos[index.0 / 8][index.0 % 8]
    }

    /// get reference to specific rank on the [Board]
    pub fn get_rank(&self, rank: Rank) -> &[Option<Piece>; 8] {
        &self.pos[rank.to_usize()]
    }

    /// get mutable reference to specific rank on the [Board]
    pub(crate) fn get_mut_rank(&mut self, rank: Rank) -> &mut [Option<Piece>; 8] {
        &mut self.pos[rank.to_usize()]
    }

    /// Iterate over whole [Board] with reference.
    pub fn iter(&self) -> std::vec::IntoIter<&Option<Piece>> {
        let mut all = Vec::new();

        for rank in &self.pos {
            for piece in rank {
                all.push(piece);
            }
        }

        all.into_iter()
    }

    /// Iterate over whole [Board] with mutable reference.
    pub(crate) fn iter_mut(&mut self) -> std::vec::IntoIter<&mut Option<Piece>> {
        let mut all = Vec::new();

        for rank in &mut self.pos {
            for piece in rank.iter_mut() {
                all.push(piece);
            }
        }

        all.into_iter()
    }

    /// Pre-function to square move functions.
    /// Switches perspective of piece by piece color.
    fn move_piece(
        &self,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
        white: fn(&Square, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str>,
        black: fn(&Square, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str>,
    ) -> Result<ChessMove, &'static str> {
        if let Some(p) = piece {
            let sq = self.get_square(piece).unwrap();
            if p.color == Color::White {
                white(&sq, mul, promo)
            } else {
                black(&sq, mul, promo)
            }
        } else {
            Err("Piece not found on the board.")
        }
    }

    pub(crate) fn up(
        &self,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        self.move_piece(piece, mul, promo, Square::up, Square::down)
    }

    pub(crate) fn down(
        &self,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        self.move_piece(piece, mul, promo, Square::down, Square::up)
    }

    pub(crate) fn right(
        &self,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        self.move_piece(piece, mul, promo, Square::right, Square::left)
    }

    pub(crate) fn left(
        &self,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        self.move_piece(piece, mul, promo, Square::left, Square::right)
    }

    pub(crate) fn up_right(
        &self,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        self.move_piece(piece, mul, promo, Square::up_right, Square::down_left)
    }

    pub(crate) fn up_left(
        &self,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        self.move_piece(piece, mul, promo, Square::up_left, Square::down_right)
    }

    pub(crate) fn down_right(
        &self,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        self.move_piece(piece, mul, promo, Square::down_right, Square::up_left)
    }

    pub(crate) fn down_left(
        &self,
        piece: &Option<Piece>,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        self.move_piece(piece, mul, promo, Square::down_left, Square::up_right)
    }

    /*
    pub fn is_valid(turn: Color) -> bool {
        todo!()
    }

    pub(crate) fn is_check() -> bool {
        todo!()
    }

    pub(crate) fn is_check_mate() -> bool {
        todo!()
    }

    pub(crate) fn is_draw() -> bool {
        todo!()
    }
    */
}
