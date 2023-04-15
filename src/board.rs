use crate::Color;
use crate::File;
use crate::Piece;
use crate::PieceType;
use crate::Rank;
use crate::Square;

/// [Board] stores position and history of position
/// position is represent by array of [Option<Piece>]
/// with unchangeable size of 8x8
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pos: [[Option<Piece>; 8]; 8],
    history: Vec<Board>,
}

impl Board {
    /// setup [Board] position to deaufult chess position
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

        let mut board = Board::empty();

        let rank_1 = board.get_mut_rank(Rank::First);

        for i in 0..8 {
            rank_1[i] = Some(Piece {
                piece_type: army[i],
                color: Color::White,
            });
        }

        let rank_8 = board.get_mut_rank(Rank::Eighth);

        for i in 0..8 {
            rank_8[i] = Some(Piece {
                piece_type: army[i],
                color: Color::Black,
            });
        }

        board
    }

    /// return empty [Board] (invalid for starting [crate::Game]) use for building chess position
    fn empty() -> Self {
        let pos = [[None; 8]; 8];

        Board {
            pos,
            history: Vec::new(),
        }
    }

    /// get reference to specific [Option<Piece>] on the [Board]
    pub fn get(&self, index: Square) -> &Option<Piece> {
        &self.pos[index.0 / 8][index.0 % 8]
    }

    /// get mutable reference to specific [Option<Piece>] on the [Board]
    pub fn get_mut(&mut self, index: Square) -> &mut Option<Piece> {
        &mut self.pos[index.0 / 8][index.0 % 8]
    }

    /// get reference to specific rank on the [Board]
    pub fn get_rank(&self, rank: Rank) -> &[Option<Piece>; 8] {
        &self.pos[rank.to_usize()]
    }

    /// get mutable reference to specific rank on the [Board]
    pub fn get_mut_rank(&mut self, rank: Rank) -> &mut [Option<Piece>; 8] {
        &mut self.pos[rank.to_usize()]
    }

    /// iterate over whole [Board] with &mut
    pub fn iter(&mut self) -> std::vec::IntoIter<&mut Option<Piece>> {
        let mut all = Vec::new();

        for rank in &mut self.pos {
            for piece in rank.iter_mut() {
                all.push(piece);
            }
        }

        all.into_iter()
    }

    /// convert string to [Board]
    pub fn from_str(str: &str) -> Board {
        todo!()
    }

    pub fn is_valid(turn: Color) -> bool {
        todo!()
    }

    pub fn is_check() -> bool {
        todo!()
    }

    pub fn is_check_mate() -> bool {
        todo!()
    }

    pub fn is_draw() -> bool {
        todo!()
    }
}

// old indexing replace by Board::get() and Board::get_mut()
// /// implement Index operator directly for [Board] using [Rank] and [File]
// impl std::ops::Index<(Rank, File)> for Board {
//     type Output = Option<Piece>;
//     fn index(&self, index: (Rank, File)) -> &Self::Output {
//         &self.pos[index.0.to_usize()][index.1.to_usize()]
//     }
// }
// /// implement IndexMut operator directly for [Board] using [Rank] and [File]
// impl std::ops::IndexMut<(Rank, File)> for Board {
//     fn index_mut(&mut self, index: (Rank, File)) -> &mut Self::Output {
//         &mut self.pos[index.0.to_usize()][index.1.to_usize()]
//     }
// }
// /// implement Index operator directly for [Board] using [Square]
// impl std::ops::Index<Square> for Board {
//     type Output = Option<Piece>;
//     fn index(&self, index: Square) -> &Self::Output {
//         &self.pos[index.0 / 8 + 1][index.0 % 8]
//     }
// }
// /// implement IndexMut operator directly for [Board] using [Square]
// impl std::ops::IndexMut<Square> for Board {
//     fn index_mut(&mut self, index: Square) -> &mut Self::Output {
//         &mut self.pos[index.0 / 8 + 1][index.0 % 8]
//     }
// }
