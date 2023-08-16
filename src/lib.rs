// TODO: make `x <-` optional (may require proc_macro)
#[macro_export]
macro_rules! du {
    ( $x:ident <- $f:expr; $( $y:ident <- $g:expr; )* %return $ret:expr; ) => {
        Box::new(move |input| {
            let ($x, rest) = $f(input)?;
            $(
                let ($y, rest) = $g(rest)?;
            )*
            Some(($ret, rest))
        })
    };
}

pub type Parser<T> = Box<dyn Fn(&str) -> Option<(T, &str)>>;

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

pub fn alternative<T: 'static>(f: Parser<T>, g: Parser<T>) -> Parser<T> {
    Box::new(move |input| match f(input) {
        None => g(input),
        Some(xs) => Some(xs),
    })
}

pub fn sequence<T: Clone + 'static, U: 'static>(f: Parser<T>, g: Parser<U>) -> Parser<(T, U)> {
    Box::new(move |input| {
        let (ftree, rest) = f(input)?;
        let (gtree, rest) = g(rest)?;
        Some(((ftree, gtree), rest))
    })
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

    #[test]
    fn combinators() {
        let input = "hello";
        assert_eq!(
            alternative(character('x'), character('h'))(input),
            Some(('h', "ello"))
        );
        assert_eq!(
            sequence(character('h'), character('e'))(input),
            Some((('h', 'e'), "llo"))
        );
    }

    #[test]
    fn do_notation() {
        fn hello() -> Parser<Vec<char>> {
            du! {
                h <- character('h');
                e <- character('e');
                l <- character('l');
                l2 <- character('l');
                o <- character('o');
                %return vec![h, e, l, l2, o];
            }
        }

        assert_eq!(hello()("hello"), Some((vec!['h', 'e', 'l', 'l', 'o'], "")));
    }
}
