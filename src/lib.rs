use std::error::Error;

pub mod expr;
pub mod utils;
pub mod var;

pub mod block;
mod env;
mod val;

pub const END: char = ';';
pub type DynError<T> = Result<T, Box<dyn Error>>;
