pub mod ptype;
pub mod evaluator;


use std::collections::BTreeMap;
use ptype::PType;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Context {
    symbol_tbl: BTreeMap<String, AtomicUsize>,
    procedures: BTreeMap<AtomicUsize, ()>,
    types: BTreeMap<AtomicUsize, PType>,
}
