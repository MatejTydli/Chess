use std::slice::Iter;

use crate::Color;
use crate::File;
use crate::Rank;
use crate::Square;
use crate::Piece;
use crate::rank;

/// Board stores position and history of position
/// position is represent by array of [Option<Piece>]
/// with unchangeable size of 8x8    
pub struct Board {
    pos: [[Option<Piece>; 8]; 8],
    history: Vec<Board>,
}

impl std::ops::Index<(Rank, File)> for Board {
    type Output = Option<Piece>;

    fn index(&self, index: (Rank, File)) -> &Self::Output {
        &self.pos[index.0.to_usize()][index.1.to_usize()]
    }
}

impl std::ops::IndexMut<(Rank, File)> for Board {
    fn index_mut(&mut self, index: (Rank, File)) -> &mut Self::Output {
        &mut self.pos[index.0.to_usize()][index.1.to_usize()]
    }
}

impl Board {
    pub fn deafult() -> Board {
        Board::from_str("")
    }

    pub fn empty() -> Self {
        let pos = [[None; 8]; 8];

        Board {
            pos,
            history: Vec::new(),
        }
    }

    pub fn from_str(FEN_str: &str) -> Board {
        Board::empty()
    }
    
    /// This is by chatGPT so I have no idea... 
    pub fn iter(&mut self) -> impl Iterator<Item = Option<Piece>> + '_ {
        let mut all = [None; 64];
        for (i, rank) in self.pos.iter().enumerate() {
            for (j, piece) in rank.iter().enumerate() {
                all[i * 8 + j] = *piece;
            }
        }
        std::array::IntoIter::new(all)
    }

    // pub fn is_valid(turn: Color) ->  {

    // }

    // pub fn is_check() -> bool {

    // }

    // pub fn is_check_mate() -> bool {

    // }

    // pub fn is_draw() -> bool {

    // }
}

// struct BoardIter(Board);

// impl Iterator for BoardIter {
//     type Item = Option<Piece>;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }
