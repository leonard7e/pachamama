pub mod ptype;

use std::collections::BTreeMap;
use ptype::PType;

use std::sync::Arc;
use super::utils::typedefs::Key;

pub struct Context {
}

impl Context {
    pub fn new() -> Self {
        Context {}
    }
}
