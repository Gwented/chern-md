use std::{collections::HashMap, fmt::Display};

use crate::parser::symbols::Symbol;

//FIXME: Change to span of bytes
#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Token {
    Id(usize),
    Literal(usize),
    Number(usize),
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
    //FIXME: INTERN THIS
    Illegal(usize),
    EOF,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        match self {
            Token::Id(_) => TokenKind::Id,
            Token::Literal(_) => TokenKind::Literal,
            Token::Number(_) => TokenKind::Number,
            Token::OBracket => TokenKind::OBracket,
            Token::CBracket => TokenKind::CBracket,
            Token::OCurlyBracket => TokenKind::OCurlyBracket,
            Token::CCurlyBracket => TokenKind::CCurlyBracket,
            Token::QuestionMark => TokenKind::QuestionMark,
            Token::Equals => TokenKind::Equals,
            Token::OAngleBracket => TokenKind::OAngleBracket,
            Token::CAngleBracket => TokenKind::CAngleBracket,
            Token::Comma => TokenKind::Comma,
            Token::SlimArrow => TokenKind::SlimArrow,
            Token::Slash => TokenKind::Slash,
            Token::HashSymbol => TokenKind::HashSymbol,
            Token::Percent => TokenKind::Percent,
            Token::Colon => TokenKind::Colon,
            Token::OParen => TokenKind::OParen,
            Token::CParen => TokenKind::CParen,
            Token::Hyphen => TokenKind::Hyphen,
            Token::ExclamationPoint => TokenKind::ExclamationPoint,
            Token::Asterisk => TokenKind::Asterisk,
            //FIX: Not possible so may remove
            Token::DoubleQuotes => TokenKind::DoubleQuotes,
            Token::Tilde => TokenKind::Tilde,
            Token::Dot => TokenKind::Dot,
            Token::VerticalBar => TokenKind::VerticalBar,
            Token::Illegal(_) => TokenKind::Illegal,
            Token::EOF => TokenKind::EOF,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Token::Id(_) => write!(f, "identifier"),
            // Token::Literal(_) => write!(f, "literal"),
            Token::Number(num) => write!(f, "{num}"),
            Token::OBracket => write!(f, "["),
            Token::CBracket => write!(f, "]"),
            Token::OCurlyBracket => write!(f, "}}"),
            Token::CCurlyBracket => write!(f, "{{"),
            Token::QuestionMark => write!(f, "?"),
            Token::Equals => write!(f, "="),
            Token::OAngleBracket => write!(f, "<"),
            Token::CAngleBracket => write!(f, ">"),
            Token::Comma => write!(f, ","),
            Token::SlimArrow => write!(f, "->"),
            Token::Slash => write!(f, "/"),
            Token::HashSymbol => write!(f, "#"),
            Token::Percent => write!(f, "%"),
            Token::Colon => write!(f, ":"),
            Token::OParen => write!(f, "("),
            Token::CParen => write!(f, ")"),
            Token::Hyphen => write!(f, "-"),
            Token::ExclamationPoint => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::DoubleQuotes => write!(f, "\""),
            Token::Tilde => write!(f, "~"),
            Token::Dot => write!(f, "."),
            Token::VerticalBar => write!(f, "|"),
            // Token::Illegal(_) => write!(f, "Illegal"),
            Token::EOF => write!(f, "<eof>"),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenKind {
    Id,
    Literal,
    Number,
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
    Illegal,
    EOF,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Id => write!(f, "identifier"),
            TokenKind::Literal => write!(f, "literal"),
            TokenKind::Number => write!(f, "Number"),
            TokenKind::OBracket => write!(f, "["),
            TokenKind::CBracket => write!(f, "]"),
            TokenKind::OCurlyBracket => write!(f, "{{"),
            TokenKind::CCurlyBracket => write!(f, "}}"),
            TokenKind::QuestionMark => write!(f, "?"),
            TokenKind::Equals => write!(f, "="),
            TokenKind::OAngleBracket => write!(f, "<"),
            TokenKind::CAngleBracket => write!(f, ">"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::SlimArrow => write!(f, "->"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::HashSymbol => write!(f, "#"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::OParen => write!(f, "("),
            TokenKind::CParen => write!(f, ")"),
            TokenKind::Hyphen => write!(f, "-"),
            TokenKind::ExclamationPoint => write!(f, "!"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::DoubleQuotes => write!(f, "\""),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::VerticalBar => write!(f, "|"),
            TokenKind::Illegal => write!(f, "illegal"),
            TokenKind::EOF => write!(f, "<eof>"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    ln: usize,
    col: usize,
}

impl Span {
    pub fn new(ln: usize, col: usize) -> Span {
        Span { ln, col }
    }

    pub fn ln(&self) -> usize {
        self.ln
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InnerArgs {
    Warn,
    Scientific,
    Hex,
    Binary,
    Octo,
}
