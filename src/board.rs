use crate::ChessMove;
use crate::Color;
use crate::File;
use crate::Piece;
use crate::PieceType;
use crate::Rank;
use crate::Square;

/// [Board] stores position and history of position.
/// Position is represent by array of [Option<Piece>], with unchangeable size of 8x8.
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pos: [[Option<Piece>; 8]; 8],
    turn: Color,
    /// Record all moves. Doesn't contains current position(Current is in board.pos). 
    history: Vec<[[Option<Piece>; 8]; 8]>,
    /// Sets new [PieceType] of promoted pawn.
    pub pawn_promo: PieceType,
}

impl Board {
    /// Setup [Board] position to deaufult chess position.
    /// Deafult pawn_promo is set to Queen.
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

    /// Return empty [Board] use for building chess position.
    /// Deafult pawn_promo is set to Queen.
    pub fn empty(turn: Color) -> Self {
        let pos = [[None; 8]; 8];

        Board {
            pos,
            history: Vec::new(),
            turn,
            pawn_promo: PieceType::Queen,
        }
    }

    /// Place(or replace) piece on board(doesn't included in history).
    pub fn place_piece(&mut self, index: Square, piece: Option<Piece>) -> &mut Board {
        *self.get_mut(index) = piece;
        self
    }

    /// Remove piece on board(doesn't included in history).
    pub fn remove_piece(&mut self, index: Square) -> &mut Board {
        *self.get_mut(index) = None;
        self
    }

    /// Get [Option<Square>] from reference on piece.
    pub fn get_square(&self, piece: &Option<Piece>) -> Option<Square> {
        for i in 0..64usize {
            let sq = Square(i);
            if std::ptr::eq(self.get(sq), piece) {
                return Some(sq);
            }
        }
        None
    }

    /// Get reference to specific [Option<Piece>] on the [Board].
    pub fn get(&self, index: Square) -> &Option<Piece> {
        &self.pos[index.0 / 8][index.0 % 8]
    }

    /// Get reference to specific [Option<Piece>] on the [Board] from last history record. 
    pub fn get_last_from_history(&self, index: Square) -> &Option<Piece> {
        &self.history[self.history.len() - 1][index.0 / 8][index.0 % 8]
    }

    /// Get mutable reference to specific [Option<Piece>] on the [Board].
    fn get_mut(&mut self, index: Square) -> &mut Option<Piece> {
        &mut self.pos[index.0 / 8][index.0 % 8]
    }

    /// Get reference to specific rank on the [Board].
    pub fn get_rank(&self, rank: Rank) -> &[Option<Piece>; 8] {
        &self.pos[rank.to_usize()]
    }

    /// Get refernce to history.
    pub fn get_history(&self) -> &Vec<[[Option<Piece>; 8]; 8]> {
        &self.history
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

    /// Make move and save new position to history.
    pub fn make_move(&mut self, mv: ChessMove) {
        self.history.push(self.pos);

        let maybe_piece = *self.get(mv.start);
        if let Some(piece) = maybe_piece {
            maybe_piece.unwrap().piece_type = self.pawn_promo;
        }

        self.place_piece(mv.dest, maybe_piece);
        self.place_piece(mv.start, None);
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
