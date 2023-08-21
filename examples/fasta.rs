use rarsec::{
    combinators::{many, optional},
    du, parse, parser,
    text::{character, letter, newline, none_of, one_of},
};

#[derive(Debug, PartialEq)]
struct Sequence {
    desc: String,
    seq: String,
}

fn parse_line() -> parser!(String) {
    du! {
        newline();
        let chars <- many(letter() | one_of("*-"));
        return chars.iter().collect();
    }
}

fn parse_sequence() -> parser!(Sequence) {
    du! {
        character('>');
        let desc <- many(none_of("\n"));
        let seq <- many(parse_line());
        optional(newline());
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
    let seq = parse(many(parse_sequence()), test).unwrap();
    println!("{:#?}", seq);
    assert_eq!(
        seq,
        vec![
            Sequence {
                desc: "Amino acids".into(),
                seq: "ABCDEF".into()
            },
            Sequence {
                desc: "Nucleic acids".into(),
                seq: "ACGT".into()
            }
        ]
    );
}
