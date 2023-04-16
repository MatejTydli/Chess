/// rank enum for simple orientation on the board
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Rank {
    /// function for converting usize to Rank
    /// if conversion fails error msg is returned in form of String
    pub(crate) fn try_from_usize(num: usize) -> Result<Self, &'static str> {
        match num {
            0 => Ok(Self::First),
            1 => Ok(Self::Second),
            2 => Ok(Self::Third),
            3 => Ok(Self::Fourth),
            4 => Ok(Self::Fifth),
            5 => Ok(Self::Sixth),
            6 => Ok(Self::Seventh),
            7 => Ok(Self::Eighth),
            _ => Err("Rank::try_from_usize endup with Error: usize to big."),
        }
    }

    /// converts Rank to usize
    pub(crate) fn to_usize(&self) -> usize {
        match self {
            Self::First => 0,
            Self::Second => 1,
            Self::Third => 2,
            Self::Fourth => 3,
            Self::Fifth => 4,
            Self::Sixth => 5,
            Self::Seventh => 6,
            Self::Eighth => 7,
        }
    }
}
