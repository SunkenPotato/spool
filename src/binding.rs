use crate::{
    utils::{extract_ident, extract_whitespace},
    Parse,
};

pub struct Identifier(pub &'static str);

impl Parse for Identifier {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (id, s) = extract_ident(&s)?;

        todo!()
    }
}
