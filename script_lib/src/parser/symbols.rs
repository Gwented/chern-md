use std::{collections::HashMap, path::PathBuf};

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
    pub(crate) ty: ActualType,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) cond: Vec<Cond>,
}

impl TypeDef {
    pub(crate) fn new(id: u32, ty: ActualType, args: Vec<InnerArgs>, cond: Vec<Cond>) -> TypeDef {
        TypeDef { id, ty, args, cond }
    }
}

#[derive(Debug)]
pub(crate) enum Cond {
    // Approximation operator is a range internally.
    Range(usize, usize),
    // Probably should just attach bool
    IsEmpty,
    Len(usize),
    // Ok this is kinda cool
    // but should likely be removed
    Not(Box<Cond>),
}

#[derive(Debug)]
pub struct SymbolTable {
    pub symbols: HashMap<u32, Symbol>,
}

impl SymbolTable {
    // In case table has something else added
    pub(crate) fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }
}
