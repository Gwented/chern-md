pub const INTRINSICS_ARRAY: [&str; 37] = [
    // primitives
    "i8", // 0
    "u8",
    "i16",
    "u16",
    "f16", // 4
    "i32",
    "u32", // 6
    "f32",
    "i64", // 8
    "u64",
    "f64", // 10
    "i128",
    "u128", // 12
    "f128",
    "sized", // 14
    "unsized",
    "char", // 16
    "str",
    "bool", // 18
    "nil",
    "BigInt", // 20
    "BigFloat",
    "List",
    "Map",
    "Set", // 24
    // structures
    "struct",
    "enum", // 26
    // Section names
    "bind",
    "var", // 28
    "nest",
    "complex", // 30
    // Directives
    "IsEmpty",
    "IsWhitespace", // 32
    "Range",
    "StartsW", // 34
    "EndsW",
    "Contains", // 36
];

// Keep a compact enum for code that prefers typed keyword identifiers.
#[repr(u32)]
pub enum PrimitiveKeywords {
    I8 = 0,
    U8 = 1,
    I16 = 2,
    U16 = 3,
    F16 = 4,
    I32 = 5,
    U32 = 6,
    F32 = 7,
    I64 = 8,
    U64 = 9,
    F64 = 10,
    I128 = 11,
    U128 = 12,
    F128 = 13,
    Sized = 14,
    Unsized = 15,
    Char = 16,
    Str = 17,
    Bool = 18,
    Nil = 19,
    BigInt = 20,
    BigFloat = 21,
    List = 22,
    Map = 23,
    Set = 24,
    Struct = 25,
    Enum = 26,
    Bind = 27,
    Var = 28,
    Nest = 29,
    Complex = 30,
    IsEmpty = 31,
    IsWhitespace = 32,
    Range = 33,
    StartsW = 34,
    EndsW = 35,
    Contains = 36,
}

impl PrimitiveKeywords {
    pub fn from_id(id: u32) -> Option<PrimitiveKeywords> {
        match id {
            0 => Some(PrimitiveKeywords::I8),
            1 => Some(PrimitiveKeywords::U8),
            2 => Some(PrimitiveKeywords::I16),
            3 => Some(PrimitiveKeywords::U16),
            4 => Some(PrimitiveKeywords::F16),
            5 => Some(PrimitiveKeywords::I32),
            6 => Some(PrimitiveKeywords::U32),
            7 => Some(PrimitiveKeywords::F32),
            8 => Some(PrimitiveKeywords::I64),
            9 => Some(PrimitiveKeywords::U64),
            10 => Some(PrimitiveKeywords::F64),
            11 => Some(PrimitiveKeywords::I128),
            12 => Some(PrimitiveKeywords::U128),
            13 => Some(PrimitiveKeywords::F128),
            14 => Some(PrimitiveKeywords::Sized),
            15 => Some(PrimitiveKeywords::Unsized),
            16 => Some(PrimitiveKeywords::Char),
            17 => Some(PrimitiveKeywords::Str),
            18 => Some(PrimitiveKeywords::Bool),
            19 => Some(PrimitiveKeywords::Nil),
            20 => Some(PrimitiveKeywords::BigInt),
            21 => Some(PrimitiveKeywords::BigFloat),
            22 => Some(PrimitiveKeywords::List),
            23 => Some(PrimitiveKeywords::Map),
            24 => Some(PrimitiveKeywords::Set),
            25 => Some(PrimitiveKeywords::Struct),
            26 => Some(PrimitiveKeywords::Enum),
            27 => Some(PrimitiveKeywords::Bind),
            28 => Some(PrimitiveKeywords::Var),
            29 => Some(PrimitiveKeywords::Nest),
            30 => Some(PrimitiveKeywords::Complex),
            31 => Some(PrimitiveKeywords::IsEmpty),
            32 => Some(PrimitiveKeywords::IsWhitespace),
            33 => Some(PrimitiveKeywords::Range),
            34 => Some(PrimitiveKeywords::StartsW),
            35 => Some(PrimitiveKeywords::EndsW),
            36 => Some(PrimitiveKeywords::Contains),
            _ => None,
        }
    }
}

pub fn is_primitive_id(id: usize) -> bool {
    id <= 24
}

pub fn is_section_id(id: usize) -> bool {
    (27..=30).contains(&id)
}
