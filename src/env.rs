use std::collections::HashMap;

use crate::val::Val;

pub struct Env {
    bindings: HashMap<String, Val>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            bindings: HashMap::new(),
        }
    }

    pub(crate) fn store_binding(&mut self, name: String, value: Val) {
        self.bindings.insert(name, value);
    }
}
