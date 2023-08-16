// TODO: add fun stuff like >>= for one-liners
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, Ident, Token};

struct Line {
    x: Option<Ident>,
    f: Expr,
}

struct MacroInput {
    lines: Vec<Line>,
    ret: Expr,
}

impl Parse for Line {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        // TODO: get rid of let
        if lookahead.peek(Token![let]) {
            let _: Token![let] = input.parse()?;
            let x: Ident = input.parse()?;
            let _: Token![<-] = input.parse()?;
            let f: Expr = input.parse()?;
            let _: Token![;] = input.parse()?;
            Ok(Line { x: Some(x), f })
        } else {
            let f: Expr = input.parse()?;
            let _: Token![;] = input.parse()?;
            Ok(Line { x: None, f })
        }
    }
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut lines = Vec::new();
        loop {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![return]) {
                let _: Token![return] = input.parse()?;
                let ret: Expr = input.parse()?;
                let _: Token![;] = input.parse()?;
                return Ok(MacroInput { lines, ret });
            } else {
                lines.push(input.parse()?);
            }
        }
    }
}

/// Emulates Haskell's `do` notation
/// # Example:
/// ```rust
/// du! {
///     let h <- character('h');
///     let e <- character('e');
///     let l <- character('l');
///     character('l');
///     let o <- character('o');
///     return vec![h, e, l, l, o];
/// }
/// ```
#[proc_macro]
pub fn du(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as MacroInput);
    if input.lines.len() == 0 {
        let ret = input.ret;
        TokenStream::from(quote! {
            Box::new(move |input| {
                Some((#ret, input))
            })
        })
    } else {
        let f = &input.lines[0].f;
        let first_line = if let Some(x) = &input.lines[0].x {
            quote! { let (#x, rest) = #f(input)?; }
        } else {
            quote! { let (_, rest) = #f(input)?; }
        };
        let next_lines: Vec<_> = input
            .lines
            .iter()
            .skip(1)
            .map(|line| {
                let f = &line.f;
                if let Some(x) = &line.x {
                    quote! { let (#x, rest) = #f(rest)?; }
                } else {
                    quote! { let (_, rest) = #f(rest)?; }
                }
            })
            .collect();
        let ret = input.ret;
        TokenStream::from(quote! {
            Box::new(move |input| {
                #first_line
                #(#next_lines)*
                Some((#ret, rest))
            })
        })
    }
}
