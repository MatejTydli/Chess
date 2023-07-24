//! The moust interesting and the moust complex(spaghetti), part of the library.

use crate::Board;
use crate::ChessMove;
use crate::Color;
use crate::Piece;
use crate::PieceType;
use crate::Rank;
use crate::Square;

/// Masks represents differnet types of generating moves via gen_move() function.
#[derive(Clone, Copy, PartialEq)]
pub enum Mask {
    White,
    Black,
    Both,
}

impl Mask {
    /// Checks if [Color] is not filtered out.
    fn compare(&self, other: Color) -> bool {
        if *self == Mask::Both
            || (*self == Mask::White && other == Color::White)
            || (*self == Mask::Black && other == Color::Black)
        {
            return true;
        }

        false
    }

    /// Inverts [Mask].
    pub fn inverse(&mut self) {
        if *self == Mask::White {
            *self = Mask::Black;
        } else {
            *self = Mask::White;
        }
    }
}

impl Board {
    /// Create valid moves from [Board], depends on [Mask].
    pub fn gen_moves(&self, mask: Mask) -> Vec<ChessMove> {
        self.gen_moves_raw(mask) // only for testing, later added check (and mate) logic here
    }

    /// Create valid moves from [Board], but not take checks in to account. Depends on [Mask].
    fn gen_moves_raw(&self, mask: Mask) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        for u in 0..64usize {
            let square = Square(u);
            let piece_raw = self.get(square);

            if let Some(piece) = piece_raw {
                if mask.compare(piece.color) {
                    match piece.piece_type {
                        crate::PieceType::Pawn => {
                            // decide if pawn stand on sevent or second rank (depends on color)
                            let is_on_start = square.get_rank().unwrap()
                                == if piece.color == Color::White {
                                    Rank::Second
                                } else {
                                    Rank::Seventh
                                };

                            let is_on_end = square.get_rank().unwrap()
                                == if piece.color == Color::White {
                                    Rank::Seventh
                                } else {
                                    Rank::Second
                                };

                            // set pawn promo from self.pawn_promo
                            let mut promo = match is_on_end {
                                true => Some(self.pawn_promo),
                                false => None,
                            };

                            // # pawn move up normal
                            // pawn up (x1) move can be unwraped directly,
                            // beacuse pawn can't get outside of board (promoting).
                            let pot_up = ChessMove::up(self, piece_raw, 1, promo).unwrap();

                            if *self.get(pot_up.dest) == None {
                                moves.push(pot_up);
                                // # pawn move up double
                                if is_on_start {
                                    moves.push(ChessMove::up(self, piece_raw, 2, promo).unwrap());
                                }
                            }

                            // # pawn takes
                            if let Ok(pot_up_left) = ChessMove::up_left(self, piece_raw, 1, promo) {
                                if let Some(target) = *self.get(pot_up_left.dest) {
                                    if target.color != piece.color {
                                        moves.push(pot_up_left);
                                    }
                                } else if square.get_rank().unwrap()
                                    == if piece.color == Color::White {
                                        Rank::Fifth
                                    } else {
                                        Rank::Fourth
                                    }
                                {
                                    // # pawn takes en passatns
                                    // left check must be right idk why
                                    if let Ok(left_check) =
                                        ChessMove::right(self, piece_raw, 1, promo)
                                    {
                                        let maybe_en_target = self.get(left_check.dest);
                                        if let Some(en_target) = maybe_en_target {
                                            let previous_sq =
                                                ChessMove::up(self, maybe_en_target, 2, None)
                                                    .unwrap()
                                                    .dest;
                                            if let Some(previous_target) =
                                                self.get_from_previous(previous_sq)
                                            {
                                                // takes en passatns
                                                moves.push(pot_up_left);
                                            }
                                        }
                                    }
                                }
                            }
                            if let Ok(pot_up_right) = ChessMove::up_right(self, piece_raw, 1, promo)
                            {
                                if let Some(target) = *self.get(pot_up_right.dest) {
                                    if target.color != piece.color {
                                        moves.push(pot_up_right);
                                    }
                                } else if self.get_square(piece_raw).unwrap().get_rank().unwrap()
                                    == if piece.color == Color::White {
                                        Rank::Fifth
                                    } else {
                                        Rank::Fourth
                                    }
                                {
                                    // # pawn takes en passatns
                                    // right check must be left idk why
                                    if let Ok(right_check) =
                                        ChessMove::left(self, piece_raw, 1, promo)
                                    {
                                        let maybe_en_target = self.get(right_check.dest);
                                        if let Some(en_target) = maybe_en_target {
                                            let previous_sq =
                                                ChessMove::down(self, maybe_en_target, 2, None)
                                                    .unwrap()
                                                    .dest;
                                            if let Some(previous_target) =
                                                self.get_from_previous(previous_sq)
                                            {
                                                moves.push(pot_up_right);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        crate::PieceType::Rook => {
                            // up
                            for i in 1..=8 {
                                if let Ok(pot_up) = ChessMove::up(self, piece_raw, i as i32, None) {
                                    if let Some(p) = self.get(pot_up.dest) {
                                        if p.color != piece.color {
                                            moves.push(pot_up);
                                        } else {
                                            break;
                                        }
                                    } else {
                                        moves.push(pot_up);
                                    }
                                }
                            }

                            // down
                            for i in 1..=8 {
                                if let Ok(pot_down) =
                                    ChessMove::down(self, piece_raw, i as i32, None)
                                {
                                    if let Some(p) = self.get(pot_down.dest) {
                                        if p.color != piece.color {
                                            moves.push(pot_down);
                                        } else {
                                            break;
                                        }
                                    } else {
                                        moves.push(pot_down);
                                    }
                                }
                            }

                            // left
                            for i in 1..=8 {
                                if let Ok(pot_left) =
                                    ChessMove::left(self, piece_raw, i as i32, None)
                                {
                                    if let Some(p) = self.get(pot_left.dest) {
                                        if p.color != piece.color {
                                            moves.push(pot_left);
                                        } else {
                                            break;
                                        }
                                    } else {
                                        moves.push(pot_left);
                                    }
                                }
                            }

                            // right
                            for i in 1..=8 {
                                if let Ok(pot_right) =
                                    ChessMove::right(self, piece_raw, i as i32, None)
                                {
                                    if let Some(p) = self.get(pot_right.dest) {
                                        if p.color != piece.color {
                                            moves.push(pot_right);
                                        } else {
                                            break;
                                        }
                                    } else {
                                        moves.push(pot_right);
                                    }
                                }
                            }
                        }
                        crate::PieceType::Bishop => continue,
                        crate::PieceType::Queen => continue,
                        crate::PieceType::King => continue,
                        crate::PieceType::Knight => continue,
                    }
                }
            }
        }

        moves
    }
}
