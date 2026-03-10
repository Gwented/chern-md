use std::collections::HashMap;

use common::symbols::{AstId, Cond, InnerArgs, NameId, TypedId};

use crate::token::BuiltinType;

// What is a drop? I am new to thinking i have never thought before what is RAII
// is that a gui framework
pub struct Table {
    //FIXME:
    //FIXME:
    //FIXME:
    //FIXME: MUCH RATHER USE IF LET
    //Maybe
    pub(super) sym_table: HashMap<NameId, TypedId>,
    pub(super) typedefs: Vec<TypeDefRepre>,
    pub(super) structs: Vec<StructRepre>,
    pub(super) funcs: Vec<FuncRepre>,
    pub(super) enums: Vec<EnumRepre>,
    pub(super) types: Vec<BuiltinType>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            sym_table: HashMap::new(),
            typedefs: Vec::new(),
            structs: Vec::new(),
            funcs: Vec::new(),
            enums: Vec::new(),
            types: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub(super) struct StructRepre {
    pub(super) name_id: NameId,
    pub(super) ast_id: AstId,
    pub(super) fields: Vec<FieldRepre>,
    pub(super) args: Vec<InnerArgs>,
    pub(super) conds: Vec<Cond>,
}

impl StructRepre {
    pub fn new(name_id: NameId, ast_id: AstId) -> StructRepre {
        StructRepre {
            name_id,
            ast_id,
            fields: Vec::new(),
            args: Vec::new(),
            conds: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub(super) struct EnumRepre {
    pub(super) name_id: NameId,
    pub(super) ast_id: AstId,
    pub(super) variants: Vec<VariantRepre>,
    pub(super) args: Vec<InnerArgs>,
    pub(super) conds: Vec<Cond>,
}

impl EnumRepre {
    pub fn new(name_id: NameId, ast_id: AstId) -> EnumRepre {
        EnumRepre {
            name_id,
            ast_id,
            variants: Vec::new(),
            args: Vec::new(),
            conds: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct VariantRepre {
    pub(super) name_id: NameId,
    //WARN: Not because of being a representation but because enum types are nullable
    pub(super) type_id: Option<TypedId>,
    pub(super) args: Vec<InnerArgs>,
    pub(super) conds: Vec<Cond>,
}

impl VariantRepre {
    pub fn new(name_id: NameId) -> VariantRepre {
        VariantRepre {
            name_id,
            type_id: None,
            args: Vec::new(),
            conds: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub(super) struct TypeDefRepre {
    pub(super) name_id: NameId,
    pub(super) ast_id: AstId,
    pub(super) type_id: Option<TypedId>,
    pub(super) args: Vec<InnerArgs>,
    pub(super) conds: Vec<Cond>,
}

impl TypeDefRepre {
    pub fn new(name_id: NameId, ast_id: AstId) -> TypeDefRepre {
        TypeDefRepre {
            name_id,
            ast_id,
            type_id: None,
            args: Vec::new(),
            conds: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub(super) struct FuncRepre {
    pub(super) name_id: NameId,
    // pub(super) field: Vec<FieldRepre>,
}

#[derive(Debug)]
pub(super) struct FieldRepre {
    pub(super) name_id: NameId,
    pub(super) ty: TypedId,
}

impl FieldRepre {
    pub fn new(name_id: NameId, ty: TypedId) -> FieldRepre {
        FieldRepre { name_id, ty }
    }
}
