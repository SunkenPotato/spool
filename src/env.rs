use crate::stmt::Stmt;

#[derive(Debug, Default, Clone)]
pub struct Env<'p, 's>
where
    'p: 's,
{
    pub bindings: Vec<Stmt<'s>>,
    pub parent: Option<&'p Self>,
}

impl<'p, 's> Env<'p, 's>
where
    'p: 's,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_parent(parent: &'p Self) -> Self {
        Self {
            bindings: vec![],
            parent: Some(parent),
        }
    }
}
