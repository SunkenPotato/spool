use std::collections::HashMap;

use crate::{val::Val, var::Identifier, EvalError};

#[derive(Default, Clone, Debug)]
pub struct Env<'parent> {
    bindings: HashMap<Identifier, Val>,
    pub parent: Option<&'parent Self>,
}

impl<'a> Env<'a> {
    pub fn new<'b: 'a>(parent: Option<&'b Self>) -> Self {
        Env {
            bindings: HashMap::new(),
            parent,
        }
    }

    pub(crate) fn store_binding(&mut self, name: Identifier, value: Val) {
        self.bindings.insert(name, value);
    }

    pub fn get_binding_val(&self, name: &Identifier) -> Result<Val, EvalError> {
        let get_val = |env: &Self, name: &Identifier| -> Result<Val, EvalError> {
            env.bindings.get(name).cloned().map(Ok).unwrap_or_else(|| {
                env.parent
                    .as_ref()
                    .map(|parent| parent.get_binding_val(name))
                    .unwrap_or_else(|| {
                        Err(EvalError::NotFound(format!(
                            "Could not find `{:?}` in environment",
                            name
                        )))
                    })
            })
        };

        get_val(self, name)
    }
}
