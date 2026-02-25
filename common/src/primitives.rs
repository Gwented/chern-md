// i8
// u8
// i16
// u16
// f16
// i32
// u32
// f32
// i64
// u64
// f64
// i128
// u128
// f128
// sized
// unsized
// char
// str
// bool
// nil
// BigInt
// BigFloat
// List
// Map
// Set
// bind
// var
// nest
// complex_rules

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
    Bind = 25,
    Var = 26,
    Nest = 27,
    ComplexRules = 28,
}
//FIX: Maybe should reserve function names for checks

impl TryFrom<u32> for PrimitiveKeywords {
    type Error = ();

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(PrimitiveKeywords::I8),
            1 => Ok(PrimitiveKeywords::U8),
            2 => Ok(PrimitiveKeywords::I16),
            3 => Ok(PrimitiveKeywords::U16),
            4 => Ok(PrimitiveKeywords::F16),
            5 => Ok(PrimitiveKeywords::I32),
            6 => Ok(PrimitiveKeywords::U32),
            7 => Ok(PrimitiveKeywords::F32),
            8 => Ok(PrimitiveKeywords::I64),
            9 => Ok(PrimitiveKeywords::U64),
            10 => Ok(PrimitiveKeywords::F64),
            11 => Ok(PrimitiveKeywords::I128),
            12 => Ok(PrimitiveKeywords::U128),
            13 => Ok(PrimitiveKeywords::F128),
            14 => Ok(PrimitiveKeywords::Sized),
            15 => Ok(PrimitiveKeywords::Unsized),
            16 => Ok(PrimitiveKeywords::Char),
            17 => Ok(PrimitiveKeywords::Str),
            18 => Ok(PrimitiveKeywords::Bool),
            19 => Ok(PrimitiveKeywords::Nil),
            20 => Ok(PrimitiveKeywords::BigInt),
            21 => Ok(PrimitiveKeywords::BigFloat),
            22 => Ok(PrimitiveKeywords::List),
            23 => Ok(PrimitiveKeywords::Map),
            24 => Ok(PrimitiveKeywords::Set),
            // 25 => Ok(ReservedKeyword::Bind),
            // 26 => Ok(ReservedKeyword::Var),
            // 27 => Ok(ReservedKeyword::Nest),
            // 28 => Ok(ReservedKeyword::ComplexRules),
            _ => Err(()),
        }
    }
}
