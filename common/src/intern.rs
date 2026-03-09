use std::{collections::HashMap, path::Path};

use crate::primitives;

// MAKE THE MACRO PLEASE
// What macro. What is a macro? What is hygiene?

pub struct Intern {
    map: HashMap<String, u32>,
    stored: Vec<String>,
    // Maybe not
    // stored_paths: Vec<OsString>
    pos: usize,
}

//TODO: CONCERNING INTRINSIC VALUES
impl Intern {
    pub fn init() -> Intern {
        let mut interner = Intern {
            map: HashMap::with_capacity(primitives::KEYWORDS_ARRAY.len()),
            stored: Vec::with_capacity(primitives::KEYWORDS_ARRAY.len()),
            pos: primitives::KEYWORDS_ARRAY.len(),
        };

        // TODO: Is this ok?
        for (id, keyword) in primitives::KEYWORDS_ARRAY.iter().enumerate() {
            interner.map.insert(keyword.to_string(), id as u32);
            interner.stored.push(keyword.to_string());
        }

        interner
    }

    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(id) = self.map.get(s) {
            return *id;
        }

        let id = self.pos as u32;
        self.pos += 1;

        let new_str = s.to_string();

        self.map.insert(new_str.clone(), id);
        self.stored.push(new_str);

        id
    }

    pub fn is_keyword(&self, id: usize) -> bool {
        id < primitives::KEYWORDS_ARRAY.len()
    }

    // Primitive being used loosely here...
    // SO IF I PUT ZERO, IT SCREAMS. BUT IF I USE ZERO UNDER A WRAPPER, ITS OK. RIGHT.
    pub fn is_primitive(&self, id: usize) -> bool {
        id >= usize::MIN && id <= 24
    }

    // TODO: Make unit test for this
    // No
    pub fn is_section(&self, id: u32) -> bool {
        if id >= 27 && id <= 30 {
            return true;
        }

        false
    }

    pub fn search(&self, index: usize) -> &str {
        &self.stored[index]
    }

    pub fn search_path(&self, index: usize) -> &Path {
        todo!()
    }
}
