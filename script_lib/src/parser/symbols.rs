use std::collections::HashMap;

use crate::token::{ActualType, InnerArgs};

//FIXME: Odd handling
//
#[derive(Debug)]
pub enum Symbol {
    Definition(TypeDef),
    Bind(Bind),
}

#[derive(Debug)]
pub struct SymbolTable {
    //Can just be a vec?
    symbols: HashMap<u32, Symbol>,
    type_ids: Vec<ActualType>,
    pos: usize,
}

impl SymbolTable {
    // In case table has something else added
    pub(crate) fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
            type_ids: Vec::new(),
            pos: 0,
        }
    }

    // Visitor pattern data encapsulation prototype pattern
    // Dog dog = new Dog();
    pub(crate) fn symbols(&self) -> &HashMap<u32, Symbol> {
        &self.symbols
    }

    // Jauva data incapsulation JAVA spring springboot
    pub(crate) fn type_ids(&self) -> &Vec<ActualType> {
        &self.type_ids
    }

    /// func is the worst function name variant I have ever heard
    pub(crate) fn reserve_id(&mut self) -> u32 {
        let type_id = self.pos;
        self.type_ids.push(ActualType::Nil);
        self.pos += 1;
        type_id as u32
    }

    pub(crate) fn store_basic(&mut self, symbol: Symbol, name_id: u32) {
        self.symbols.insert(name_id, symbol);
    }

    pub(crate) fn store_symbol(
        &mut self,
        symbol: Symbol,
        name_id: u32,
        type_id: u32,
        raw_type: ActualType,
    ) {
        self.type_ids[type_id as usize] = raw_type;
        self.symbols.insert(name_id, symbol);
    }

    pub(crate) fn search_symbol(&self, id: u32) -> &Symbol {
        &self.symbols[&id]
    }

    pub(crate) fn search_type(&self, id: usize) -> &ActualType {
        &self.type_ids[id]
    }
}

#[derive(Debug)]
//FIX: Give interner a list of pathbufs
pub struct Bind {
    pub(crate) id: u32,
}

impl Bind {
    pub fn new(id: u32) -> Bind {
        Bind { id }
    }
}

#[derive(Debug)]
pub struct TypeDef {
    // May be integer idk
    pub(crate) id: u32,
    // type_id: u32
    pub(crate) type_id: u32,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) cond: Vec<Cond>,
}

impl TypeDef {
    pub(crate) fn new(id: u32, type_id: u32, args: Vec<InnerArgs>, cond: Vec<Cond>) -> TypeDef {
        TypeDef {
            id,
            type_id,
            args,
            cond,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Cond {
    // Approximation operator is a range internally.
    // FIX: Possibly NEEDS usize but unsure
    // Should probably just be usize
    // Unsure whether to remove range or len
    Range(usize, usize),
    // Probably should just attach bool
    IsEmpty,
    Len(usize),
    // But should likely be removed
    Not(Box<Cond>),
}
