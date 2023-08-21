use crate::parser;

/// Parse nothing, returning `()`
pub fn empty() -> parser!(()) {
    move |input| Some(((), input))
}

/// Parse one instance of `c`
pub fn character(c: char) -> parser!(char) {
    move |input| {
        if input.starts_with(c) {
            Some((c, &input[1..]))
        } else {
            None
        }
    }
}

/// Parse one instance of one of the characters in `cs`
pub fn one_of(cs: &'static str) -> parser!(char) {
    move |input| {
        cs.chars()
            .find(|x| input.starts_with(*x))
            .map(|x| (x, &input[1..]))
    }
}

/// Parse one character if it satisfies `f`
pub fn satisfy(f: impl Fn(char) -> bool + 'static) -> parser!(char) {
    move |input| {
        let c = input.chars().next()?;
        if f(c) {
            Some((c, &input[1..]))
        } else {
            None
        }
    }
}

/// Parse one numeric character
pub fn digit() -> parser!(char) {
    satisfy(|c| c.is_numeric())
}

/// Parse one alphabetic character
pub fn letter() -> parser!(char) {
    satisfy(|c| c.is_alphabetic())
}

/// Parse an uppercase letter
pub fn upper() -> parser!(char) {
    satisfy(|c| c.is_uppercase())
}

/// Parse a lowercase letter
pub fn lower() -> parser!(char) {
    satisfy(|c| c.is_lowercase())
}

/// Parse one whitespace character
pub fn whitespace() -> parser!(char) {
    satisfy(|c| c.is_whitespace())
}

/// Parse a newline
pub fn newline() -> parser!(char) {
    character('\n')
}

/// Parse one character if it is not present in `cs`
pub fn none_of(cs: &'static str) -> parser!(char) {
    satisfy(|c| !cs.contains(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizers() {
        let input = "hello";
        assert_eq!(empty()(input), Some(((), "hello")));
        assert_eq!(character('h')(input), Some(('h', "ello")));
    }
}
