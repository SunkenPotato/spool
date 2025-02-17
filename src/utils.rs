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

pub fn extract_numbers(s: &str) -> (String, String) {
    take_while(s, |c| c.is_ascii_digit())
}
