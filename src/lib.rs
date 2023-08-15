pub type Parser<T> = Box<dyn Fn(&str) -> Vec<(T, &str)>>;

pub fn empty() -> Parser<()> {
    Box::new(move |input| vec![((), input)])
}

pub fn character(x: char) -> Parser<char> {
    Box::new(move |input| {
        if input.starts_with(x) {
            vec![(x, &input[1..])]
        } else {
            Vec::new()
        }
    })
}

pub fn alternative<T: 'static>(f: Parser<T>, g: Parser<T>) -> Parser<T> {
    Box::new(move |input| match f(input) {
        xs if xs.is_empty() => g(input),
        xs => xs,
    })
}

pub fn sequence<T: Clone + 'static, U: 'static>(f: Parser<T>, g: Parser<U>) -> Parser<(T, U)> {
    Box::new(move |input| {
        let mut out = Vec::new();
        for (ftree, rest) in f(input) {
            for (gtree, rest) in g(rest) {
                out.push(((ftree.clone(), gtree), rest));
            }
        }
        out
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizers() {
        let input = "hello";
        assert_eq!(empty()(input), vec![((), "hello")]);
        assert_eq!(character('h')(input), vec![('h', "ello")]);
    }

    #[test]
    fn combinators() {
        let input = "hello";
        assert_eq!(
            alternative(character('x'), character('h'))(input),
            vec![('h', "ello")]
        );
        assert_eq!(
            sequence(character('h'), character('e'))(input),
            vec![(('h', 'e'), "llo")]
        );
    }
}
