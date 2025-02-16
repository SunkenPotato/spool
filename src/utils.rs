use crate::DynError;

pub fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (String, String) {
    let end = s
        .char_indices()
        .find_map(|(i, c)| if accept(c) { None } else { Some(i) })
        .unwrap_or_else(|| s.len());

    ((&s[..end]).into(), (&s[end..]).into())
}

pub fn extract_digits(s: &str) -> (String, String) {
    take_while(|c| c.is_ascii_digit(), s)
}

pub fn extract_whitespace(s: &str) -> (String, String) {
    take_while(|c| c == ' ', s)
}

pub fn extract_operator(s: &str) -> (String, String) {
    match &s.chars().next().unwrap_or(' ') {
        '+' | '-' | '*' | '/' => {}
        _ => return (s.into(), "".into()),
    };

    ((&s[1..]).into(), (&s[0..1]).into())
}

pub fn extract_identifier(s: &str) -> Result<(String, String), &'static str> {
    let starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if starts_with_alphabetic {
        return Ok(take_while(|c| c.is_ascii_alphanumeric(), s));
    } else {
        return Err("identifier does not start with an alphabetical character");
    }
}

pub fn tag(word: &str, input: &str) -> DynError<String> {
    if input.starts_with(word) {
        Ok(input[word.len()..].into())
    } else {
        Err(format!("expected `{}`", word).into())
    }
}
