use std::error::Error;

pub type ParseError = Box<dyn Error>;
pub type ParseOutput<S> = Result<(String, S), ParseError>;

pub trait Parse: Sized {
    fn parse(input: &str) -> Result<(String, Self), ParseError>;
}
