pub mod key_codes;
pub mod input_emulator;
mod utils;

pub use key_codes::{KeyCode, KeyCodes};
pub use input_emulator::{Coord, InputEmulator};

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
