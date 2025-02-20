use crate::{
    binding::Identifier,
    utils::{extract_whitespace, tag},
    Eval, Parse,
};

#[derive(Debug, PartialEq, Clone)]
pub struct FuncCall {
    pub(crate) callee: Identifier,
    pub(crate) params: Vec<Identifier>,
}

impl Parse for FuncCall {
    fn parse(s: &str) -> crate::ParseOutput<Self> {
        let (_, s) = extract_whitespace(s);
        let (s, id) = Identifier::parse(&s)?;

        let s = tag("(", &s)?;

        let mut params = vec![];
        let mut s = s;

        while let Ok((new_s, id)) = Identifier::parse(&s) {
            params.push(id);
            s = match tag(",", &new_s) {
                Ok(v) => v,
                Err(_) => new_s,
            };
        }

        let s = tag(")", &s)?;

        Ok((s, Self { callee: id, params }))
    }
}

impl Eval for FuncCall {
    fn eval(&self, _env: &mut crate::Env) -> Result<crate::val::Val, crate::EvalError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{fn_call::FuncCall, Parse};

    #[test]
    fn parse_fn_call_no_params() {
        assert_eq!(
            FuncCall::parse("test()"),
            Ok((
                "".into(),
                FuncCall {
                    callee: "test".into(),
                    params: vec![]
                }
            ))
        )
    }

    #[test]
    fn parse_fn_call_one_param() {
        assert_eq!(
            FuncCall::parse("test(hello)"),
            Ok((
                "".into(),
                FuncCall {
                    callee: "test".into(),
                    params: vec!["hello".into()]
                }
            ))
        )
    }

    #[test]
    fn parse_fn_call_multiple_params() {
        assert_eq!(
            FuncCall::parse("test(hello, world)"),
            Ok((
                "".into(),
                FuncCall {
                    callee: "test".into(),
                    params: vec!["hello".into(), "world".into()]
                }
            ))
        )
    }
}
