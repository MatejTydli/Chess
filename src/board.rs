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
    pub(crate) king_moved_w: bool,
    pub(crate) left_rook_moved_w: bool,
    pub(crate) right_rook_moved_w: bool,
    pub(crate) king_moved_b: bool,
    pub(crate) left_rook_moved_b: bool,
    pub(crate) right_rook_moved_b: bool,
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
            PieceType::King,
            PieceType::Queen,
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
                color: Color::Black,
            });

            *board.get_mut(Square::new(
                Rank::try_from_usize(7).unwrap(),
                File::try_from_usize(i).unwrap(),
            )) = Some(Piece {
                piece_type: army[i],
                color: Color::White,
            });
        }

        board.pos[1] = [Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }); 8];

        board.pos[6] = [Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }); 8];

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
            king_moved_w: false,
            left_rook_moved_w: false,
            right_rook_moved_w: false,
            king_moved_b: false,
            left_rook_moved_b: false,
            right_rook_moved_b: false,
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
    pub fn get_from_previous(&self, index: Square) -> &Option<Piece> {
        &self.history[self.history.len() - 1][index.0 / 8][index.0 % 8]
    }

    /// Get mutable reference to specific [Option<Piece>] on the [Board].
    fn get_mut(&mut self, index: Square) -> &mut Option<Piece> {
        &mut self.pos[index.0 / 8][index.0 % 8]
    }

    /// Get reference to specific rank on the [Board].
    pub fn get_mut_rank(&mut self, rank: Rank) -> &mut [Option<Piece>; 8] {
        &mut self.pos[rank.to_usize()]
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

    // Iterate over whole [Board] with mutable reference.
    // pub(crate) fn iter_mut(&mut self) -> std::vec::IntoIter<&mut Option<Piece>> {
    //     let mut all = Vec::new();

    //     for rank in &mut self.pos {
    //         for piece in rank.iter_mut() {
    //             all.push(piece);
    //         }
    //     }

    //     all.into_iter()
    // }

    /// Make move and save new position to history.
    /// Also handling exceptions like en passant, castle and apwn promotinon.
    pub fn make_move(&mut self, mv: ChessMove) {
        self.history.push(self.pos);

        let mut maybe_piece = *self.get(mv.start);
        let mut en_passant = false;
        let mut castle_short_w = false;
        let mut castle_short_b = false;
        let mut castle_long_w = false;
        let mut castle_long_b = false;
        if let Some(piece) = maybe_piece {
            if mv.start.get_rank().unwrap()
                == if piece.color == Color::White {
                    Rank::Fifth
                } else {
                    Rank::Fourth
                }
                && mv.dest.get_rank().unwrap()
                    == if piece.color == Color::White {
                        Rank::Sixth
                    } else {
                        Rank::Third
                    }
                && *self.get(mv.dest) == None
                && piece.piece_type == PieceType::Pawn
            {
                // en passant
                en_passant = true;
            }

            if let Some(promo) = mv.promo {
                maybe_piece = Some(Piece::new(promo, piece.color));
            }

            match piece.piece_type {
                PieceType::Rook => {
                    // long castle
                    // lw
                    if !self.left_rook_moved_w && mv.start == Square::A1 {
                        self.left_rook_moved_w = true;
                    }
                    // lb
                    if !self.right_rook_moved_b && mv.start == Square::H8 {
                        self.right_rook_moved_b = true;
                    }
                    // short castle
                    // rw
                    if !self.right_rook_moved_w && mv.start == Square::H1 {
                        self.right_rook_moved_w = true;
                    }
                    // rb
                    if !self.left_rook_moved_b && mv.start == Square::A8 {
                        self.left_rook_moved_b = true;
                    }
                }
                PieceType::King => {
                    // w castle logic for moving rook
                    if !self.king_moved_w {
                        if mv.dest == Square::G1 {
                            castle_short_w = true;
                        } else if mv.dest == Square::C1 {
                            castle_long_w = true;
                        }
                    }
                    // w castle logic for moving rook
                    if !self.king_moved_b {
                        if mv.dest == Square::G8 {
                            castle_short_b = true;
                        } else if mv.dest == Square::C8 {
                            castle_long_b = true;
                        }
                    }

                    if piece.color == Color::White {
                        self.king_moved_w = true;
                    } else {
                        self.king_moved_b = true;
                    }
                }
                _ => {}
            }
        }

        self.place_piece(mv.dest, maybe_piece);
        self.remove_piece(mv.start);
        // en passant
        if en_passant {
            self.place_piece(
                ChessMove::down(&self, self.get(mv.dest), 1, None)
                    .unwrap()
                    .dest,
                None,
            );
        }
        // castle
        if castle_short_w {
            self.place_piece(Square::F1, *self.get(Square::H1));
            self.remove_piece(Square::H1);
        }
        if castle_long_w {
            self.place_piece(Square::D1, *self.get(Square::A1));
            self.remove_piece(Square::A1);
        }
        if castle_short_b {
            self.place_piece(Square::F8, *self.get(Square::H8));
            self.remove_piece(Square::H8);
        }
        if castle_long_b {
            self.place_piece(Square::D8, *self.get(Square::A8));
            self.remove_piece(Square::A8);
        }
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
