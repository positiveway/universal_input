pub mod key_codes;
pub mod input_emulator;
mod utils;

pub use key_codes::{KeyCode, KeyCodes};
pub use input_emulator::{OS_Input_Coord, InputEmulator};

#[cfg(all(target_os = "linux", not(feature = "enigo-always"), not(feature = "use-tfc")))]
pub use mouse_keyboard_input::{EventParams};

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
