/*

*/

use crate::core::utils::typedefs::Key;

pub trait Node {
    fn has_input_slot(&self, s: Key) -> bool;
    fn has_output_slot(&self, s: Key) -> bool;
}
