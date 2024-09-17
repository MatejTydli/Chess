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

    /// Converts [Rank] to [String].
    pub fn to_string(&self) -> String {
        match self {
            Self::First => 1.to_string(),
            Self::Second => 2.to_string(),
            Self::Third => 3.to_string(),
            Self::Fourth => 4.to_string(),
            Self::Fifth => 5.to_string(),
            Self::Sixth => 6.to_string(),
            Self::Seventh => 7.to_string(),
            Self::Eighth => 8.to_string(),
        }
    }
}
