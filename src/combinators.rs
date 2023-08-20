use macros::du;

use crate::{text, Parser};

pub fn choice<T: 'static>(f: Parser<T>, g: Parser<T>) -> Parser<T> {
    Box::new(move |input| f(input).or_else(|| g(input)))
}

pub fn count<T: 'static>(n: usize, f: Parser<T>) -> Parser<Vec<T>> {
    Box::new(move |input| {
        let mut res = Vec::with_capacity(n);
        let (tree, rest) = f(input)?;
        res.push(tree);
        let mut ptr = rest;
        for _ in 0..(n - 1) {
            let (tree, rest) = f(ptr)?;
            res.push(tree);
            ptr = rest;
        }
        Some((res, ptr))
    })
}

pub fn between<T: 'static, U: 'static, V: 'static>(
    open: Parser<U>,
    close: Parser<V>,
    p: Parser<T>,
) -> Parser<T> {
    du! {
        open;
        let res <- p;
        close;
        return res;
    }
}

pub fn sequence<T: 'static, U: 'static>(f: Parser<T>, g: Parser<U>) -> Parser<(T, U)> {
    Box::new(move |input| {
        let (ftree, rest) = f(input)?;
        let (gtree, rest) = g(rest)?;
        Some(((ftree, gtree), rest))
    })
}

pub fn then<T: 'static, U: 'static>(f: Parser<T>, g: Parser<U>) -> Parser<U> {
    Box::new(move |input| {
        let (_, rest) = f(input)?;
        let (gtree, rest) = g(rest)?;
        Some((gtree, rest))
    })
}

pub fn option<T: Clone + 'static>(default: T, f: Parser<T>) -> Parser<T> {
    Box::new(move |input| {
        if let Some(res) = f(input) {
            Some(res)
        } else {
            Some((default.clone(), input))
        }
    })
}

/// this doesn't translate well from Haskell `optionMaybe`
pub fn option_option<T: 'static>(f: Parser<T>) -> Parser<Option<T>> {
    Box::new(move |input| {
        if let Some((tree, rest)) = f(input) {
            Some((Some(tree), rest))
        } else {
            Some((None, input))
        }
    })
}

pub fn optional<T: 'static>(f: Parser<T>) -> Parser<()> {
    choice(
        du! {
            f;
            return ();
        },
        text::empty(),
    )

    // TODO: multiple symbols don't work?
    // TODO: maybe custom struct for nested closures
    // du! {
    //     let res <- f >> text::empty() | text::empty();
    //     return res;
    // }

    // TODO: return as keyword
    // du! {
    //     let res <- (f >> return ()) | text::empty();
    //     return res;
    // }
}

pub fn skip_many1<T: 'static>(f: Parser<T>) -> Parser<()> {
    Box::new(move |input| {
        if let Some((_, rest)) = f(input) {
            let mut ptr = rest;
            while let Some((_, rest)) = f(ptr) {
                ptr = rest;
            }
            Some(((), ptr))
        } else {
            None
        }
    })
}

pub fn many1<T: 'static>(f: Parser<T>) -> Parser<Vec<T>> {
    Box::new(move |input| {
        let mut out = Vec::new();
        if let Some((tree, rest)) = f(input) {
            let mut ptr = rest;
            out.push(tree);
            while let Some((tree, rest)) = f(ptr) {
                out.push(tree);
                ptr = rest;
            }
            Some((out, ptr))
        } else {
            None
        }
    })
}

pub fn many<T: 'static>(f: Parser<T>) -> Parser<Vec<T>> {
    Box::new(move |input| {
        let mut out = Vec::new();
        if let Some((tree, rest)) = f(input) {
            let mut ptr = rest;
            out.push(tree);
            while let Some((tree, rest)) = f(ptr) {
                out.push(tree);
                ptr = rest;
            }
            Some((out, ptr))
        } else {
            Some((Vec::new(), input))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text;

    #[test]
    fn combinators() {
        let input = "hello";
        assert_eq!(
            choice(text::character('x'), text::character('h'))(input),
            Some(('h', "ello"))
        );
        assert_eq!(
            sequence(text::character('h'), text::character('e'))(input),
            Some((('h', 'e'), "llo"))
        );
        assert_eq!(
            many(text::one_of("he"))(input),
            Some((vec!['h', 'e'], "llo"))
        );
        assert_eq!(
            many(text::one_of("abc"))(input),
            Some((Vec::new(), "hello"))
        );
        assert_eq!(optional(text::character('!'))(input), Some(((), "hello")));
        assert_eq!(optional(text::character('h'))(input), Some(((), "ello")));
        assert_eq!(
            count(3, text::letter())(input),
            Some((vec!['h', 'e', 'l'], "lo"))
        );
        assert_eq!(
            between(
                text::character('('),
                text::character(')'),
                many(text::letter())
            )("(hi)"),
            Some((vec!['h', 'i'], ""))
        );
    }
}
