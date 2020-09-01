pub fn unescape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        result.push(if ch != '\\' {
            ch
        } else {
            match chars.next() {
                Some('u') => {
                    let value = chars
                        .by_ref()
                        .take(4)
                        .fold(0, |acc, c| acc * 16 + c.to_digit(16).unwrap());
                    char::from_u32(value).unwrap()
                }
                Some('b') => '\x08',
                Some('f') => '\x0c',
                Some('n') => '\n',
                Some('r') => '\r',
                Some('t') => '\t',
                Some(ch) => ch,
                _ => panic!("Malformed escape"),
            }
        })
    }
    result
}
