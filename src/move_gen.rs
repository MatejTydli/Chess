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

impl Board {
    /// Create valid moves from [Board], depends on [GenMask].
    pub fn gen_moves(&self, mask: &GenMask) -> Vec<ChessMove> {
        todo!()
    }

    /// Create valid moves from [Board], but not take checks in to account. Depends on [GenMask].
    fn gen_moves_raw(&self, mask: GenMask) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        for u in 0..64usize {
            let square = Square(u);
            let piece_raw = self.get(square);

            if let Some(piece) = piece_raw {
                if mask.compare(piece.color) {
                    let opponent_color = piece.color.inverse();

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
                                } else {
                                    // # pawn takes en passatns
                                    if let Ok(left_check) =
                                        ChessMove::left(self, piece_raw, 1, promo)
                                    {
                                        let maybe_en_target = self.get(left_check.dest);
                                        if let Some(en_target) = maybe_en_target {
                                            if let Some(last_target) = self.get_last_from_history(/*continue here with*/) {
                                                
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
                                } else {
                                    // # pawn takes en passatns
                                }
                            }
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
}
