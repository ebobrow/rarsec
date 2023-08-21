pub mod combinators;
pub mod text;

pub use macros::du;

// TODO: not just strings (maybe use Read trait)
pub type Parser<T> = Box<dyn Fn(&str) -> Option<(T, &str)>>;

pub fn parse<T>(parser: Parser<T>, input: &str) -> Option<T> {
    let (tree, rest) = parser(input)?;
    assert!(rest.is_empty());
    Some(tree)
}

#[cfg(test)]
mod tests {
    use crate::{combinators::many, text::character};

    use super::*;

    #[test]
    fn do_notation() {
        fn hello() -> Parser<Vec<char>> {
            Box::new(du! {
                let h <- character('h');
                let e <- character('e');
                let l <- character('l') >> character('l');
                let o <- character('o');
                return vec![h, e, l, l, o];
            })
        }

        assert_eq!(hello()("hello"), Some((vec!['h', 'e', 'l', 'l', 'o'], "")));

        assert_eq!(
            (du! {
                let he <- many(character('h') | character('e'));
                let l <- character('l');
                let lo <- character('l') >> character('o');
                return (he, l, lo);
            })("hello"),
            Some(((vec!['h', 'e'], 'l', 'o'), ""))
        );
    }
}
