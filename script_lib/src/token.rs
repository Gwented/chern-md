use std::fmt::Display;

use crate::parser::symbols::{Cond, InnerArgs, TypeDef};

#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub(crate) token: Token,
    pub(crate) span: Span,
}

// WARN: TEMP
#[derive(Debug, Clone, Copy)]
pub(crate) enum Token {
    Id(u32),
    Literal(u32),
    Number(u32),
    Illegal(u32),
    OParen,
    CParen,
    OBracket,
    CBracket,
    OCurlyBracket,
    CCurlyBracket,
    OAngleBracket,
    CAngleBracket,
    QuestionMark,
    Equals,
    Colon,
    // This name NEEDS to be changed
    Walrus,
    Comma,
    SlimArrow,
    DotRange,
    Slash,
    HashSymbol,
    Percent,
    Hyphen,
    // At,
    ExclamationPoint,
    Asterisk,
    DoubleQuotes,
    Tilde,
    Dot,
    VerticalBar,
    Poison,
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
            Token::Poison => TokenKind::Poison,
            Token::Walrus => TokenKind::Walrus,
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
            Token::DoubleQuotes => TokenKind::DoubleQuotes,
            Token::Tilde => TokenKind::Tilde,
            Token::Dot => TokenKind::Dot,
            Token::VerticalBar => TokenKind::VerticalBar,
            Token::Illegal(_) => TokenKind::Illegal,
            Token::EOF => TokenKind::EOF,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) enum TokenKind {
    Id,
    Literal,
    // START OF TYPES
    // I8,
    // U8,
    // I16,
    // U16,
    // F16,
    // I32,
    // U32,
    // F32,
    // I64,
    // U64,
    // F64,
    // I128,
    // U128,
    // F128,
    // Sized,
    // Unsized,
    // Str,
    // BigInt,
    // BigFloat,
    // List,
    // Set,
    // Map,
    // // QUESTIONABLE
    // Any,
    // //
    // Type,
    // UserType,
    // END OF TYPES
    Number,
    OBracket,
    CBracket,
    OCurlyBracket,
    CCurlyBracket,
    QuestionMark,
    Equals,
    Walrus,
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
    Poison,
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
            TokenKind::Walrus => write!(f, ":="),
            TokenKind::DoubleQuotes => write!(f, "\""),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::VerticalBar => write!(f, "|"),
            TokenKind::Illegal => write!(f, "illegal"),
            TokenKind::EOF => write!(f, "<eof>"),
            // TokenKind::I8 => write!(f, "i8"),
            // TokenKind::U8 => write!(f, "u8"),
            // TokenKind::I16 => write!(f, "i16"),
            // TokenKind::U16 => write!(f, "u16"),
            // TokenKind::F16 => write!(f, "f16"),
            // TokenKind::I32 => write!(f, "i32"),
            // TokenKind::U32 => write!(f, "u32"),
            // TokenKind::F32 => write!(f, "f32"),
            // TokenKind::I64 => write!(f, "i64"),
            // TokenKind::U64 => write!(f, "u64"),
            // TokenKind::F64 => write!(f, "f64"),
            // TokenKind::I128 => write!(f, "i128"),
            // TokenKind::U128 => write!(f, "u128"),
            // TokenKind::F128 => write!(f, "f128"),
            // TokenKind::Sized => write!(f, "sized"),
            // TokenKind::Unsized => write!(f, "unsized"),
            // TokenKind::Str => write!(f, "str"),
            // TokenKind::BigInt => write!(f, "BigInt"),
            // TokenKind::BigFloat => write!(f, "BigFloat"),
            // TokenKind::Type => write!(f, "type"),
            // TokenKind::List => write!(f, "List"),
            // TokenKind::Set => write!(f, "Set"),
            // TokenKind::Map => write!(f, "Map"),
            // TokenKind::Any => write!(f, "Any"),
            // TokenKind::UserType => write!(f, "User type"),
            TokenKind::Poison => write!(f, "<poisoned>"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Span {
    pub(crate) fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }
}

#[derive(Debug)]
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
    BigFloat,
    Template(Template),
    Definition(TypeDef),
    List(Box<ActualType>),
    Set(Box<ActualType>),
    Map(Box<ActualType>, Box<ActualType>),
    Any(Option<Box<ActualType>>),
}

