mod context;
pub mod representation;
mod semantic;

use common::{
    intern::Intern,
    primitives::{self, Keyword},
    symbols::{AstId, Cond, EnumId, PrimitiveId, StructId, TypeDefId},
};

use crate::{
    analyzer::representation::{EnumRepre, FieldRepre, StructRepre, Table, TypeDefRepre, TypedId},
    parser::{
        ast::{AbstractEnum, AbstractStruct, AbstractTypeDef, Expr, Item, Program, TypeExpr},
        error::Diagnostic,
    },
    token::ActualType,
};

//WARN: 232 bytes 232 bytes 232 bytes 232 bytes 232 bytes 232 bytes
pub struct Analyzer<'a> {
    pub(super) program: &'a Program,
    pub(super) interner: &'a Intern,
    //WARN: Horrors
    pub(super) table: Table,
    pub(super) err_vec: Vec<Diagnostic>,
}

impl Analyzer<'_> {
    pub fn new<'a>(program: &'a Program, interner: &'a Intern) -> Analyzer<'a> {
        Analyzer {
            program,
            interner,
            table: Table::new(),
            err_vec: Vec::new(),
        }
    }

    //FIXME: USE A SINGULAR VECTOR INDEXED BY NAMEID LATER OVER A HASHMAP NOT NOW PLEASE NOT NOW
    // Ok
    pub fn analyze(&mut self) {
        // Registering namespaces
        for (id, item) in self.program.items.iter().enumerate() {
            let ast_id = AstId::new(id as u32);

            match item {
                Item::Var(type_def) => self.register_typedef(&type_def, ast_id),
                Item::Struct(structure) => self.register_struct(structure, ast_id),
                Item::Enum(enumeration) => self.register_enum(enumeration, ast_id),
            }
        }

        //WARN: I don't know about this
        //Probably better off being functional
        let ids: Vec<TypedId> = self.table.sym_table.values().copied().collect();

        for typed_id in ids {
            match typed_id {
                TypedId::TypeDef(type_def_id) => {
                    _ = self.resolve_typedef(type_def_id);
                    let thing = &self.table.typedefs[type_def_id.id as usize];

                    let name = self.interner.search(thing.name_id.id as usize);

                    let typed_id = thing.type_id.unwrap();

                    let ty = if let TypedId::Type(id) = typed_id {
                        &self.table.types[id.id as usize]
                    } else {
                        panic!("Typedef broke");
                    };

                    println!("Typedef: name = {}\ntype = {ty:?}", name);
                }
                TypedId::Struct(struct_id) => todo!(),
                TypedId::Enum(enum_id) => todo!(),
                TypedId::Func(func_id) => todo!(),
                // Maybe call intrinsic type since prim is a lie?
                TypedId::Type(primitive_id) => todo!(),
            }
        }

        // Can maybe match in-line if similar to linter recursion issue is not had

        // dbg!(&self.table.sym_table);
        // dbg!(&self.table.typedefs);
        // dbg!(&self.table.structs);
        // dbg!(&self.table.enums);
        // dbg!(&self.table.types);
    }

    fn resolve_typedef(&mut self, type_def_id: TypeDefId) -> Result<(), ()> {
        let ast_def = {
            let type_def = &mut self.table.typedefs[type_def_id.id as usize];
            &self.program.items[type_def.ast_id.id as usize]
        };

        // DIRTY
        if let Item::Var(abstract_typedef) = ast_def {
            let ty = self.resolve_type_expr(&abstract_typedef.ty)?;
            let args = abstract_typedef.args.clone();

            let mut conds = Vec::new();

            for expr in &abstract_typedef.conds {
                conds.push(self.resolve_cond(expr));
            }

            let type_def = &mut self.table.typedefs[type_def_id.id as usize];
            type_def.type_id = Some(ty);
            type_def.args = args;
            type_def.conds = conds;
        }

        Ok(())
    }

    fn resolve_type_expr(&mut self, ty: &TypeExpr) -> Result<TypedId, ()> {
        match ty {
            TypeExpr::Var(name_id) => {
                if let Some(kw) = Keyword::try_as_prim(name_id.id) {
                    //WARN: This can't actually fail here
                    if let Some(actual) = ActualType::from_keyword(kw) {
                        let prim_id = PrimitiveId::new(self.table.types.len() as u32);

                        self.table.types.push(actual);

                        return Ok(TypedId::Type(prim_id));
                    }
                }
                todo!("Var");
            }
            TypeExpr::Generic(generic) => match Keyword::try_as_kw(generic.base.id) {
                Some(_) => todo!("Generic"),
                None => todo!(),
            },
            TypeExpr::Any => {
                let index = self.table.types.len();

                self.table.types.push(ActualType::Any(None));

                Ok(TypedId::Type(PrimitiveId::new(index as u32)))
            }
        }
    }

    fn resolve_cond(&mut self, expr: &Expr) -> Cond {
        todo!();
    }

    fn resolve_struct(&mut self, struct_id: StructId) -> Result<(), ()> {
        let ast_struct = {
            let structure = &mut self.table.structs[struct_id.id as usize];
            &self.program.items[structure.ast_id.id as usize]
        };

        // DIRTY
        if let Item::Struct(abstract_struct) = ast_struct {
            for type_def in &abstract_struct.fields {
                let typed_id = self.resolve_type_expr(&type_def.ty).unwrap();

                let field_repre = FieldRepre::new(type_def.name_id, typed_id);

                let structure = &mut self.table.structs[struct_id.id as usize];

                structure.fields.push(field_repre);
            }
        }

        todo!();
    }

    fn resolve_conds(&mut self, exprs: &Vec<Expr>) -> Vec<Cond> {
        let conds = Vec::new();

        for expr in exprs {
            match expr {
                Expr::Var(name_id) => todo!(),
                Expr::Number(_) => todo!(),
                Expr::Literal(name_id) => todo!(),
                Expr::Call(call) => todo!(),
                Expr::Unary(unary) => todo!(),
                Expr::FieldAccess(field_access) => todo!(),
            }
        }

        panic!("HIiii");
        conds
    }

    fn resolve_expr(&mut self, exprs: &Vec<Expr>) -> TypedId {
        todo!();
    }

    // Maybe put args earlier if possible since not expr
    fn register_typedef(&mut self, type_def: &AbstractTypeDef, ast_id: AstId) {
        let def_id = TypeDefId::new(self.table.typedefs.len() as u32);

        self.table
            .sym_table
            .insert(type_def.name_id, TypedId::TypeDef(def_id));

        // self.register_types(&type_def.ty);

        let ty = TypeDefRepre::new(type_def.name_id, ast_id);

        self.table.typedefs.push(ty);
    }

    fn register_struct(&mut self, structure: &AbstractStruct, ast_id: AstId) {
        let struct_id = StructId::new(self.table.structs.len() as u32);

        self.table
            .sym_table
            .insert(structure.name_id, TypedId::Struct(struct_id));

        let ty = StructRepre::new(structure.name_id, ast_id);

        self.table.structs.push(ty);
    }

    fn register_enum(&mut self, enumeration: &AbstractEnum, ast_id: AstId) {
        let enum_id = EnumId::new(self.table.enums.len() as u32);

        self.table
            .sym_table
            .insert(enumeration.name_id, TypedId::Enum(enum_id));

        let ty = EnumRepre::new(enumeration.name_id, ast_id);

        self.table.enums.push(ty);
    }
}
