use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, BinOp, Expr, ExprCall, Ident, Token};

struct Line {
    x: Option<Ident>,
    f: Expr,
}

// TODO: implicit return for stuff like `du! { f() >> g() }`
struct MacroInput {
    lines: Vec<Line>,
    ret: Expr,
}

impl Parse for Line {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
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

fn quote_expr(e: &Expr) -> proc_macro2::TokenStream {
    match e {
        Expr::Binary(binexp) => {
            match binexp.op {
                // |
                BinOp::BitOr(_) => {
                    let left = quote_expr(&binexp.left);
                    let right = quote_expr(&binexp.right);
                    quote! {
                        Box::new(move |input| #left(input).or_else(|| #right(input)))
                    }
                }

                // >>
                BinOp::Shr(_) => {
                    let left = quote_expr(&binexp.left);
                    let right = quote_expr(&binexp.right);
                    quote! {
                        Box::new(move |input| {
                            let (_, rest) = #left(input)?;
                            let (tree, rest) = #right(rest)?;
                            Some((tree, rest))
                        })
                    }
                }

                _ => unimplemented!(),
            }
        }
        Expr::Group(group) => quote_expr(&group.expr),
        Expr::Call(ExprCall { args, func, .. }) => {
            let args = args.clone().into_iter().map(|arg| quote_expr(&arg));
            quote! { #func(#(#args),*) }
        }
        _ => quote! { #e },
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
        let f = quote_expr(&input.lines[0].f);
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
                let f = quote_expr(&line.f);
                if let Some(x) = &line.x {
                    quote! { let (#x, rest) = #f(rest)?; }
                } else {
                    quote! { let (_, rest) = #f(rest)?; }
                }
            })
            .collect();
        let ret = input.ret;
        quote! {
            move |input| {
                #first_line
                #(#next_lines)*
                Some((#ret, rest))
            }
        }
        .into()
    }
}
