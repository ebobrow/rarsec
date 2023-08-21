use macros::dum;

use crate::{parser, text::empty};

/// Parse using `f`, or if `f` fails, using `g`
pub fn choice<T: 'static>(f: parser!(T), g: parser!(T)) -> parser!(T) {
    move |input| f(input).or_else(|| g(input))
}

/// Parse `n` consecutive occurrences of `f`
pub fn count<T: 'static>(n: usize, f: parser!(T)) -> parser!(Vec<T>) {
    move |input| {
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
    }
}

/// Parse `p` if it occurs between `open` and `close`
/// # Example
/// ```rust
/// # use rarsec::{combinators::between, parse, text::{character, digit}};
/// let parens = between(character('('), character(')'), digit());
/// assert_eq!(parse(parens, "(1)").unwrap(), '1');
/// ```
pub fn between<T: 'static, U: 'static, V: 'static>(
    open: parser!(U),
    close: parser!(V),
    p: parser!(T),
) -> parser!(T) {
    dum! {
        open;
        let res <- p;
        close;
        return res;
    }
}

/// Parse `f`, discarding its output, and then parse `g`. This is the same as `f >> g` in Haskell
/// or `du!` notation.
pub fn then<T: 'static, U: 'static>(f: parser!(T), g: parser!(U)) -> parser!(U) {
    move |input| {
        let (_, rest) = f(input)?;
        let (gtree, rest) = g(rest)?;
        Some((gtree, rest))
    }
}

/// Attempt parsing using `f`, returning `default` if it fails
pub fn option<T: Clone + 'static>(default: T, f: parser!(T)) -> parser!(T) {
    move |input| {
        if let Some(res) = f(input) {
            Some(res)
        } else {
            Some((default.clone(), input))
        }
    }
}

/// this doesn't translate well from Haskell `optionMaybe`
pub fn option_option<T: 'static>(f: parser!(T)) -> parser!(Option<T>) {
    move |input| {
        if let Some((tree, rest)) = f(input) {
            Some((Some(tree), rest))
        } else {
            Some((None, input))
        }
    }
}

/// Parse 0 or 1 instances of `f`, returning `()`
pub fn optional<T: 'static>(f: parser!(T)) -> parser!(()) {
    dum! {
        let res <- f >> empty() | empty();
        return res;
    }

    // TODO: return as keyword
    // du! {
    //     let res <- (f >> return ()) | empty();
    //     return res;
    // }
}

/// Skip 1 or more instances of `f`, returning `()`
pub fn skip_many1<T: 'static>(f: parser!(T)) -> parser!(()) {
    move |input| {
        if let Some((_, rest)) = f(input) {
            let mut ptr = rest;
            while let Some((_, rest)) = f(ptr) {
                ptr = rest;
            }
            Some(((), ptr))
        } else {
            None
        }
    }
}

/// Parse 1 or more instances of `f`
pub fn many1<T: 'static>(f: parser!(T)) -> parser!(Vec<T>) {
    move |input| {
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
    }
}

/// Parse 0 or more instances of `f`
pub fn many<T: 'static>(f: parser!(T)) -> parser!(Vec<T>) {
    move |input| {
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
    }
}

/// Parse 0 or more instances of `f`, separated by `sep`
/// # Example:
/// ```rust
/// # use rarsec::{combinators::sep_by, parse, text::{character, digit}};
/// let num_list = sep_by(digit(), character(','));
/// assert_eq!(parse(num_list, "1,2,3").unwrap(), vec!['1', '2', '3']);
/// ```
pub fn sep_by<T: 'static, U: 'static>(f: parser!(T), sep: parser!(U)) -> parser!(Vec<T>) {
    move |input| {
        let mut out = Vec::new();
        if let Some((tree, rest)) = f(input) {
            let mut ptr = rest;
            out.push(tree);
            while let Some((_, rest)) = sep(ptr) {
                let (tree, rest) = f(rest)?;
                out.push(tree);
                ptr = rest;
            }
            Some((out, ptr))
        } else {
            Some((Vec::new(), input))
        }
    }
}

/// Parse 1 or more instances of `f`, separated by `sep`
pub fn sep_by1<T: 'static, U: 'static>(f: parser!(T), sep: parser!(U)) -> parser!(Vec<T>) {
    move |input| {
        let mut out = Vec::new();
        if let Some((tree, rest)) = f(input) {
            let mut ptr = rest;
            out.push(tree);
            while let Some((_, rest)) = sep(ptr) {
                let (tree, rest) = f(rest)?;
                out.push(tree);
                ptr = rest;
            }
            Some((out, ptr))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::{character, digit, letter, one_of};

    #[test]
    fn combinators() {
        let input = "hello";
        assert_eq!(
            choice(character('x'), character('h'))(input),
            Some(('h', "ello"))
        );
        assert_eq!(many(one_of("he"))(input), Some((vec!['h', 'e'], "llo")));
        assert_eq!(many1(one_of("he"))(input), Some((vec!['h', 'e'], "llo")));
        assert_eq!(many(one_of("abc"))(input), Some((Vec::new(), "hello")));
        assert_eq!(many1(one_of("abc"))(input), None);
        assert_eq!(optional(character('!'))(input), Some(((), "hello")));
        assert_eq!(optional(character('h'))(input), Some(((), "ello")));
        assert_eq!(count(3, letter())(input), Some((vec!['h', 'e', 'l'], "lo")));
        assert_eq!(
            between(character('('), character(')'), many(letter()))("(hi)"),
            Some((vec!['h', 'i'], ""))
        );
        assert_eq!(
            sep_by(digit(), character(','))("1,..."),
            Some((vec!['1'], ",..."))
        );
        assert_eq!(
            sep_by1(digit(), character(','))("1,2,3..."),
            Some((vec!['1', '2', '3'], "..."))
        );
        assert_eq!(
            sep_by(letter(), character('/'))("1/2/3"),
            Some((Vec::new(), "1/2/3"))
        );
        assert_eq!(sep_by1(letter(), character('/'))("1/2/3"), None);
    }
}
