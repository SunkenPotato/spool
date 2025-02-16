use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Val {
    Integer(i32),
    Unit,
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Integer(int) => format!("{int}"),
            Self::Unit => String::from("()"),
        };

        write!(f, "{s}")
    }
}
