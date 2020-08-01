pub mod ptype;

use std::collections::BTreeMap;
use ptype::PType;

use std::sync::Arc;
use super::utils::typedefs::Key;

pub struct Context <T:PType>{
    symbol_tbl: BTreeMap<String, Key>,
    types: BTreeMap<Key, Arc<T>>,
}
