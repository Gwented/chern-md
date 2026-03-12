use std::{collections::HashMap, path::Path};

use crate::keywords;

// MAKE THE MACRO PLEASE
// What macro. What is a macro? What is hygiene?

pub struct Intern {
    map: HashMap<String, u32>,
    stored: Vec<String>,
    // Maybe not
    // stored_paths: Vec<OsString>
    pos: usize,
}

impl Intern {
    pub fn init() -> Intern {
        let mut interner = Intern {
            map: HashMap::with_capacity(keywords::KEYWORDS_ARRAY.len()),
            stored: Vec::with_capacity(keywords::KEYWORDS_ARRAY.len()),
            pos: keywords::KEYWORDS_ARRAY.len(),
        };

        for (id, keyword) in keywords::KEYWORDS_ARRAY.iter().enumerate() {
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

    pub fn search(&self, index: usize) -> &str {
        &self.stored[index]
    }

    pub fn search_path(&self, index: usize) -> &Path {
        todo!()
    }
}
