use std::collections::HashMap;

use crate::parser::Word;

#[derive(Debug)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug)]
pub enum Token {
    Id(String),
    Literal(String),
    Number(String),
    OBracket,
    CBracket,
    OCurlyBracket,
    CCurlyBracket,
    QuestionMark,
    Equals,
    OAngleBracket,
    CAngleBracket,
    Comma,
    SlimArrow,
    Slash,
    HashSymbol,
    Percent,
    Colon,
    OParen,
    CParen,
    Hyphen,
    // At,
    ExclamationPoint,
    Asterisk,
    DoubleQuotes,
    Tilde,
    Dot,
    VerticalBar,
    //TODO: Include branch or specific state of lexer. Maybe.
    Illegal(String),
    EOF,
}

#[derive(Debug)]
pub struct Span {
    ln: usize,
    col: usize,
}

impl Span {
    pub fn new(ln: usize, col: usize) -> Span {
        Span { ln, col }
    }
}

#[derive(Debug)]
pub enum ActualType {
    I8,
    U8,
    I16,
    U16,
    F16,
    I32,
    U32,
    F32,
    I64,
    U64,
    F64,
    I128,
    U128,
    F128,
    Sized,
    Unsized,
    Str(String),
    BigInt(String),
    // Hex?
    BigFloat(String),
    Array(Box<ActualType>),
    Set(Box<ActualType>),
    Map(Box<ActualType>, Box<ActualType>),
    Any,
    Custom(String),
}

#[derive(Debug)]
pub enum Cond {
    // Approximation operator is a range internally.
    Range(usize, usize),
    // Probably should just attach bool
    IsEmpty,
    Len(usize),
    // Ok this is kinda cool
    Not(Box<Cond>),
}

#[derive(Debug)]
pub enum InnerArgs {
    Warn,
    Scientific,
    Hex,
    Binary,
    Octo,
}

#[derive(Debug)]
pub struct Table {
    symbols: HashMap<u32, Vec<Word>>,
}
