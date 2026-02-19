use std::collections::HashMap;

// I'm scared
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

    // fn is_reserved(&self, id: usize) -> bool {
    //     id >= GLOBAL_KEYWORDS
    // }

    // JavaJAVAJVAVJAVJAJV
    pub fn search(&self, index: usize) -> &str {
        &self.stored[index]
    }
}
