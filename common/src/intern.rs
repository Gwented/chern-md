use std::{collections::HashMap, path::Path};

// This thing scares me
const PRMITIVES_ARRAY: [&str; 31] = [
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
    "str", // 17
    "bool",
    "nil", // 19
    "BigInt",
    "BigFloat",
    "List",
    "Map",
    "Set", // 24
    "bind",
    "var", // 26
    "nest",
    "complex_rules", // 28
    "Len",
    "IsEmpty",
];

// I'm scared
// Also the interner shouldn't own this

pub struct Intern {
    map: HashMap<String, u32>,
    stored: Vec<String>,
    cursor: usize,
}

//TODO: CONCERNING INTRINSIC VALUES
impl Intern {
    pub fn init() -> Intern {
        let mut interner = Intern {
            map: HashMap::with_capacity(PRMITIVES_ARRAY.len()),
            stored: Vec::with_capacity(PRMITIVES_ARRAY.len()),
            cursor: PRMITIVES_ARRAY.len(),
        };

        // TODO: Is this ok?
        for (id, keyword) in PRMITIVES_ARRAY.iter().enumerate() {
            interner.map.insert(keyword.to_string(), id as u32);
            interner.stored.push(keyword.to_string());
        }

        interner
    }

    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(id) = self.map.get(s) {
            return *id;
        }

        let id = self.cursor as u32;
        self.cursor += 1;

        let new_str = s.to_string();

        self.map.insert(new_str.clone(), id);
        self.stored.push(new_str);

        id
    }

    pub fn is_reserved(&self, id: usize) -> bool {
        id < PRMITIVES_ARRAY.len()
    }

    // HOW DO I USE RANGE FOR THIS. I AM NEW TO THINKING.
    pub fn is_section(&self, id: u32) -> bool {
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
