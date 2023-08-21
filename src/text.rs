use crate::Parser;

pub fn empty() -> Parser<()> {
    Box::new(move |input| Some(((), input)))
}

pub fn character(x: char) -> Parser<char> {
    Box::new(move |input| {
        if input.starts_with(x) {
            Some((x, &input[1..]))
        } else {
            None
        }
    })
}

pub fn one_of(xs: &'static str) -> Parser<char> {
    Box::new(move |input| {
        xs.chars()
            .find(|x| input.starts_with(*x))
            .map(|x| (x, &input[1..]))
    })
}

pub fn satisfy(f: impl Fn(char) -> bool + 'static) -> Parser<char> {
    Box::new(move |input| {
        let c = input.chars().next()?;
        if f(c) {
            Some((c, &input[1..]))
        } else {
            None
        }
    })
}

pub fn digit() -> Parser<char> {
    satisfy(|c| c.is_numeric())
}

pub fn letter() -> Parser<char> {
    satisfy(|c| c.is_alphabetic())
}

pub fn upper() -> Parser<char> {
    satisfy(|c| c.is_uppercase())
}

pub fn lower() -> Parser<char> {
    satisfy(|c| c.is_lowercase())
}

pub fn whitespace() -> Parser<char> {
    satisfy(|c| c.is_whitespace())
}

pub fn newline() -> Parser<char> {
    character('\n')
}

pub fn none_of(xs: &'static str) -> Parser<char> {
    satisfy(|c| !xs.contains(c))
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
