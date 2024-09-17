//! The moust interesting and the moust complex(spaghetti), part of the library.

use crate::board;
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
    /// Helper for creating [ChessMove]s inside gen_moves_raw.
    pub(crate) fn gen_moves_raw_temp(
        &self,
        piece_raw: &Option<Piece>,
        piece: &Piece,
        moves: &mut Vec<ChessMove>,
        createfn: fn(
            &Board,
            &Option<Piece>,
            i32,
            Option<PieceType>,
        ) -> Result<ChessMove, &'static str>,
    ) {
        for m in 1..=8 {
            if let Ok(pot) = createfn(self, piece_raw, m as i32, None) {
                if let Some(p) = self.get(pot.dest) {
                    if p.color != piece.color {
                        moves.push(pot);
                    }
                    break;
                } else {
                    moves.push(pot);
                }
            }
        }
    }

    /// Create valid moves from [Board], depends on [Mask].
    pub fn gen_moves(&self, mask: Mask) -> Vec<ChessMove> {
        let mut moves = Vec::new();

        let mut board = self.clone();
        board.clear_history();

        for mv in self.gen_moves_raw(mask) {
            let mut test_board = board.clone();
            test_board.make_move(mv);

            let mut check = false;
            for tmv in test_board.gen_moves_raw(mask) {
                if let Some(p) = test_board.get(tmv.dest) {
                    if p.piece_type == PieceType::King {
                        check = true;
                        break;
                    }
                }
            }

            if !check {
                moves.push(mv);
            }
        }

        let mut moves_rm_dp = Vec::new();
        for m in moves {
            if !moves_rm_dp.contains(&m) {
                moves_rm_dp.push(m);
            }
        }

        moves_rm_dp
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
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::up);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::down);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::left);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::right);
                        }
                        crate::PieceType::Bishop => {
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::up_left);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::up_right);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::down_left);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::down_right);
                        }
                        crate::PieceType::Queen => {
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::up);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::down);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::left);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::right);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::up_left);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::up_right);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::down_left);
                            self.gen_moves_raw_temp(piece_raw, piece, &mut moves, ChessMove::down_right);
                        }
                        crate::PieceType::King => {
                            for m in [
                                ChessMove::up(self, piece_raw, 1, None),
                                ChessMove::down(self, piece_raw, 1, None),
                                ChessMove::left(self, piece_raw, 1, None),
                                ChessMove::right(self, piece_raw, 1, None),
                                ChessMove::up_left(self, piece_raw, 1, None),
                                ChessMove::up_right(self, piece_raw, 1, None),
                                ChessMove::down_left(self, piece_raw, 1, None),
                                ChessMove::down_right(self, piece_raw, 1, None),
                            ] {
                                if let Ok(pot) = m {
                                    if let Some(p) = self.get(pot.dest) {
                                        if p.color != piece.color {
                                            moves.push(pot);
                                        }
                                    } else {
                                        moves.push(pot);
                                    }
                                }
                            }

                            if piece.color == Color::White {
                                if !self.king_moved_w {
                                    // short castle
                                    if !self.right_rook_moved_w {
                                        if *self.get(Square::F1) == None
                                            && *self.get(Square::G1) == None
                                        {
                                            moves.push(
                                                ChessMove::left(self, piece_raw, 2, None).unwrap(),
                                            );
                                        }
                                    }
                                    // long castle
                                    if !self.left_rook_moved_w {
                                        if *self.get(Square::D1) == None
                                            && *self.get(Square::C1) == None
                                        {
                                            moves.push(
                                                ChessMove::right(self, piece_raw, 2, None).unwrap(),
                                            );
                                        }
                                    }
                                }
                            } else {
                                if !self.king_moved_b {
                                    // short castle
                                    if !self.right_rook_moved_b {
                                        if *self.get(Square::F8) == None
                                            && *self.get(Square::F8) == None
                                        {
                                            moves.push(
                                                ChessMove::right(self, piece_raw, 2, None).unwrap(),
                                            );
                                        }
                                    }
                                    // long castle
                                    if !self.left_rook_moved_b {
                                        if *self.get(Square::D8) == None
                                            && *self.get(Square::C8) == None
                                        {
                                            moves.push(
                                                ChessMove::left(self, piece_raw, 2, None).unwrap(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        crate::PieceType::Knight => {
                            let pos = self.get_square(piece_raw).unwrap();
                            for pot_m in [
                                ChessMove::new(pos, Square(pos.0.abs_diff(17)), None),
                                ChessMove::new(pos, Square(pos.0.abs_diff(15)), None),
                                ChessMove::new(pos, Square(pos.0 + 17), None),
                                ChessMove::new(pos, Square(pos.0 + 15), None),
                                ChessMove::new(pos, Square(pos.0 + 10), None),
                                ChessMove::new(pos, Square(pos.0.abs_diff(6)), None),
                                ChessMove::new(pos, Square(pos.0 + 6), None),
                                ChessMove::new(pos, Square(pos.0.abs_diff(10)), None),
                            ] {
                                if pot_m.dest.0 > 63 {
                                    continue;
                                } else if pos
                                    .get_file()
                                    .unwrap()
                                    .to_usize()
                                    .abs_diff(pot_m.dest.get_file().unwrap().to_usize())
                                    > 3
                                {
                                    continue;
                                }

                                if let Some(m) = self.get(pot_m.dest) {
                                    if m.color != piece.color {
                                        moves.push(pot_m);
                                    }
                                } else {
                                    moves.push(pot_m);
                                }
                            }
                        }
                    }
                }
            }
        }

        moves
    }
}
