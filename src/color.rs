#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

// impl Color {
//     pub(crate) fn inverse(&self) -> Color {
//         if *self == Color::White {
//             Color::Black
//         } else {
//             Color::White
//         }
//     }
// }
