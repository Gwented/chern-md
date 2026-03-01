use common::intern::Intern;

use crate::parser::symbols::{Symbol, SymbolTable};

#[macro_export]
macro_rules! print_all {
    ($($x:ident),* (?),) => {
        $()*
    };
}

// Ignore the naming
// No
const _INDENT_: usize = 4;

//TEST: THIS WILL BE MOVED
pub fn print_all(sym_table: &SymbolTable, interner: &Intern) {
    let indent = " ".repeat(_INDENT_);

    for symbol in sym_table.symbols().values() {
        match symbol {
            Symbol::Bind(bind) => {
                let name = interner.search(bind.name_id.id as usize);
                println!("Bind:\n{indent}{name}");
                println!("],");
            }
            Symbol::Def(type_def_id) => {
                let type_def = sym_table.extract_typedef(type_def_id.clone());

                let name = interner.search(type_def.sym_id.id as usize);

                println!("TypeDef: [\n{indent}{name}");

                println!("],");
            }
            Symbol::Func(function_def) => todo!(),
        }
    }
    println!("]");
}

fn print_type(sym_table: &SymbolTable, interner: &Intern) {}
