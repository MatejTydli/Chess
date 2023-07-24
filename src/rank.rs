/// Rank enum for simple orientation on the [crate::Board].
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
    /// Function for converting [usize] to [Rank].
    /// If conversion fails error message is returned in form of [str].
    pub(crate) fn try_from_usize(num: usize) -> Result<Self, &'static str> {
        match num {
            0 => Ok(Self::Eighth),
            1 => Ok(Self::Seventh),
            2 => Ok(Self::Sixth),
            3 => Ok(Self::Fifth),
            4 => Ok(Self::Fourth),
            5 => Ok(Self::Third),
            6 => Ok(Self::Second),
            7 => Ok(Self::First),
            _ => Err("Rank::try_from_usize endup with Error: usize to big."),
        }
    }

    /// Converts [Rank] to [usize].
    pub(crate) fn to_usize(&self) -> usize {
        match self {
            Self::First => 7,
            Self::Second => 6,
            Self::Third => 5,
            Self::Fourth => 4,
            Self::Fifth => 3,
            Self::Sixth => 2,
            Self::Seventh => 1,
            Self::Eighth => 0,
        }
    }
}
