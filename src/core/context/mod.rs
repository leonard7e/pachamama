pub mod ptype;
pub mod evaluator;

use std::collections::BTreeMap;
use ptype::PType;

use std::sync::Arc;
use super::utils::typedefs::Key;

pub struct Context {
    symbol_tbl: BTreeMap<String, Key>,
    types: BTreeMap<Key, Arc<PType>>,
}
