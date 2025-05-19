use logos::{Logos, SpannedIter};

fn main() {
    let mut lexer = Token::lexer("-1");

    while let Some(val) = lexer.next() {
        println!("{val:?}: {:?}", lexer.slice());
    }
}


#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[token("-")]
    Sub,

    #[regex(r"[0-9]+", |output| {
        output.slice().parse::<i64>().ok()
    })]
    IntLiteral(i64),

    // #[regex(r"(  (([0-9]+[eE][+-]?[0-9]+)|([0-9]*\.[0-9]+[eE][+-]?[0-9]+|[0-9]*\.[0-9]+)))", |output| {    // alternative without [+-]?
    #[regex(r"([+-]?(([0-9]+[eE][+-]?[0-9]+)|([0-9]*\.[0-9]+[eE][+-]?[0-9]+|[0-9]*\.[0-9]+)))", |output| {
            output.slice().parse::<f64>().ok()
    })]
    FloatLiteral(f64),
}


/// Holds the state of the lexical analyzer.
pub struct Lexer<'a> {
    /// The iterator over tokens including their spans (locations).
    _token_stream: SpannedIter<'a, Token>,
}

impl<'input> Lexer<'input> {
    /// Construct a new `Lexer` from an input string holding the source code.
    pub fn new(input: &'input str) -> Self {
        // The `Token::lexer()` method is provided by the `Logos` derive macro.
        Self {
            _token_stream: Token::lexer(input).spanned(),
        }
    }
}


#[test]
fn neg_int_two_tokens() {
    let mut lexer = Token::lexer("-1");
    assert_eq!(lexer.next(), Some(Ok(Token::Sub)));
    assert_eq!(lexer.next(), Some(Ok(Token::IntLiteral(1))));
    assert_eq!(lexer.next(), None);
}


#[test]
fn float_literals() {
    for (input, output) in [
        ("1.0", Token::FloatLiteral(1.0)),
        (".01", Token::FloatLiteral(0.01)),
        ("3.1e-12", Token::FloatLiteral(3.1e-12)),
        ("2E3", Token::FloatLiteral(2E3)),
        ("1.5", Token::FloatLiteral(1.5)),
        ("-1.5", Token::FloatLiteral(-1.5)),
    ] {
        let token = Token::lexer(input).next();
        assert_eq!(token, Some(Ok(output)));
    }
}