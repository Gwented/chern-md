use std::fmt::Display;

use common::{
    keywords::{self, Keyword},
    symbols::TypedId,
};

// Not sure what to do with this..
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum Notation {
    Bin = 2,
    // Now this looks weird
    Decimal = 10,
    Octal = 8,
    Hex = 16,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Token {
    Id(u32),
    Str(u32),
    Integer(u32, Notation),
    Float(u32, Notation),
    Illegal(u32),
    Char(char),
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
    Plus,
    Asterisk,
    Hyphen,
    // At,
    ExclamationPoint,
    DoubleQuotes,
    Tilde,
    VerticalBar,
    Dot,
    Poison,
    EOF,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        match self {
            Token::Id(_) => TokenKind::Id,
            Token::Str(_) => TokenKind::Literal,
            Token::Integer(_, _) => TokenKind::Integer,
            Token::Float(_, _) => TokenKind::Float,
            Token::Char(_) => TokenKind::Char,
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
            Token::Plus => TokenKind::Plus,
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
pub enum TokenKind {
    Id,
    Literal,
    Integer,
    Float,
    Char,
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
    Plus,
    Hyphen,
    // At,
    ExclamationPoint,
    Asterisk,
    DoubleQuotes,
    Tilde,
    Dot,
    VerticalBar,
    Illegal,
    Poison,
    EOF,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Id => write!(f, "identifier"),
            TokenKind::Literal => write!(f, "string literal"),
            TokenKind::Integer => write!(f, "integer"),
            TokenKind::Float => write!(f, "float"),
            TokenKind::Char => write!(f, "character"),
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
            TokenKind::Plus => write!(f, "+"),
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
            TokenKind::Poison => write!(f, "<poisoned>"),
        }
    }
}

// IS THIS EVEN OPTIMAL?
// I DID NOT KNOW ABOUT PUB CONST AT ALL
// FIX: NEED CATCH ALL.
// Ok maybe it can stay an identifier since it makes lexing a bit weird.
pub const ID: u64 = 1 << 0;
pub const LITERAL: u64 = 1 << 1;
pub const INTEGER: u64 = 1 << 2;
pub const FLOAT: u64 = 1 << 3;
pub const CHAR: u64 = 1 << 4;
pub const O_BRACKET: u64 = 1 << 5;
pub const C_BRACKET: u64 = 1 << 6;
pub const O_CURLY_BRACKET: u64 = 1 << 7;
pub const C_CURLY_BRACKET: u64 = 1 << 8;
pub const QUESTION_MARK: u64 = 1 << 9;
pub const EQUALS: u64 = 1 << 10;
pub const WALRUS: u64 = 1 << 11;
pub const O_ANGLE_BRACKET: u64 = 1 << 12;
pub const C_ANGLE_BRACKET: u64 = 1 << 13;
pub const COMMA: u64 = 1 << 14;
pub const SLIM_ARROW: u64 = 1 << 15;
pub const SLASH: u64 = 1 << 16;
pub const HASH_SYMBOL: u64 = 1 << 17;
pub const DOT_RANGE: u64 = 1 << 18;
pub const PERCENT: u64 = 1 << 19;
pub const COLON: u64 = 1 << 20;
pub const O_PAREN: u64 = 1 << 21;
pub const C_PAREN: u64 = 1 << 22;
pub const PLUS: u64 = 1 << 23;
pub const HYPHEN: u64 = 1 << 24;
pub const ASTERISK: u64 = 1 << 25;
pub const EXCLAMATION_POINT: u64 = 1 << 26;
pub const DOUBLE_QUOTES: u64 = 1 << 27;
pub const TILDE: u64 = 1 << 28;
pub const DOT: u64 = 1 << 29;
pub const VERTICAL_BAR: u64 = 1 << 30;
pub const ILLEGAL: u64 = 1 << 31;
pub const POISON: u64 = 1 << 32;
pub const EOF: u64 = 1 << 33;

//FIX: PLEASE ASSERT THIS THING
impl TokenKind {
    pub fn to_u64(&self) -> u64 {
        // Ignore this...
        match self {
            TokenKind::Id => ID,
            TokenKind::Literal => LITERAL,
            TokenKind::Integer => INTEGER,
            TokenKind::Float => FLOAT,
            TokenKind::Char => CHAR,
            TokenKind::OBracket => O_BRACKET,
            TokenKind::CBracket => C_BRACKET,
            TokenKind::OCurlyBracket => O_CURLY_BRACKET,
            TokenKind::CCurlyBracket => C_CURLY_BRACKET,
            TokenKind::QuestionMark => QUESTION_MARK,
            TokenKind::Equals => EQUALS,
            TokenKind::Walrus => WALRUS,
            TokenKind::OAngleBracket => O_ANGLE_BRACKET,
            TokenKind::CAngleBracket => C_ANGLE_BRACKET,
            TokenKind::Comma => COMMA,
            TokenKind::SlimArrow => SLIM_ARROW,
            TokenKind::Slash => SLASH,
            TokenKind::HashSymbol => HASH_SYMBOL,
            TokenKind::DotRange => DOT_RANGE,
            TokenKind::Percent => PERCENT,
            TokenKind::Colon => COLON,
            TokenKind::OParen => O_PAREN,
            TokenKind::CParen => C_PAREN,
            TokenKind::Plus => PLUS,
            TokenKind::Hyphen => HYPHEN,
            TokenKind::ExclamationPoint => EXCLAMATION_POINT,
            TokenKind::Asterisk => ASTERISK,
            TokenKind::DoubleQuotes => DOUBLE_QUOTES,
            TokenKind::Tilde => TILDE,
            TokenKind::Dot => DOT,
            TokenKind::VerticalBar => VERTICAL_BAR,
            TokenKind::Illegal => ILLEGAL,
            TokenKind::Poison => POISON,
            TokenKind::EOF => EOF,
        }
    }
}

//TODO: Move
#[derive(Debug)]
pub enum BuiltinType {
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
    List(TypedId),
    Set(TypedId),
    Map(TypedId, TypedId),
    Any(Option<TypedId>),
}

impl BuiltinType {
    pub fn kind(&self) -> BuiltinTypeKind {
        match self {
            BuiltinType::I8 => BuiltinTypeKind::I8,
            BuiltinType::U8 => BuiltinTypeKind::U8,
            BuiltinType::I16 => BuiltinTypeKind::I16,
            BuiltinType::U16 => BuiltinTypeKind::U16,
            BuiltinType::F16 => BuiltinTypeKind::F16,
            BuiltinType::I32 => BuiltinTypeKind::I32,
            BuiltinType::U32 => BuiltinTypeKind::U32,
            BuiltinType::F32 => BuiltinTypeKind::F32,
            BuiltinType::I64 => BuiltinTypeKind::I64,
            BuiltinType::U64 => BuiltinTypeKind::U64,
            BuiltinType::F64 => BuiltinTypeKind::F64,
            BuiltinType::I128 => BuiltinTypeKind::I128,
            BuiltinType::U128 => BuiltinTypeKind::U128,
            BuiltinType::F128 => BuiltinTypeKind::F128,
            BuiltinType::Sized => BuiltinTypeKind::Sized,
            BuiltinType::Unsized => BuiltinTypeKind::Unsized,
            BuiltinType::Bool => BuiltinTypeKind::Bool,
            BuiltinType::Nil => BuiltinTypeKind::Nil,
            BuiltinType::Char => BuiltinTypeKind::Char,
            BuiltinType::Str => BuiltinTypeKind::Str,
            BuiltinType::BigInt => BuiltinTypeKind::BigInt,
            BuiltinType::BigFloat => BuiltinTypeKind::BigFloat,
            BuiltinType::List(_) => BuiltinTypeKind::List,
            BuiltinType::Set(_) => BuiltinTypeKind::Set,
            BuiltinType::Map(_, _) => BuiltinTypeKind::Map,
            BuiltinType::Any(_) => BuiltinTypeKind::Any,
        }
    }
}

pub enum BuiltinTypeKind {
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
}

// SHOULD THIS ERR?
impl BuiltinType {
    //TODO: Find out if one of these should be removed

    /// Uses `Keyword` to map directly to a `BuiltinType` excluding data structures.
    pub fn try_from_kw(kw: Keyword) -> Option<BuiltinType> {
        match kw {
            Keyword::I8 => Some(BuiltinType::I8),
            Keyword::U8 => Some(BuiltinType::U8),
            Keyword::I16 => Some(BuiltinType::I16),
            Keyword::U16 => Some(BuiltinType::U16),
            Keyword::F16 => Some(BuiltinType::F16),
            Keyword::I32 => Some(BuiltinType::I32),
            Keyword::U32 => Some(BuiltinType::U32),
            Keyword::F32 => Some(BuiltinType::F32),
            Keyword::I64 => Some(BuiltinType::I64),
            Keyword::U64 => Some(BuiltinType::U64),
            Keyword::F64 => Some(BuiltinType::F64),
            Keyword::I128 => Some(BuiltinType::I128),
            Keyword::U128 => Some(BuiltinType::U128),
            Keyword::F128 => Some(BuiltinType::F128),
            Keyword::Sized => Some(BuiltinType::Sized),
            Keyword::Unsized => Some(BuiltinType::Unsized),
            Keyword::Char => Some(BuiltinType::Char),
            Keyword::Str => Some(BuiltinType::Str),
            Keyword::Bool => Some(BuiltinType::Bool),
            Keyword::Nil => Some(BuiltinType::Nil),
            Keyword::BigInt => Some(BuiltinType::BigInt),
            Keyword::BigFloat => Some(BuiltinType::BigFloat),
            _ => None,
        }
    }

    //NOTE: This may still be replaced by a `BuiltinType` TypeExpr but seems fine
    /// Uses `Keyword` to map directly to a `BuiltinType` excluding data structures.
    pub fn try_from_id(id: u32) -> Option<BuiltinType> {
        match Keyword::try_as_kw(id) {
            Some(kw) => match kw {
                Keyword::I8 => Some(BuiltinType::I8),
                Keyword::U8 => Some(BuiltinType::U8),
                Keyword::I16 => Some(BuiltinType::I16),
                Keyword::U16 => Some(BuiltinType::U16),
                Keyword::F16 => Some(BuiltinType::F16),
                Keyword::I32 => Some(BuiltinType::I32),
                Keyword::U32 => Some(BuiltinType::U32),
                Keyword::F32 => Some(BuiltinType::F32),
                Keyword::I64 => Some(BuiltinType::I64),
                Keyword::U64 => Some(BuiltinType::U64),
                Keyword::F64 => Some(BuiltinType::F64),
                Keyword::I128 => Some(BuiltinType::I128),
                Keyword::U128 => Some(BuiltinType::U128),
                Keyword::F128 => Some(BuiltinType::F128),
                Keyword::Sized => Some(BuiltinType::Sized),
                Keyword::Unsized => Some(BuiltinType::Unsized),
                Keyword::Char => Some(BuiltinType::Char),
                Keyword::Str => Some(BuiltinType::Str),
                Keyword::Bool => Some(BuiltinType::Bool),
                Keyword::Nil => Some(BuiltinType::Nil),
                Keyword::BigInt => Some(BuiltinType::BigInt),
                Keyword::BigFloat => Some(BuiltinType::BigFloat),
                _ => None,
            },
            None => None,
        }
    }
}

//TEST:
impl Display for BuiltinTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinTypeKind::I8 => write!(f, "i8"),
            BuiltinTypeKind::U8 => write!(f, "u8"),
            BuiltinTypeKind::I16 => write!(f, "u16"),
            BuiltinTypeKind::U16 => write!(f, "u16"),
            BuiltinTypeKind::F16 => write!(f, "f16"),
            BuiltinTypeKind::I32 => write!(f, "i32"),
            BuiltinTypeKind::U32 => write!(f, "u32"),
            BuiltinTypeKind::F32 => write!(f, "f32"),
            BuiltinTypeKind::I64 => write!(f, "i64"),
            BuiltinTypeKind::U64 => write!(f, "u64"),
            BuiltinTypeKind::F64 => write!(f, "f64"),
            BuiltinTypeKind::I128 => write!(f, "i128"),
            BuiltinTypeKind::U128 => write!(f, "u128"),
            BuiltinTypeKind::F128 => write!(f, "f128"),
            BuiltinTypeKind::Sized => write!(f, "sized"),
            BuiltinTypeKind::Unsized => write!(f, "unsized"),
            BuiltinTypeKind::Str => write!(f, "str"),
            BuiltinTypeKind::Char => write!(f, "char"),
            BuiltinTypeKind::Nil => write!(f, "nil"),
            BuiltinTypeKind::Bool => write!(f, "bool"),
            BuiltinTypeKind::BigInt => write!(f, "BigInt"),
            BuiltinTypeKind::BigFloat => write!(f, "BigFloat"),
            BuiltinTypeKind::List => write!(f, "List"),
            BuiltinTypeKind::Set => write!(f, "Set"),
            BuiltinTypeKind::Map => write!(f, "Map"),
            BuiltinTypeKind::Any => write!(f, "Any"),
        }
    }
}

//FIXME:
// No
// PLEASE change this from a try_from
// Maybe
// Definitely
