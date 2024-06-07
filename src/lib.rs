pub mod key_codes;
mod utils;
mod spec_mki;
mod spec_tfc;
mod stubs;
mod spec_enigo;
mod spec_hidg;

pub type OS_Input_Coord = i32;

pub use key_codes::{KeyCode, KeyCodes};
pub use crate::stubs::*;

#[cfg(feature = "use_mki")]
pub use crate::spec_mki::*;

#[cfg(feature = "use_tfc")]
pub use crate::spec_tfc::*;

#[cfg(feature = "use_enigo")]
pub use crate::spec_enigo::*;

#[cfg(feature = "use_hidg")]
pub use crate::spec_hidg::*;


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
