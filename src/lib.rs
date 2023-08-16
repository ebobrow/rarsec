pub mod combinators;
pub mod text;

pub use macros::du;

// TODO: not just strings (maybe use Read trait)
pub type Parser<T> = Box<dyn Fn(&str) -> Option<(T, &str)>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_notation() {
        fn hello() -> Parser<Vec<char>> {
            du! {
                let h <- text::character('h');
                let e <- text::character('e');
                let l <- text::character('l');
                text::character('l');
                let o <- text::character('o');
                return vec![h, e, l, l, o];
            }
        }

        assert_eq!(hello()("hello"), Some((vec!['h', 'e', 'l', 'l', 'o'], "")));
    }
}
