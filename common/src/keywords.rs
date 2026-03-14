//TEST:
pub const LARGEST_KW: usize = 12;

pub static KEYWORDS_ARRAY: [&str; 39] = [
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
    // Statements
    "bind", // 27
    "alias",
    // Section names
    "var", // 29
    "nest",
    "complex", // 31
    "override",
    // Directives
    "IsEmpty",
    "IsWhitespace", // 34
    "Range",
    "StartsW", // 36
    "EndsW",
    "Contains", // 38
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
    Alias = 28,
    Var = 29,
    Nest = 30,
    Complex = 31,
    Override = 32,
    IsEmpty = 33,
    IsWhitespace = 34,
    Range = 35,
    StartsW = 36,
    EndsW = 37,
    Contains = 38,
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
            28 => Some(Keyword::Alias),
            29 => Some(Keyword::Var),
            30 => Some(Keyword::Nest),
            31 => Some(Keyword::Complex),
            32 => Some(Keyword::Override),
            33 => Some(Keyword::IsEmpty),
            34 => Some(Keyword::IsWhitespace),
            35 => Some(Keyword::Range),
            36 => Some(Keyword::StartsW),
            37 => Some(Keyword::EndsW),
            38 => Some(Keyword::Contains),
            _ => None,
        }
    }

    pub fn try_as_builtin(id: u32) -> Option<Keyword> {
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

    // pub fn try_as_cond(id: u32) -> Option<Keyword> {
    //     if let Some(kw) = Self::try_as_kw(id) {
    //         match kw {
    //             Keyword::IsEmpty
    //             | Keyword::IsWhitespace
    //             | Keyword::Range
    //             | Keyword::StartsW
    //             | Keyword::EndsW
    //             | Keyword::Contains => return Some(kw),
    //             _ => return None,
    //         }
    //     }
    //
    //     None
    // }
}

//TODO: Assert these
pub fn is_type(id: u32) -> bool {
    id <= 24
}

pub fn is_section(id: u32) -> bool {
    (29..=32).contains(&id)
}
