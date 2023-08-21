# Rarsec

Have you ever wanted to use Haskell's Parsec in a lower-level language with
more complicated semantics for higher-order functions? Me neither.

# Example

Parse FASTA files:
```rust
use rarsec::{
    combinators::{many, optional},
    du, parser,
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
```

Compared to my original version in Haskell:
```hs
import Data.Char
import Data.Functor
import Text.Parsec
import Text.Parsec.String

data Sequence = Sequence {description :: String, seq :: String} deriving (Show, Eq)

parseLine :: Parser String
parseLine = newline >> many (letter <|> oneOf "*-")

parseSequence :: Parser Sequence
parseSequence = do
    char '>'
    description <- many $ noneOf "\n"
    seq <- many parseLine
    optional newline
    return $ Sequence description (map toUpper $ concat seq)
```
