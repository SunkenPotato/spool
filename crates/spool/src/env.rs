use std::collections::HashMap;

use crate::{binding::Identifier, expr::Expr, val::Val, EvalError};

#[derive(Debug, PartialEq, Clone)]
pub enum Storeable {
    Binding(Val),
    Func(NamelessFunction),
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamelessFunction {
    pub(crate) params: Vec<Identifier>,
    pub(crate) body: Expr,
}

#[derive(Debug, Default, Clone)]
pub struct Env<'p> {
    pub store: HashMap<Identifier, Storeable>,
    pub parent: Option<&'p Self>,
}

impl<'p> Env<'p> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_parent(parent: &'p Self) -> Self {
        Self {
            store: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn store_binding(&mut self, id: Identifier, val: Val) {
        self.store.insert(id, Storeable::Binding(val));
    }

    pub fn get_stored_binding(&self, id: &Identifier) -> Result<Val, EvalError> {
        match self.store.get(id).cloned() {
            Some(v) => match v {
                Storeable::Binding(v) => Ok(v),
                _ => Err(EvalError::InvalidStoredType),
            },
            None => match self.parent {
                Some(v) => v.get_stored_binding(id),
                None => Err(EvalError::IdentifierNotFound(id.clone())),
            },
        }
    }

    pub fn store_func(&mut self, id: Identifier, params: Vec<Identifier>, body: Expr) {
        self.store
            .insert(id, Storeable::Func(NamelessFunction { params, body }));
    }

    pub fn get_stored_func(&self, id: &Identifier) -> Result<NamelessFunction, EvalError> {
        match self.store.get(id).cloned() {
            Some(v) => match v {
                Storeable::Func(v) => Ok(v),
                _ => Err(EvalError::InvalidStoredType),
            },
            None => match self.parent {
                Some(v) => v.get_stored_func(id),
                None => Err(EvalError::IdentifierNotFound(id.clone())),
            },
        }
    }
}
