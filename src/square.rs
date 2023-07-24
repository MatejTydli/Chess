/// Allow unused import crate::Board for using [Board] in documentation.
#[allow(unused_imports)]
use crate::Board;
use crate::ChessMove;
use crate::File;
use crate::PieceType;
use crate::Rank;

/// Represent a index on the [Board].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square(pub(crate) usize);

impl Square {
    /// Possible way to create a [Square].
    /// Consider using predefined constants.
    pub(crate) fn new(rank: Rank, file: File) -> Square {
        Square(rank.to_usize() * 8 + file.to_usize())
    }

    /// Extract [Rank] from [Square], return [Result].
    /// Return [Err] when [Square] is not valid (it's outside the [Board]).
    pub fn get_rank(&self) -> Result<Rank, &str> {
        Rank::try_from_usize(self.0 / 8)
    }

    /// Extract [File] from [Square], return [Result].
    /// Return [Err] when [Square] is not valid (it's outside the [Board]).
    pub fn get_file(&self) -> Result<File, &str> {
        File::try_from_usize(self.0 % 8)
    }

    const CREATE_MOVE_ERR: Result<ChessMove, &'static str> =
        Err("The resulting square is out of bounds.");

    /// Returning [Result].
    /// Create a new instance of [ChessMove] from [Square] destination that is moved n squares up.
    /// Return [Err] if [Square] is out of bounds of board.
    fn create_move(
        &self,
        sq_i: usize,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        let sq = Square(sq_i);
        // outside: up/down
        // outside: left/right is handled by functions which creating that moves
        if sq.0 > 63 {
            return Square::CREATE_MOVE_ERR;
        }

        Ok(ChessMove::new(*self, sq, promo))
    }

    /// Simplifie the procces of creating diagonal move.
    /// Centralize the error catching, which is same for every diagonal move. 
    fn create_diagonal_move(
        &self,
        sq_i: usize,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        let sq = Square(sq_i);
        if sq.0 > 63 || (self.get_rank().unwrap().to_usize() as i32 - sq.get_rank().unwrap().to_usize() as i32)
            .abs()
            != mul
        {
            return Square::CREATE_MOVE_ERR;
        }
        self.create_move(sq.0, promo)
    }

