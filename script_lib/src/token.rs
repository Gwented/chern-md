use std::fmt::Display;

use common::{
    builtins::Keyword,
    symbols::{PrimitiveId, TypedId},
};

#[derive(Debug, Clone, Copy)]
pub enum Token {
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
pub enum TokenKind {
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

// IS THIS EVEN OPTIMAL?
// I DID NOT KNOW ABOUT PUB CONST AT ALL
pub const ID: u64 = 1 << 0;
pub const LITERAL: u64 = 1 << 1;
pub const NUMBER: u64 = 1 << 2;
pub const O_BRACKET: u64 = 1 << 3;
pub const C_BRACKET: u64 = 1 << 4;
pub const O_CURLY_BRACKET: u64 = 1 << 5;
pub const C_CURLY_BRACKET: u64 = 1 << 6;
pub const QUESTION_MARK: u64 = 1 << 7;
pub const EQUALS: u64 = 1 << 8;
pub const WALRUS: u64 = 1 << 9;
pub const O_ANGLE_BRACKET: u64 = 1 << 10;
pub const C_ANGLE_BRACKET: u64 = 1 << 11;
pub const COMMA: u64 = 1 << 12;
pub const SLIM_ARROW: u64 = 1 << 13;
pub const SLASH: u64 = 1 << 14;
pub const HASH_SYMBOL: u64 = 1 << 15;
pub const DOT_RANGE: u64 = 1 << 16;
pub const PERCENT: u64 = 1 << 17;
pub const COLON: u64 = 1 << 18;
pub const O_PAREN: u64 = 1 << 19;
pub const C_PAREN: u64 = 1 << 20;
pub const HYPHEN: u64 = 1 << 21;
pub const EXCLAMATION_POINT: u64 = 1 << 22;
pub const ASTERISK: u64 = 1 << 23;
pub const DOUBLE_QUOTES: u64 = 1 << 24;
pub const TILDE: u64 = 1 << 25;
pub const DOT: u64 = 1 << 26;
pub const VERTICAL_BAR: u64 = 1 << 27;
pub const ILLEGAL: u64 = 1 << 28;
pub const POISON: u64 = 1 << 29;
pub const EOF: u64 = 1 << 30;

impl TokenKind {
    pub fn to_u64(&self) -> u64 {
        // Ignore this...
        match self {
            TokenKind::Id => ID,
            TokenKind::Literal => LITERAL,
            TokenKind::Number => NUMBER,
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

//TODO: Call it builtin?
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
    // Template
    // TypeDef
    // ActualType
    // Maybe stay with typeident since List is NOT a primitive
    List(TypedId),
    Set(TypedId),
    // ActualType
    Map(TypedId, TypedId),
    // Activation from None
    Any(Option<TypedId>),
}

impl BuiltinType {
    pub fn kind(&self) -> ActualTypeKind {
        match self {
            BuiltinType::I8 => ActualTypeKind::I8,
            BuiltinType::U8 => ActualTypeKind::U8,
            BuiltinType::I16 => ActualTypeKind::I16,
            BuiltinType::U16 => ActualTypeKind::U16,
            BuiltinType::F16 => ActualTypeKind::F16,
            BuiltinType::I32 => ActualTypeKind::I32,
            BuiltinType::U32 => ActualTypeKind::U32,
            BuiltinType::F32 => ActualTypeKind::F32,
            BuiltinType::I64 => ActualTypeKind::I64,
            BuiltinType::U64 => ActualTypeKind::U64,
            BuiltinType::F64 => ActualTypeKind::F64,
            BuiltinType::I128 => ActualTypeKind::I128,
            BuiltinType::U128 => ActualTypeKind::U128,
            BuiltinType::F128 => ActualTypeKind::F128,
            BuiltinType::Sized => ActualTypeKind::Sized,
            BuiltinType::Unsized => ActualTypeKind::Unsized,
            BuiltinType::Bool => ActualTypeKind::Bool,
            BuiltinType::Nil => ActualTypeKind::Nil,
            BuiltinType::Char => ActualTypeKind::Char,
            BuiltinType::Str => ActualTypeKind::Str,
            BuiltinType::BigInt => ActualTypeKind::BigInt,
            BuiltinType::BigFloat => ActualTypeKind::BigFloat,
            BuiltinType::List(_) => ActualTypeKind::List,
            BuiltinType::Set(_) => ActualTypeKind::Set,
            BuiltinType::Map(_, _) => ActualTypeKind::Map,
            BuiltinType::Any(_) => ActualTypeKind::Any,
        }
    }
}

pub enum ActualTypeKind {
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
        }
    }
}

// The weight of every enum grows heavy, I don't know what isn't an enum anymore.
// Just one more enum.

//WARN: May not need to create
// #[derive(Debug)]
// pub struct EnumTemplate {
//     pub(crate) id: SymbolId,
//     pub(crate) args: Vec<InnerArgs>,
//     pub(crate) conds: Vec<Cond>,
//     pub(crate) variants: Vec<SymbolId>,
// }

//FIXME:
// No
// PLEASE change this from a try_from
// Maybe
impl TryFrom<u32> for BuiltinType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(BuiltinType::I8),
            1 => Ok(BuiltinType::U8),
            2 => Ok(BuiltinType::I16),
            3 => Ok(BuiltinType::U16),
            4 => Ok(BuiltinType::F16),
            5 => Ok(BuiltinType::I32),
            6 => Ok(BuiltinType::U32),
            7 => Ok(BuiltinType::F32),
            8 => Ok(BuiltinType::I64),
            9 => Ok(BuiltinType::U64),
            10 => Ok(BuiltinType::F64),
            11 => Ok(BuiltinType::I128),
            12 => Ok(BuiltinType::U128),
            13 => Ok(BuiltinType::F128),
            14 => Ok(BuiltinType::Sized),
            15 => Ok(BuiltinType::Unsized),
            16 => Ok(BuiltinType::Char),
            17 => Ok(BuiltinType::Str),
            18 => Ok(BuiltinType::Bool),
            19 => Ok(BuiltinType::Nil),
            20 => Ok(BuiltinType::BigInt),
            21 => Ok(BuiltinType::BigFloat),
            // 25 => Ok(ReservedKeyword::Bind),
            // 26 => Ok(ReservedKeyword::Var),
            // 27 => Ok(ReservedKeyword::Nest),
            // 28 => Ok(ReservedKeyword::ComplexRules),
            _ => Err(()),
        }
    }
}
