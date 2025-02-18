#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Str(String),
    Real(f32),
    Unit,
}

impl Val {
    pub fn get_type(&self) -> &'static str {
        match self {
            Self::Str(_) => "String",
            Self::Unit => "()",
            Self::Real(_) => "Real number",
        }
    }
}