    /// Create [ChessMove] from [Square] to n squares up.
    pub(crate) fn up(&self, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str> {
        self.create_move((self.0 as i32 + 8 * mul) as usize, promo)
    }

    /// Create [ChessMove] from [Square] to n squares down.
    pub(crate) fn down(&self, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str> {
        self.create_move((self.0 as i32 - 8 * mul) as usize, promo)
    }

    /// Create [ChessMove] from [Square] to n squares right.
    pub(crate) fn right(&self, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str> {
        let sq = Square((self.0 as i32 + mul) as usize);
        if self.get_rank().unwrap() != sq.get_rank().unwrap() {
            return Square::CREATE_MOVE_ERR;
        }
        self.create_move(sq.0, promo)
    }

    /// Create [ChessMove] from [Square] to n squares left.
    pub(crate) fn left(&self, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str> {
        let sq = Square((self.0 as i32 - mul) as usize);
        if self.get_rank().unwrap() != sq.get_rank().unwrap() {
            return Square::CREATE_MOVE_ERR;
        }
        self.create_move(sq.0, promo)
    }

    /// Create [ChessMove] from [Square] to n squares right and down.
    pub(crate) fn down_right(
        &self,
        mul: i32,
        promo: Option<PieceType>,
    ) -> Result<ChessMove, &'static str> {
        self.create_diagonal_move((self.0 as i32 - 9 * mul) as usize, mul, promo)
    }

    /// Create [ChessMove] from [Square] to n squares left and down.
    pub(crate) fn down_left(&self, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str> {
        self.create_diagonal_move((self.0 as i32 - 7 * mul) as usize, mul, promo)
    }

    /// Create [ChessMove] from [Square] to n squares right and up.
    pub(crate) fn up_right(&self, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str> {
        self.create_diagonal_move((self.0 as i32 + 9 * mul) as usize, mul, promo)
    }

    /// Create [ChessMove] from [Square] to n squares left and up.
    pub(crate) fn up_left(&self, mul: i32, promo: Option<PieceType>) -> Result<ChessMove, &'static str> {
        self.create_diagonal_move((self.0 as i32 + 7 * mul) as usize, mul, promo)
    }

    /// built in constant A1 [Square] for easy indexing to [Board]
    pub const A1: Square = Square(56);
    /// built in constant B1 [Square] for easy indexing to [Board]
    pub const B1: Square = Square(57);
    /// built in constant C1 [Square] for easy indexing to [Board]
    pub const C1: Square = Square(58);
    /// built in constant D1 [Square] for easy indexing to [Board]
    pub const D1: Square = Square(59);
    /// built in constant E1 [Square] for easy indexing to [Board]
    pub const E1: Square = Square(60);
    /// built in constant F1 [Square] for easy indexing to [Board]
    pub const F1: Square = Square(61);
    /// built in constant G1 [Square] for easy indexing to [Board]
    pub const G1: Square = Square(62);
    /// built in constant H1 [Square] for easy indexing to [Board]
    pub const H1: Square = Square(63);
    /// built in constant A2 [Square] for easy indexing to [Board]
    pub const A2: Square = Square(48);
    /// built in constant B2 [Square] for easy indexing to [Board]
    pub const B2: Square = Square(49);
    /// built in constant C2 [Square] for easy indexing to [Board]
    pub const C2: Square = Square(50);
    /// built in constant D2 [Square] for easy indexing to [Board]
    pub const D2: Square = Square(51);
    /// built in constant E2 [Square] for easy indexing to [Board]
    pub const E2: Square = Square(52);
    /// built in constant F2 [Square] for easy indexing to [Board]
    pub const F2: Square = Square(53);
    /// built in constant G2 [Square] for easy indexing to [Board]
    pub const G2: Square = Square(54);
    /// built in constant H2 [Square] for easy indexing to [Board]
    pub const H2: Square = Square(55);
    /// built in constant A3 [Square] for easy indexing to [Board]
    pub const A3: Square = Square(40);
    /// built in constant B3 [Square] for easy indexing to [Board]
    pub const B3: Square = Square(41);
    /// built in constant C3 [Square] for easy indexing to [Board]
    pub const C3: Square = Square(42);
    /// built in constant D3 [Square] for easy indexing to [Board]
    pub const D3: Square = Square(43);
    /// built in constant E3 [Square] for easy indexing to [Board]
    pub const E3: Square = Square(44);
    /// built in constant F3 [Square] for easy indexing to [Board]
    pub const F3: Square = Square(45);
    /// built in constant G3 [Square] for easy indexing to [Board]
    pub const G3: Square = Square(46);
    /// built in constant H3 [Square] for easy indexing to [Board]
    pub const H3: Square = Square(47);
    /// built in constant A4 [Square] for easy indexing to [Board]
    pub const A4: Square = Square(32);
    /// built in constant B4 [Square] for easy indexing to [Board]
    pub const B4: Square = Square(33);
    /// built in constant C4 [Square] for easy indexing to [Board]
    pub const C4: Square = Square(34);
    /// built in constant D4 [Square] for easy indexing to [Board]
    pub const D4: Square = Square(35);
    /// built in constant E4 [Square] for easy indexing to [Board]
    pub const E4: Square = Square(36);
    /// built in constant F4 [Square] for easy indexing to [Board]
    pub const F4: Square = Square(37);
    /// built in constant G4 [Square] for easy indexing to [Board]
    pub const G4: Square = Square(38);
    /// built in constant H4 [Square] for easy indexing to [Board]
    pub const H4: Square = Square(39);
    /// built in constant A5 [Square] for easy indexing to [Board]
    pub const A5: Square = Square(24);
    /// built in constant B5 [Square] for easy indexing to [Board]
    pub const B5: Square = Square(25);
    /// built in constant C5 [Square] for easy indexing to [Board]
    pub const C5: Square = Square(26);
    /// built in constant D5 [Square] for easy indexing to [Board]
    pub const D5: Square = Square(27);
    /// built in constant E5 [Square] for easy indexing to [Board]
    pub const E5: Square = Square(28);
    /// built in constant F5 [Square] for easy indexing to [Board]
    pub const F5: Square = Square(29);
    /// built in constant G5 [Square] for easy indexing to [Board]
    pub const G5: Square = Square(30);
    /// built in constant H5 [Square] for easy indexing to [Board]
    pub const H5: Square = Square(31);
    /// built in constant A6 [Square] for easy indexing to [Board]
    pub const A6: Square = Square(16);
    /// built in constant B6 [Square] for easy indexing to [Board]
    pub const B6: Square = Square(17);
    /// built in constant C6 [Square] for easy indexing to [Board]
    pub const C6: Square = Square(18);
    /// built in constant D6 [Square] for easy indexing to [Board]
    pub const D6: Square = Square(19);
    /// built in constant E6 [Square] for easy indexing to [Board]
    pub const E6: Square = Square(20);
    /// built in constant F6 [Square] for easy indexing to [Board]
    pub const F6: Square = Square(21);
    /// built in constant G6 [Square] for easy indexing to [Board]
    pub const G6: Square = Square(22);
    /// built in constant H6 [Square] for easy indexing to [Board]
    pub const H6: Square = Square(23);
    /// built in constant A7 [Square] for easy indexing to [Board]
    pub const A7: Square = Square(8);
    /// built in constant B7 [Square] for easy indexing to [Board]
    pub const B7: Square = Square(9);
    /// built in constant C7 [Square] for easy indexing to [Board]
    pub const C7: Square = Square(10);
    /// built in constant D7 [Square] for easy indexing to [Board]
    pub const D7: Square = Square(11);
    /// built in constant E7 [Square] for easy indexing to [Board]
    pub const E7: Square = Square(12);
    /// built in constant F7 [Square] for easy indexing to [Board]
    pub const F7: Square = Square(13);
    /// built in constant G7 [Square] for easy indexing to [Board]
    pub const G7: Square = Square(14);
    /// built in constant H7 [Square] for easy indexing to [Board]
    pub const H7: Square = Square(15);
    /// built in constant A8 [Square] for easy indexing to [Board]
    pub const A8: Square = Square(0);
    /// built in constant B8 [Square] for easy indexing to [Board]
    pub const B8: Square = Square(1);
    /// built in constant C8 [Square] for easy indexing to [Board]
    pub const C8: Square = Square(2);
    /// built in constant D8 [Square] for easy indexing to [Board]
    pub const D8: Square = Square(3);
    /// built in constant E8 [Square] for easy indexing to [Board]
    pub const E8: Square = Square(4);
    /// built in constant F8 [Square] for easy indexing to [Board]
    pub const F8: Square = Square(5);
    /// built in constant G8 [Square] for easy indexing to [Board]
    pub const G8: Square = Square(6);
    /// built in constant H8 [Square] for easy indexing to [Board]
    pub const H8: Square = Square(7);
}
