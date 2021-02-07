use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub(crate) enum Token {
    #[regex("[a-zA-Z]+", |lex| String::from(lex.slice()))]
    Identifier(String),
    #[regex("-?[0-9]+(.[0-9]+)?", |lex| lex.slice().parse())]
    Number(f64),

    #[regex(r#""[^"]*""#, |lex| String::from(lex.slice()))]
    VString(String),

    #[regex("[\r\n]+")]
    #[regex(r"#[^\n]*\n")]
    NewLine,
    #[token("=")]
    Equal,
    #[token(",")]
    Comma,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[error]
    #[regex(r"[ \t\f]", logos::skip)]
    Error,
}
