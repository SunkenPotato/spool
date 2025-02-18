use std::collections::HashMap;

use crate::{binding::Identifier, val::Val, EvalError};

#[derive(Debug, Default, Clone)]
pub struct Env<'p> {
    pub bindings: HashMap<Identifier, Val>,
    pub parent: Option<&'p Self>,
}

impl<'p> Env<'p> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_parent(parent: &'p Self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn store_binding(&mut self, id: Identifier, val: Val) {
        self.bindings.insert(id, val);
    }

    pub fn get_stored_binding(&self, id: &Identifier) -> Result<Val, EvalError> {
        let get_val = |env: &Self, id: &Identifier| -> Result<Val, EvalError> {
            env.bindings.get(id).cloned().map(Ok).unwrap_or_else(|| {
                env.parent
                    .as_ref()
                    .map(|parent| parent.get_stored_binding(id))
                    .unwrap_or_else(|| Err(EvalError::IdentifierNotFound))
            })
        };

        get_val(self, id)
    }
}
