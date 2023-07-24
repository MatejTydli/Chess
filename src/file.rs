/// File enum for simple orientation on the board
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    /// Function for converting [usize] to [File].
    /// If conversion fails error message is returned in form of [str].
    pub(crate) fn try_from_usize(num: usize) -> Result<Self, &'static str> {
        match num {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::G),
            7 => Ok(Self::H),
            _ => Err("File::try_from_usize endup with Error: usize to big."),
        }
    }

    /// Converts [File] to [usize].
    pub(crate) fn to_usize(&self) -> usize {
        match self {
            Self::A => 7,
            Self::B => 6,
            Self::C => 5,
            Self::D => 4,
            Self::E => 3,
            Self::F => 2,
            Self::G => 1,
            Self::H => 0,
        }
    }
}
