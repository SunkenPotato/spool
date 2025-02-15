use std::collections::HashMap;

use crate::{val::Val, var::Identifier};

pub struct Env {
    bindings: HashMap<Identifier, Val>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            bindings: HashMap::new(),
        }
    }

    pub(crate) fn store_binding(&mut self, name: Identifier, value: Val) {
        self.bindings.insert(name, value);
    }

    pub fn get_binding_val(&self, name: &Identifier) -> Option<Val> {
        self.bindings.get(name).cloned()
    }
}
