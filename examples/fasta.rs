use rarsec::{combinators, du, text, Parser};

#[allow(unused)]
#[derive(Debug)]
struct Sequence {
    desc: String,
    seq: String,
}

fn parse_line() -> Parser<String> {
    du! {
        text::newline();
        let chars <- combinators::many(
            combinators::choice(
                text::letter(),
                text::one_of("*-")
            )
        );
        return chars.iter().collect();
    }
}

fn parse_sequence() -> Parser<Sequence> {
    du! {
        text::character('>');
        let desc <- combinators::many(text::none_of("\n"));
        let seq <- combinators::many(parse_line());
        combinators::optional(text::newline());
        return Sequence {
            desc: desc.iter().collect(),
            seq: seq.iter().flat_map(|s| s.chars()).collect(),
        };
    }
}

fn main() {
    let test = r#">Amino acids
ABC
DEF

>Nucleic acids
A
C
G
T"#;
    let (seq, _) = combinators::many(parse_sequence())(test).unwrap();
    println!("{:#?}", seq);
}
