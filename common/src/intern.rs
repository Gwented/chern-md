use std::{collections::HashMap, path::Path};

use crate::builtins;

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
            map: HashMap::with_capacity(builtins::KEYWORDS_ARRAY.len()),
            stored: Vec::with_capacity(builtins::KEYWORDS_ARRAY.len()),
            pos: builtins::KEYWORDS_ARRAY.len(),
        };

        // TODO: Is this ok?
        for (id, keyword) in builtins::KEYWORDS_ARRAY.iter().enumerate() {
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

    // Primitive being used loosely here...
    // SO IF I PUT ZERO, IT SCREAMS. BUT IF I USE ZERO UNDER A WRAPPER, ITS OK. RIGHT.

    // TODO: Make unit test for this
    // No
    pub fn search(&self, index: usize) -> &str {
        &self.stored[index]
    }

    pub fn search_path(&self, index: usize) -> &Path {
        todo!()
    }
}
