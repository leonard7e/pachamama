/*

*/

use super::Context;

pub trait Evaluator {
    fn eval(&self, context: Context);
}
