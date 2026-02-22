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

//FIXME: May need type ids
#[derive(Debug)]
pub struct SymbolTable {
    //Can just be a vec?
    symbols: HashMap<u32, Symbol>,
    type_ids: Vec<ActualType>,
    cursor: usize,
}

impl SymbolTable {
    // In case table has something else added
    pub(crate) fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
            type_ids: Vec::new(),
            cursor: 0,
        }
    }

    /// To make the type I either have to make dedicated funcs
    /// or reserve it first. Reserving makes no assumptions of position and just
    /// reserves a single slot. Will be null but impossible to access.
    pub(crate) fn reserve_t_id(&mut self) -> u32 {
        let type_id = self.cursor;
        self.type_ids.push(ActualType::Nil);
        self.cursor += 1;
        type_id as u32
    }

    pub(crate) fn store_basic(&mut self, symbol: Symbol, name_id: u32) {
        self.symbols.insert(name_id, symbol);
    }

    pub(crate) fn store_complex(
        &mut self,
        symbol: Symbol,
        name_id: u32,
        type_id: u32,
        raw_type: ActualType,
    ) {
        self.type_ids[type_id as usize] = raw_type;
        self.symbols.insert(name_id, symbol);
    }

    pub(crate) fn search(&self, id: u32) -> &Symbol {
        &self.symbols[&id]
    }
}

#[derive(Debug)]
pub(crate) enum Cond {
    // Approximation operator is a range internally.
    // FIX: Possibly NEEDS usize but unsure
    // Should probably just be usize
    Range(usize, usize),
    // Probably should just attach bool
    IsEmpty,
    Len(usize),
    // Ok this is kinda cool
    // but should likely be removed
    Not(Box<Cond>),
}
