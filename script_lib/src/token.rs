use std::{collections::HashMap, fmt::Display};

use crate::parser::symbols::Symbol;

//FIXME: Change to span of bytes
#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub(crate) token: Token,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Token {
    Id(u32),
    Literal(u32),
    Number(u32),
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
    DotRange,
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
    Illegal(u32),
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
            Token::DotRange => TokenKind::DotRange,
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

//FIX: Should turn into impl method since I need the interner
// impl Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             // Token::Id(_) => write!(f, "identifier"),
//             // Token::Literal(_) => write!(f, "literal"),
//             Token::Number(num) => write!(f, "{num}"),
//             Token::OBracket => write!(f, "["),
//             Token::CBracket => write!(f, "]"),
//             Token::OCurlyBracket => write!(f, "}}"),
//             Token::CCurlyBracket => write!(f, "{{"),
//             Token::QuestionMark => write!(f, "?"),
//             Token::Equals => write!(f, "="),
//             Token::OAngleBracket => write!(f, "<"),
//             Token::CAngleBracket => write!(f, ">"),
//             Token::Comma => write!(f, ","),
//             Token::SlimArrow => write!(f, "->"),
//             Token::Slash => write!(f, "/"),
//             Token::HashSymbol => write!(f, "#"),
//             Token::Percent => write!(f, "%"),
//             Token::Colon => write!(f, ":"),
//             Token::OParen => write!(f, "("),
//             Token::CParen => write!(f, ")"),
//             Token::Hyphen => write!(f, "-"),
//             Token::ExclamationPoint => write!(f, "!"),
//             Token::Asterisk => write!(f, "*"),
//             Token::DoubleQuotes => write!(f, "\""),
//             Token::Tilde => write!(f, "~"),
//             Token::Dot => write!(f, "."),
//             Token::VerticalBar => write!(f, "|"),
//             // Token::Illegal(_) => write!(f, "Illegal"),
//             Token::EOF => write!(f, "<eof>"),
//             _ => unreachable!(),
//         }
//     }
// }

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum TokenKind {
    Id,
    Literal,
    // START OF TYPES
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
    Str,
    BigInt,
    BigFloat,
    List,
    Set,
    Map,
    // QUESTIONABLE
    Any,
    //
    Type,
    UserType,
    // END OF TYPES
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
    DotRange,
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
            TokenKind::Number => write!(f, "number"),
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
            TokenKind::DotRange => write!(f, "(range)"),
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
            TokenKind::I8 => write!(f, "i8"),
            TokenKind::U8 => write!(f, "u8"),
            TokenKind::I16 => write!(f, "i16"),
            TokenKind::U16 => write!(f, "u16"),
            TokenKind::F16 => write!(f, "f16"),
            TokenKind::I32 => write!(f, "i32"),
            TokenKind::U32 => write!(f, "u32"),
            TokenKind::F32 => write!(f, "f32"),
            TokenKind::I64 => write!(f, "i64"),
            TokenKind::U64 => write!(f, "u64"),
            TokenKind::F64 => write!(f, "f64"),
            TokenKind::I128 => write!(f, "i128"),
            TokenKind::U128 => write!(f, "u128"),
            TokenKind::F128 => write!(f, "f128"),
            TokenKind::Sized => write!(f, "sized"),
            TokenKind::Unsized => write!(f, "unsized"),
            TokenKind::Str => write!(f, "str"),
            TokenKind::BigInt => write!(f, "BigInt"),
            TokenKind::BigFloat => write!(f, "BigFloat"),
            TokenKind::Type => write!(f, "type"),
            TokenKind::List => write!(f, "List"),
            TokenKind::Set => write!(f, "Set"),
            TokenKind::Map => write!(f, "Map"),
            TokenKind::Any => write!(f, "Any"),
            TokenKind::UserType => write!(f, "User type"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Span {
    pub(crate) ln: usize,
    pub(crate) col: usize,
}

impl Span {
    pub(crate) fn new(ln: usize, col: usize) -> Span {
        Span { ln, col }
    }
}
//FIXME: Add enum and struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ActualType {
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
    Bool,
    Nil,
    Char,
    Str,
    BigInt,
    // Hex?
    BigFloat,
    List(Box<ActualType>),
    Set(Box<ActualType>),
    Map(Box<ActualType>, Box<ActualType>),
    // TODO: Figure out if this should be an 'init' type of type
    Any(Option<Box<ActualType>>),
    UserStruct,
    UserEnum,
}

// OR
pub struct StructuralType {
    pub(crate) id: usize,
    // pub(crate) ty: ActualType,
    // pub(crate) args: Vec<InnerArgs>,
    // pub(crate) cond: Vec<crate::parser::symbols::Cond>,
    // Box<ActualType>?
    pub(crate) children: Vec<u32>,
    // 64,000 fields????
    pub(crate) total_fields: u16,
}

//TODO: IS THIS
// pub struct UserStruct {
//     type_defs: Vec<TypeDef>,
// }
//
// pub struct UserEnum {
//     variants: Vec<usize>,
// }

//FIXME: Change match to actual enum name
impl TryFrom<u32> for ActualType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(ActualType::I8),
            1 => Ok(ActualType::U8),
            2 => Ok(ActualType::I16),
            3 => Ok(ActualType::U16),
            4 => Ok(ActualType::F16),
            5 => Ok(ActualType::I32),
            6 => Ok(ActualType::U32),
            7 => Ok(ActualType::F32),
            8 => Ok(ActualType::I64),
            9 => Ok(ActualType::U64),
            10 => Ok(ActualType::F64),
            11 => Ok(ActualType::I128),
            12 => Ok(ActualType::U128),
            13 => Ok(ActualType::F128),
            14 => Ok(ActualType::Sized),
            15 => Ok(ActualType::Unsized),
            16 => Ok(ActualType::Char),
            17 => Ok(ActualType::Str),
            18 => Ok(ActualType::Bool),
            19 => Ok(ActualType::Nil),
            20 => Ok(ActualType::BigInt),
            21 => Ok(ActualType::BigFloat),
            // 25 => Ok(ReservedKeyword::Bind),
            // 26 => Ok(ReservedKeyword::Var),
            // 27 => Ok(ReservedKeyword::Nest),
            // 28 => Ok(ReservedKeyword::ComplexRules),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InnerArgs {
    Warn,
    Scientific,
    Hex,
    Binary,
    Octo,
}

impl<'a> TryFrom<&'a str> for InnerArgs {
    type Error = &'a str;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        match v {
            "warn" => Ok(InnerArgs::Warn),
            "scientific" => Ok(InnerArgs::Scientific),
            "hex" => Ok(InnerArgs::Hex),
            "binary" => Ok(InnerArgs::Binary),
            "octo" => Ok(InnerArgs::Octo),
            v => Err(v),
        }
    }
}
