use std::{collections::HashMap, path::Path};

// I'm scared
// Also the interner shouldn't own this
// Also add null
const GLOBAL_KEYWORDS: [&str; 29] = [
    "i8",
    "u8",
    "i16",
    "u16",
    "f16", // 4
    "i32",
    "u32",
    "f32",
    "i64",
    "u64", // 9
    "f64",
    "i128",
    "u128",
    "f128",
    "sized", // 14
    "unsized",
    "char",
    "str",
    "bool",
    "nil", // 19
    "BigInt",
    "BigFloat",
    "Array",
    "Map",
    "Set", // 24
    "bind",
    "var", // 26
    "nest",
    "complex_rules", // 28
];

pub enum ReservedKeyword {
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
    Array = 22,
    Map = 23,
    Set = 24,
    Bind = 25,
    Var = 26,
    Nest = 27,
    ComplexRules = 28,
}

//FIXME: IM SCARED OF THIS
impl TryFrom<usize> for ReservedKeyword {
    type Error = ();

    fn try_from(val: usize) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(ReservedKeyword::I8),
            1 => Ok(ReservedKeyword::U8),
            2 => Ok(ReservedKeyword::I16),
            3 => Ok(ReservedKeyword::U16),
            4 => Ok(ReservedKeyword::F16),
            5 => Ok(ReservedKeyword::I32),
            6 => Ok(ReservedKeyword::U32),
            7 => Ok(ReservedKeyword::F32),
            8 => Ok(ReservedKeyword::I64),
            9 => Ok(ReservedKeyword::U64),
            10 => Ok(ReservedKeyword::F64),
            11 => Ok(ReservedKeyword::I128),
            12 => Ok(ReservedKeyword::U128),
            13 => Ok(ReservedKeyword::F128),
            14 => Ok(ReservedKeyword::Sized),
            15 => Ok(ReservedKeyword::Unsized),
            16 => Ok(ReservedKeyword::Char),
            17 => Ok(ReservedKeyword::Str),
            18 => Ok(ReservedKeyword::Bool),
            19 => Ok(ReservedKeyword::Nil),
            20 => Ok(ReservedKeyword::BigInt),
            21 => Ok(ReservedKeyword::BigFloat),
            22 => Ok(ReservedKeyword::Array),
            23 => Ok(ReservedKeyword::Map),
            24 => Ok(ReservedKeyword::Set),
            // 25 => Ok(ReservedKeyword::Bind),
            // 26 => Ok(ReservedKeyword::Var),
            // 27 => Ok(ReservedKeyword::Nest),
            // 28 => Ok(ReservedKeyword::ComplexRules),
            _ => Err(()),
        }
    }
}

pub struct Intern {
    // Onboarding
    map: HashMap<String, usize>,
    // Actual search
    stored: Vec<String>,
    // 80 BYTES
    cursor: usize,
}

//TODO: CONCERNING INTRINSIC VALUES
impl Intern {
    pub fn new() -> Intern {
        let mut interner = Intern {
            map: HashMap::with_capacity(GLOBAL_KEYWORDS.len()),
            stored: Vec::with_capacity(GLOBAL_KEYWORDS.len()),
            cursor: GLOBAL_KEYWORDS.len(),
        };

        // TODO: Is this ok?
        for (id, keyword) in GLOBAL_KEYWORDS.iter().enumerate() {
            interner.map.insert(keyword.to_string(), id);
            interner.stored.push(keyword.to_string());
        }

        interner
    }

    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(id) = self.map.get(s) {
            return *id;
        }

        let id = self.cursor;
        self.cursor += 1;

        let new_str = s.to_string();

        self.map.insert(new_str.clone(), id);
        self.stored.push(new_str);

        id
    }

    pub fn is_reserved(&self, id: usize) -> bool {
        id < GLOBAL_KEYWORDS.len()
    }

    // HOW DO I USE RANGE FOR THIS. I AM NEW TO THINKING.
    pub fn is_section(&self, id: usize) -> bool {
        if id >= 25 && id <= 28 {
            return true;
        }

        false
    }

    // JavaJAVAJVAVJAVJAJV
    pub fn search(&self, index: usize) -> &str {
        &self.stored[index]
    }

    pub fn search_path(&self, index: usize) -> &Path {
        todo!()
    }
}
