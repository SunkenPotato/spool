use std::collections::HashMap;

use crate::{expr::EvalError, val::Val, var::Identifier};

#[derive(Default, Clone)]
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

    pub fn get_binding_val(&self, name: &Identifier) -> Result<Val, EvalError> {
        self.bindings
            .get(name)
            .cloned()
            .ok_or("Could not find value in environment".into())
    }
}
