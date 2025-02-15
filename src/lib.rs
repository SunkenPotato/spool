pub mod expr;
pub mod utils;
pub mod var;

mod env;
#[cfg(test)]
mod tests;
mod val;

pub const END: char = ';';
