pub static KEYWORDS_ARRAY: [&str; 38] = [
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
    "override",
    //TODO: Add override
    // Directives
    "IsEmpty",
    "IsWhitespace", // 32
    "Range",
    "StartsW", // 34
    "EndsW",
    "Contains", // 36
];

// Keep a compact enum for code that prefers typed keyword identifiers.
// I think I don't know I am new to thinking does anyone have beginner thoughts?
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Keyword {
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
    Override = 31,
    IsEmpty = 32,
    IsWhitespace = 33,
    Range = 34,
    StartsW = 35,
    EndsW = 36,
    Contains = 37,
}

impl Keyword {
    pub fn try_as_kw(id: u32) -> Option<Keyword> {
        match id {
            // Using literal because scared of if
            0 => Some(Keyword::I8),
            1 => Some(Keyword::U8),
            2 => Some(Keyword::I16),
            3 => Some(Keyword::U16),
            4 => Some(Keyword::F16),
            5 => Some(Keyword::I32),
            6 => Some(Keyword::U32),
            7 => Some(Keyword::F32),
            8 => Some(Keyword::I64),
            9 => Some(Keyword::U64),
            10 => Some(Keyword::F64),
            11 => Some(Keyword::I128),
            12 => Some(Keyword::U128),
            13 => Some(Keyword::F128),
            14 => Some(Keyword::Sized),
            15 => Some(Keyword::Unsized),
            16 => Some(Keyword::Char),
            17 => Some(Keyword::Str),
            18 => Some(Keyword::Bool),
            19 => Some(Keyword::Nil),
            20 => Some(Keyword::BigInt),
            21 => Some(Keyword::BigFloat),
            22 => Some(Keyword::List),
            23 => Some(Keyword::Map),
            24 => Some(Keyword::Set),
            25 => Some(Keyword::Struct),
            26 => Some(Keyword::Enum),
            27 => Some(Keyword::Bind),
            28 => Some(Keyword::Var),
            29 => Some(Keyword::Nest),
            30 => Some(Keyword::Complex),
            31 => Some(Keyword::Override),
            32 => Some(Keyword::IsEmpty),
            33 => Some(Keyword::IsWhitespace),
            34 => Some(Keyword::Range),
            35 => Some(Keyword::StartsW),
            36 => Some(Keyword::EndsW),
            37 => Some(Keyword::Contains),
            _ => None,
        }
    }

    pub fn try_as_prim(id: u32) -> Option<Keyword> {
        if let Some(kw) = Self::try_as_kw(id) {
            match kw {
                Keyword::I8
                | Keyword::U8
                | Keyword::I16
                | Keyword::U16
                | Keyword::F16
                | Keyword::I32
                | Keyword::U32
                | Keyword::F32
                | Keyword::I64
                | Keyword::U64
                | Keyword::F64
                | Keyword::I128
                | Keyword::U128
                | Keyword::F128
                | Keyword::Sized
                | Keyword::Unsized
                | Keyword::Char
                | Keyword::Str
                | Keyword::Bool
                | Keyword::Nil
                | Keyword::BigInt
                | Keyword::BigFloat => return Some(kw),
                _ => return None,
            }
        }

        None
    }

    pub fn try_as_data_struct(id: u32) -> Option<Keyword> {
        if let Some(kw) = Self::try_as_kw(id) {
            match kw {
                Keyword::List | Keyword::Map => todo!(),
                Keyword::Set => return Some(kw),
                _ => return None,
            }
        }

        None
    }

    pub fn try_as_cond(id: u32) -> Option<Keyword> {
        if let Some(kw) = Self::try_as_kw(id) {
            match kw {
                Keyword::IsEmpty
                | Keyword::IsWhitespace
                | Keyword::Range
                | Keyword::StartsW
                | Keyword::EndsW
                | Keyword::Contains => return Some(kw),
                _ => return None,
            }
        }

        None
    }
}

pub fn is_type(id: u32) -> bool {
    id <= 24
}

pub fn is_section(id: u32) -> bool {
    (27..=31).contains(&id)
}
