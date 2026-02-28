use std::collections::HashMap;

use crate::token::{ActualType, Template};

//FIXME:
//MOVE ALL TO COMMON

#[derive(Debug)]
pub enum Symbol {
    Bind(Bind),
    Func(FunctionDef),
    Definition(TypeDef),
}

#[derive(Debug)]
pub struct SymbolTable {
    //Can just be a vec?
    symbols: HashMap<u32, Symbol>,
    // Rename registered something?
    type_ids: Vec<ActualType>,
    pos: usize,
}

//TODO: Worst struct in this entire project
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

    /// func is actually alright as a name
    pub(crate) fn reserve_id(&mut self) -> u32 {
        let type_id = self.pos;
        self.type_ids.push(ActualType::Nil);
        self.pos += 1;
        type_id as u32
    }

    pub(crate) fn store_basic(&mut self, symbol: Symbol, name_id: u32) {
        self.symbols.insert(name_id, symbol);
    }

    pub(crate) fn store_type(&mut self, actual_type: ActualType, type_id: u32) {
        self.type_ids[type_id as usize] = actual_type;
    }

    pub(crate) fn store_symbol(
        &mut self,
        name_id: u32,
        type_id: u32,
        raw_type: ActualType,
        symbol: Symbol,
    ) {
        self.type_ids[type_id as usize] = raw_type;
        self.symbols.insert(name_id, symbol);
    }

    pub(crate) fn search_symbol(&self, name_id: u32) -> &Symbol {
        &self.symbols[&name_id]
    }

    pub(crate) fn search_type(&self, type_id: usize) -> &ActualType {
        &self.type_ids[type_id]
    }

    pub(crate) fn search_type_mut(&mut self, id: usize) -> &mut ActualType {
        &mut self.type_ids[id]
    }
}

/// I have no comment on this.
#[derive(Debug)]
pub(crate) enum FuncArgs {
    Id(u32),
    Num(usize),
}

#[derive(Debug)]
//FIX: Give interner a list of pathbufs
pub struct Bind {
    pub(crate) name_id: u32,
}

impl Bind {
    pub fn new(id: u32) -> Bind {
        Bind { name_id: id }
    }
}

#[derive(Debug)]
pub struct TypeDef {
    // May be integer idk
    pub(crate) name_id: u32,
    pub(crate) type_id: u32,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) cond: Vec<Cond>,
}

impl TypeDef {
    pub(crate) fn new(
        name_id: u32,
        type_id: u32,
        args: Vec<InnerArgs>,
        cond: Vec<Cond>,
    ) -> TypeDef {
        TypeDef {
            name_id,
            type_id,
            args,
            cond,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Cond {
    // Approximation operator is a range internally.
    // Unsure whether to remove range or len
    Func(FunctionDef),
    // Probably should just attach bool
    Len(usize),
    // should likely be removed
    Not(Box<Cond>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InnerArgs {
    Warn,
    Scientific,
    Hex,
    Binary,
    Octo,
}

impl<'a> TryFrom<&'a str> for InnerArgs {
    type Error = &'a str;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        match v {
            "warn" => Ok(InnerArgs::Warn),
            "scientific" => Ok(InnerArgs::Scientific),
            "hex" => Ok(InnerArgs::Hex),
            "binary" => Ok(InnerArgs::Binary),
            "octo" => Ok(InnerArgs::Octo),
            v => Err(v),
        }
    }
}

#[derive(Debug)]
pub(crate) struct FunctionDef {
    pub(crate) name_id: u32,
    pub(crate) args: Vec<FuncArgs>,
}

impl FunctionDef {
    pub(crate) fn new(name_id: u32, args: Vec<FuncArgs>) -> FunctionDef {
        FunctionDef { name_id, args }
    }
}
