use crate::ParseError;

fn take_while(s: &str, f: impl Fn(char) -> bool) -> (String, String) {
    let end = s
        .char_indices()
        .find_map(|(idx, c)| if f(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    (s[..end].into(), s[end..].into())
}

pub fn extract_whitespace(s: &str) -> (String, String) {
    take_while(s, |c| c.is_ascii_whitespace())
}

pub fn extract_string(s: &str) -> Result<(String, String), ParseError> {
    let s = tag("\"", s)?;

    let (string, rest) = take_while(&s, |c| c != '"');
    let rest = tag("\"", &rest)?;

    Ok((string, rest))
}

pub fn extract_float(s: &str) -> (String, String) {
    take_while(s, |c| c.is_ascii_digit() || c == '.')
}

pub fn extract_op(s: &str) -> Result<(String, String), ParseError> {
    match s.chars().next() {
        Some('+' | '-' | '*' | '/') => (),
        e => {
            return Err(ParseError::SequenceNotFound {
                expected: "[+, -, *, /]".into(),
                received: format!("{e:?}"),
            });
        }
    };

    Ok((s[..1].into(), s[1..].into()))
}

pub fn extract_ident(s: &str) -> Result<(String, String), ParseError> {
    let (id, rest) = take_while(s, |c| c.is_ascii_alphanumeric());

    if s.chars()
        .next()
        .ok_or(ParseError::SequenceNotFound {
            expected: "an identifier".into(),
            received: "".into(),
        })?
        .is_ascii_digit()
    {
        return Err(ParseError::InvalidSequence {
            expected: "an identifier not beginning with a digit".into(),
            received: id.into(),
        });
    }

    Ok((id.into(), rest.into()))
}

pub fn tag(seq: &str, s: &str) -> Result<String, ParseError> {
    if s.starts_with(seq) {
        Ok(s[seq.len()..].into())
    } else {
        Err(ParseError::SequenceNotFound {
            expected: seq.into(),
            received: s.into(),
        })
    }
}