impl ActualType {
    pub(crate) fn kind(&self) -> ActualTypeKind {
        match self {
            ActualType::I8 => ActualTypeKind::I8,
            ActualType::U8 => ActualTypeKind::U8,
            ActualType::I16 => ActualTypeKind::I16,
            ActualType::U16 => ActualTypeKind::U16,
            ActualType::F16 => ActualTypeKind::F16,
            ActualType::I32 => ActualTypeKind::I32,
            ActualType::U32 => ActualTypeKind::U32,
            ActualType::F32 => ActualTypeKind::F32,
            ActualType::I64 => ActualTypeKind::I64,
            ActualType::U64 => ActualTypeKind::U64,
            ActualType::F64 => ActualTypeKind::F64,
            ActualType::I128 => ActualTypeKind::I128,
            ActualType::U128 => ActualTypeKind::U128,
            ActualType::F128 => ActualTypeKind::F128,
            ActualType::Sized => ActualTypeKind::Sized,
            ActualType::Unsized => ActualTypeKind::Unsized,
            ActualType::Bool => ActualTypeKind::Bool,
            ActualType::Nil => ActualTypeKind::Nil,
            ActualType::Char => ActualTypeKind::Char,
            ActualType::Str => ActualTypeKind::Str,
            ActualType::BigInt => ActualTypeKind::BigInt,
            ActualType::BigFloat => ActualTypeKind::BigFloat,
            ActualType::Template(template) => ActualTypeKind::Template,
            ActualType::Definition(type_def) => ActualTypeKind::TypeDef,
            ActualType::List(actual_type) => ActualTypeKind::List,
            ActualType::Set(actual_type) => ActualTypeKind::Set,
            ActualType::Map(actual_type, actual_type1) => ActualTypeKind::Map,
            ActualType::Any(actual_type) => ActualTypeKind::Any,
        }
    }
}

pub(crate) enum ActualTypeKind {
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
    Char,
    Nil,
    Bool,
    BigInt,
    BigFloat,
    List,
    Set,
    Map,
    Any,
    TypeDef,
    Template,
}

//TEST:
impl Display for ActualTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActualTypeKind::I8 => write!(f, "i8"),
            ActualTypeKind::U8 => write!(f, "u8"),
            ActualTypeKind::I16 => write!(f, "u16"),
            ActualTypeKind::U16 => write!(f, "u16"),
            ActualTypeKind::F16 => write!(f, "f16"),
            ActualTypeKind::I32 => write!(f, "i32"),
            ActualTypeKind::U32 => write!(f, "u32"),
            ActualTypeKind::F32 => write!(f, "f32"),
            ActualTypeKind::I64 => write!(f, "i64"),
            ActualTypeKind::U64 => write!(f, "u64"),
            ActualTypeKind::F64 => write!(f, "f64"),
            ActualTypeKind::I128 => write!(f, "i128"),
            ActualTypeKind::U128 => write!(f, "u128"),
            ActualTypeKind::F128 => write!(f, "f128"),
            ActualTypeKind::Sized => write!(f, "sized"),
            ActualTypeKind::Unsized => write!(f, "unsized"),
            ActualTypeKind::Str => write!(f, "str"),
            ActualTypeKind::Char => write!(f, "char"),
            ActualTypeKind::Nil => write!(f, "nil"),
            ActualTypeKind::Bool => write!(f, "bool"),
            ActualTypeKind::BigInt => write!(f, "BigInt"),
            ActualTypeKind::BigFloat => write!(f, "BigFloat"),
            ActualTypeKind::List => write!(f, "List"),
            ActualTypeKind::Set => write!(f, "Set"),
            ActualTypeKind::Map => write!(f, "Map"),
            ActualTypeKind::Any => write!(f, "Any"),
            ActualTypeKind::TypeDef => write!(f, "TypeDef"),
            ActualTypeKind::Template => write!(f, "Template"),
        }
    }
}

// The weight of every enum grows heavy, I don't know what isn't an enum anymore.
// Just one more enum.

#[derive(Debug)]
pub struct Template {
    pub(crate) symbol_id: u32,
    pub(crate) args: Vec<InnerArgs>,
    // May remove conditions
    pub(crate) conds: Vec<Cond>,
    // Fields can be variants or separate strugg <-- Sgwom
    //WARN: Maybe (u32, u32) can return
    pub(crate) fields: Vec<u32>,
    // Should it just be ids, or ids and type ids?
}

impl Template {
    pub fn new(symbol_id: u32) -> Template {
        Template {
            symbol_id,
            args: Vec::new(),
            conds: Vec::new(),
            fields: Vec::new(),
        }
    }
}

// I DONT WANT TO MAKE THIS
#[derive(Debug)]
pub struct EnumTemplate {
    pub(crate) id: u32,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) conds: Vec<Cond>,
    pub(crate) variants: Vec<u32>,
}

//FIXME: Change match to actual enum name
// No
// PLEASE change this from a try_from
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
