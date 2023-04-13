use crate::file;
use crate::rank;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pos: (file::File, rank::Rank),
    // piece: Option<Piece>
}
