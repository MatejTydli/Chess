#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Returns inverted [Color].
    pub fn get_inverse(&self) -> Color {
        if *self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}
