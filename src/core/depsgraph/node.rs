/*

*/

use crate::core::{context::Context, utils::typedefs::Key};

use super::slot::Slot;

pub trait Node {
    fn has_input_slot(&self, s: &Key) -> bool;
    fn has_output_slot(&self, s: &Key) -> bool;
    fn eval(&self, context: Context, input: Vec<Slot>) -> Vec<Slot>;
}
