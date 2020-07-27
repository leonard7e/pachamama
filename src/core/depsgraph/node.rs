/*

*/

use std::arc::Arc;
use std::rc::RefCell;

struct Node <E: Evaluator>{
        name: String,
        evaluator: Arc<RefCell<E>>
};
