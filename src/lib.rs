use std::error::Error;

pub mod block;
pub mod expr;
pub mod parse;
pub mod utils;
pub mod var;

mod env;
mod val;

pub type DynError<T> = Result<T, Box<dyn Error>>;
