//! The moust interesting and the moust complex, part of the library.

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

/// Create valid moves from [Board], depends on [GenMask].
pub fn gen_moves(board: &Board, mask: &GenMask) -> Vec<ChessMove> {
    todo!()
}

/// Create valid moves from [Board], but not take checks in to account. Depends on [GenMask].
fn gen_moves_raw(board: &mut Board, mask: GenMask) -> Vec<ChessMove> {
    let mut moves = Vec::new();

    for u in 0..64usize {
        let square = Square(u);
        let piece_raw = board.get(square);

        if let Some(piece) = piece_raw {
            if mask.compare(piece.color) {
                let opponent_color = piece.color.inverse();

                match piece.piece_type {
                    crate::PieceType::Pawn => {
                        // set pawn promo from board.pawn_promo
                        // if pawn stand on sevent or second rank (depends on color)
                        let mut promo = None;
                        if square.get_rank().unwrap()
                            == if piece.color == Color::White {
                                Rank::Seventh
                            } else {
                                Rank::Second
                            }
                        {
                            promo = Some(board.pawn_promo);
                        }

                        // # pawn move up normal
                        // pawn up (x1) move can be unwraped directly,
                        // beacuse pawn can't get outside of board (promoting).
                        let pot_up = board.up(piece_raw, 1, promo).unwrap();

                        if *board.get(pot_up.dest) == None {
                            moves.push(pot_up);

                            // # pawn move up double
                            let pot_up_d = board.up(piece_raw, 2, promo);
                            if square.get_rank().unwrap()
                                == if piece.color == Color::White {
                                    Rank::Seventh
                                } else {
                                    Rank::Second
                                }
                            {
                                moves.push(pot_up_d.unwrap());
                            }
                        }

                        // # pawn takes
                        if let Ok(pot_up_left) = board.up_left(piece_raw, 1, promo) {
                            if let Some(target) = *board.get(pot_up_left.dest) {
                                if target.color != piece.color {
                                    moves.push(pot_up_left);
                                }
                            }
                        }
                        if let Ok(pot_up_right) = board.up_right(piece_raw, 1, promo) {
                            if let Some(target) = *board.get(pot_up_right.dest) {
                                if target.color != piece.color {
                                    moves.push(pot_up_right);
                                }
                            }
                        }

                        // # pawn takes en passatns
                        // ## continue here
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
