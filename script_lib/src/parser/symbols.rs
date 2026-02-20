use std::collections::HashMap;

use crate::token::{ActualType, InnerArgs};

//FIXME: Odd handling
//
#[derive(Debug)]
pub enum Symbol {
    Definition(TypeDef),
    Path { id: usize },
}

#[derive(Debug)]
pub struct TypeDef {
    // May be integer idk
    id: usize,
    ty: ActualType,
    args: Vec<InnerArgs>,
    cond: Vec<Cond>,
}

impl TypeDef {
    pub fn new(id: usize, ty: ActualType, args: Vec<InnerArgs>, cond: Vec<Cond>) -> TypeDef {
        TypeDef { id, ty, args, cond }
    }
}

#[derive(Debug)]
pub enum Cond {
    // Approximation operator is a range internally.
    Range(usize, usize),
    // Probably should just attach bool
    IsEmpty,
    Len(usize),
    // Ok this is kinda cool
    Not(Box<Cond>),
}

#[derive(Debug)]
pub struct Table {
    pub symbols: HashMap<usize, Symbol>,
}

impl Table {
    // In case table has something else added
    pub fn new() -> Table {
        Table {
            symbols: HashMap::new(),
        }
    }
}
