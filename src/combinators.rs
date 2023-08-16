use macros::du;

use crate::{text, Parser};

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

// could be du! { (f >> ()) <|> text::empty() }
pub fn optional<T: 'static>(f: Parser<T>) -> Parser<()> {
    alternative(
        du! {
            f;
            return ();
        },
        text::empty(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text;

    #[test]
    fn combinators() {
        let input = "hello";
        assert_eq!(
            alternative(text::character('x'), text::character('h'))(input),
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
    }
}
