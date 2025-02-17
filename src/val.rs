#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Str(String),
    Real(f32),
    Unit,
}
