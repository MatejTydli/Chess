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
    /// function for converting usize to File
    /// if conversion fails error msg is returned in form of String 
    pub fn try_from_usize(num: usize) -> Result<Self, String> {
        match num {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::G),
            7 => Ok(Self::H),
            _ => Err(String::from(
                "File::try_from_usize endup with Error: usize to big.",
            )),
        }
    }

    /// converts File to usize
    pub fn to_usize(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
            Self::E => 4,
            Self::F => 5,
            Self::G => 6,
            Self::H => 7,
        }
    }
}
