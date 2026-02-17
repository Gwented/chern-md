use std::collections::HashMap;

pub struct Intern {
    // Onboarding
    map: HashMap<String, usize>,
    // Actual search
    stored: Vec<String>,
    // 80 BYTES
    pos: usize,
}

//TODO: CONCERNING INTRINSIC VALUES
impl Intern {
    /// Reserved ids
    /// 0: bind
    /// 1: var
    /// 2: nest
    /// 3: complex_rules
    pub fn new() -> Intern {
        Intern {
            map: HashMap::new(),
            stored: vec![
                //TODO: Trying something
                '\0'.to_string(),
                '\0'.to_string(),
                '\0'.to_string(),
                '\0'.to_string(),
            ],
            pos: 4,
        }
    }

    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(id) = self.map.get(s) {
            return *id;
        }

        let id = self.pos;
        self.pos += 1;

        let new_str = s.to_string();

        self.map.insert(new_str.clone(), id);
        self.stored.push(new_str);

        id
    }

    // JavaJAVAJVAVJAVJAJV
    pub fn search(&self, index: usize) -> &str {
        &self.stored[index]
    }
}
