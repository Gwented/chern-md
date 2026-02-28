use crate::{
    parser::symbols::{Symbol, SymbolId, SymbolTable, TypeDef, TypeIdent},
    token::ActualType,
};

// Should this just be the primitive keywords type?
// pub(crate) fn search_type_id(sym_table: &SymbolTable, name_id: u32) -> Result<usize, ()> {
//     let symbol = sym_table.search_symbol(name_id);
//     match symbol {
//         Symbol::Template(template) => template.id,
//         Symbol::Definition(type_def) => type_def.type_id,
//         Symbol::Func(function_def) => todo!(),
//         Symbol::Bind(bind) => todo!(),
//     }
// }
// pub(crate) fn search_as_template(
//     sym_table: &SymbolTable,
//     type_id: u32,
// ) -> Result<&Template, &ActualType> {
//     let ty = sym_table.search_type(type_id as usize);
//
//     match ty {
//         ActualType::Template(template) => Ok(template),
//         _ => Err(ty),
//     }
// }

pub(crate) fn search_as_template(
    sym_table: &SymbolTable,
    sym_id: SymbolId,
) -> Result<TypeIdent, &Symbol> {
    dbg!(sym_table.symbols());
    dbg!(sym_id);
    let symbol = sym_table.search_symbol(sym_id.id);

    match symbol {
        Symbol::Definition(type_def) => {
            let ty = sym_table.search_type(type_def.type_id);

            if let ActualType::Template(template) = ty {
                dbg!(template);
                return Ok(template.symbol_id);
            }

            return Err(symbol);
        }
        _ => Err(symbol),
    }
}

pub(crate) fn search_template_id(
    sym_table: &SymbolTable,
    sym_id: SymbolId,
) -> Result<TypeIdent, &Symbol> {
    dbg!(sym_table.symbols());
    dbg!(sym_id);

    let symbol = sym_table.search_symbol(sym_id.id);

    match symbol {
        Symbol::Definition(type_def) => {
            let ty = sym_table.search_type(type_def.type_id);

            if let ActualType::Template(template) = ty {
                return Ok(type_def.type_id);
            }

            return Err(symbol);
        }
        _ => Err(symbol),
    }
}

// fn get_typedef_as_id(sym_table: &SymbolTable) -> TypeIdent {
// }

// May just return id
pub(crate) fn search_as_typedef(
    sym_table: &SymbolTable,
    name_id: u32,
) -> Result<&TypeDef, &Symbol> {
    let symbol = sym_table.search_symbol(name_id);

    match symbol {
        Symbol::Definition(type_def) => Ok(type_def),
        _ => Err(symbol),
    }
}

pub(crate) fn search_as_typedef_mut(
    sym_table: &mut SymbolTable,
    sym_id: SymbolId,
) -> Result<&mut TypeDef, &Symbol> {
    let symbol = sym_table.search_symbol_mut(sym_id.id).unwrap();

    match symbol {
        Symbol::Definition(type_def) => Ok(type_def),
        _ => Err(symbol),
    }
}

pub(crate) fn search_as_func() {}
