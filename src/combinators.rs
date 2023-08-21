use macros::du;

use crate::{text::empty, Parser};

/// Parse using `f`, or if `f` fails, using `g`
pub fn choice<T: 'static>(f: Parser<T>, g: Parser<T>) -> Parser<T> {
    Box::new(move |input| f(input).or_else(|| g(input)))
}

/// Parse `n` consecutive occurrences of `f`
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

/// Parse `p` if it occurs between `open` and `close`
/// # Example
/// ```rust
/// # use rarsec::{combinators::between, parse, text::{character, digit}};
/// let parens = between(character('('), character(')'), digit());
/// assert_eq!(parse(parens, "(1)").unwrap(), '1');
/// ```
pub fn between<T: 'static, U: 'static, V: 'static>(
    open: Parser<U>,
    close: Parser<V>,
    p: Parser<T>,
) -> Parser<T> {
    Box::new(du! {
        open;
        let res <- p;
        close;
        return res;
    })
}

/// Parse `f`, discarding its output, and then parse `g`. This is the same as `f >> g` in Haskell
/// or `du!` notation.
pub fn then<T: 'static, U: 'static>(f: Parser<T>, g: Parser<U>) -> Parser<U> {
    Box::new(move |input| {
        let (_, rest) = f(input)?;
        let (gtree, rest) = g(rest)?;
        Some((gtree, rest))
    })
}

/// Attempt parsing using `f`, returning `default` if it fails
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

/// Parse 0 or 1 instances of `f`, returning `()`
pub fn optional<T: 'static>(f: Parser<T>) -> Parser<()> {
    choice(
        Box::new(du! {
            f;
            return ();
        }),
        empty(),
    )

    // TODO: multiple symbols don't work?
    // TODO: maybe custom struct for nested closures
    // du! {
    //     let res <- f >> empty() | empty();
    //     return res;
    // }

    // TODO: return as keyword
    // du! {
    //     let res <- (f >> return ()) | empty();
    //     return res;
    // }
}

/// Skip 1 or more instances of `f`, returning `()`
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

/// Parse 1 or more instances of `f`
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

/// Parse 0 or more instances of `f`
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
    use crate::text::{character, letter, one_of};

    #[test]
    fn combinators() {
        let input = "hello";
        assert_eq!(
            choice(character('x'), character('h'))(input),
            Some(('h', "ello"))
        );
        assert_eq!(many(one_of("he"))(input), Some((vec!['h', 'e'], "llo")));
        assert_eq!(many(one_of("abc"))(input), Some((Vec::new(), "hello")));
        assert_eq!(optional(character('!'))(input), Some(((), "hello")));
        assert_eq!(optional(character('h'))(input), Some(((), "ello")));
        assert_eq!(count(3, letter())(input), Some((vec!['h', 'e', 'l'], "lo")));
        assert_eq!(
            between(character('('), character(')'), many(letter()))("(hi)"),
            Some((vec!['h', 'i'], ""))
        );
    }
}
