use std::collections::HashMap;

use common::{
    intern::Intern,
    symbols::{NameId, SymbolId, TypeIdent},
};

use crate::{
    parser::{
        ast::{AbstractBind, AbstractEnum, AbstractFunc, AbstractStruct, AbstractType, Item},
        error::Diagnostic,
    },
    symbols::SymbolTable,
};

pub struct Analyzer<'a> {
    ast: &'a Vec<Item>,
    interner: &'a Intern,
    scopes: Vec<HashMap<NameId, SymbolId>>,
    sym_table: SymbolTable,
    err_vec: Vec<Diagnostic>,
}

impl Analyzer<'_> {
    pub fn new<'a>(ast: &'a Vec<Item>, interner: &'a Intern) -> Analyzer<'a> {
        let sym_table = SymbolTable::new();

        Analyzer {
            ast,
            interner,
            scopes: Vec::new(),
            sym_table,
            err_vec: Vec::new(),
        }
    }
}
