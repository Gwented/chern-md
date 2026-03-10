//TODO: GET NAME ID SPANS FOR REGISTER PHASE
mod context;
mod error;
pub mod representation;
mod semantic;

use common::{
    builtins::Keyword,
    intern::Intern,
    symbols::{AstId, Cond, EnumId, PrimitiveId, StructId, TypeDefId, TypedId},
};

use crate::{
    analyzer::{
        context::SemanticReporter,
        representation::{EnumRepre, FieldRepre, StructRepre, Table, TypeDefRepre},
    },
    parser::ast::{AbstractEnum, AbstractStruct, AbstractTypeDef, Expr, Item, Program, TypeExpr},
    token::BuiltinType,
};

//WARN: 232 bytes 232 bytes 232 bytes 232 bytes 232 bytes 232 bytes
pub struct Analyzer<'a> {
    program: &'a Program,
    interner: &'a Intern,
    //WARN: Horrors
    table: Table,
    // Startup idea:
    reporter: SemanticReporter<'a>,
}

impl Analyzer<'_> {
    pub fn new<'a>(program: &'a Program, interner: &'a Intern, src_txt: &'a [u8]) -> Analyzer<'a> {
        Analyzer {
            program,
            interner,
            table: Table::new(),
            reporter: SemanticReporter::new(src_txt),
        }
    }

    //FIXME: USE A SINGULAR VECTOR INDEXED BY NAMEID LATER OVER A HASHMAP NOT NOW PLEASE NOT NOW
    // Ok
    pub fn analyze(&mut self) -> Result<(), ()> {
        // Registering namespaces
        for (id, item) in self.program.items.iter().enumerate() {
            let ast_id = AstId::new(id as u32);

            match item {
                Item::Var(type_def) => self.register_typedef(&type_def, ast_id),
                Item::Struct(structure) => self.register_struct(structure, ast_id),
                Item::Enum(enumeration) => self.register_enum(enumeration, ast_id),
            }
        }

        if !self.reporter.err_vec.is_empty() {
            println!("Error:");
            for err in &self.reporter.err_vec {
                println!("{}", err.msg);
            }

            std::process::exit(1);
        }

        //WARN: I don't know about this
        //Probably better off being functional
        let ids: Vec<TypedId> = self.table.sym_table.values().copied().collect();

        for typed_id in ids {
            match typed_id {
                TypedId::TypeDef(type_def_id) => {
                    _ = self.resolve_typedef(type_def_id);
                    // let thing = &self.table.typedefs[type_def_id.id as usize];
                    //
                    // let name = self.interner.search(thing.name_id.id as usize);
                    //
                    // let typed_id = thing.type_id.unwrap();
                    //
                    // let ty = if let TypedId::Type(id) = typed_id {
                    //     &self.table.types[id.id as usize]
                    // } else {
                    //     panic!("Typedef broke");
                    // };
                    //
                    // println!("Typedef: name = {}\ntype = {ty:?}", name);
                }
                TypedId::Struct(struct_id) => {
                    _ = self.resolve_struct(struct_id);
                    let thing = &self.table.structs[struct_id.id as usize];

                    let name = self.interner.search(thing.name_id.id as usize);

                    println!("Struct: name = {}\n", name);
                    dbg!(&thing.fields);
                    dbg!(&thing.args);
                    dbg!(&thing.conds);

                    let ty = if let TypedId::Struct(id) = typed_id {
                        &self.table.structs[id.id as usize]
                    } else {
                        panic!("Typedef broke");
                    };
                    dbg!(&ty);
                    panic!("Strucken");
                }
                TypedId::Enum(enum_id) => todo!(),
                TypedId::Func(func_id) => todo!(),
                // Maybe call intrinsic type since prim is a lie?
                TypedId::Type(primitive_id) => todo!(),
            }
        }

        if !self.reporter.err_vec.is_empty() {
            println!("Error:");
            for err in &self.reporter.err_vec {
                println!("{}", err.msg);
            }

            eprintln!("Reported {} error(s)\n", self.reporter.err_vec.len());
            std::process::exit(1);
        }

        // Can maybe match in-line if similar to linter recursion issue is not had

        // dbg!(&self.table.sym_table);
        // dbg!(&self.table.typedefs);
        // dbg!(&self.table.structs);
        // dbg!(&self.table.enums);
        // dbg!(&self.table.types);
        Ok(())
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

            // DIRTY
            let type_def = &mut self.table.typedefs[type_def_id.id as usize];
            type_def.type_id = Some(ty);
            type_def.args = args;
            type_def.conds = conds;
        }

        todo!("Main");
        Ok(())
    }

    fn resolve_type_expr(&mut self, ty: &TypeExpr) -> Result<TypedId, ()> {
        match ty {
            TypeExpr::Var(name_id, span) => {
                if let Some(kw) = Keyword::try_as_prim(name_id.id) {
                    //WARN: This can't actually fail here
                    if let Some(actual) = BuiltinType::try_from_kw(kw) {
                        let prim_id = PrimitiveId::new(self.table.types.len() as u32);

                        self.table.types.push(actual);

                        return Ok(TypedId::Type(prim_id));
                    }
                }

                if let Some(typed_id) = self.table.sym_table.get(name_id) {
                    return Ok(typed_id.clone());
                }

                let err_name = self.interner.search(name_id.id as usize);
                let err_msg = format!("Could not find the type \"{err_name}\" defined");

                self.reporter.report_spanned(&err_msg, span);

                return Err(());
            }
            TypeExpr::Generic(generic, span) => {
                if let Some(kw) = Keyword::try_as_kw(generic.base.id) {
                    match kw {
                        Keyword::List => {
                            if generic.args.len() != 1 {
                                return Err(());
                            }

                            self.resolve_type_expr(&generic.args[0])
                        }
                        Keyword::Map => {
                            if generic.args.len() != 2 {
                                return Err(());
                            }

                            let key = self.resolve_type_expr(&generic.args[0])?;
                            let val = self.resolve_type_expr(&generic.args[1])?;

                            let map = BuiltinType::Map(key, val);

                            let prim_id = PrimitiveId::new(self.table.types.len() as u32);

                            self.table.types.push(map);

                            Ok(TypedId::Type(prim_id))
                        }
                        Keyword::Set => {
                            if generic.args.len() != 1 {
                                return Err(());
                            }

                            self.resolve_type_expr(&generic.args[0])
                        }
                        _ => {
                            let err_name = self.interner.search(generic.base.id as usize);
                            //WARN: Questionablly phrased error message
                            //This COULD change so this will not be upheld at the parsing stage
                            let err_msg = format!(
                                "Found identifier \"{err_name}\" before generic parameters, but only `List`, `Set`, and `Map` are valid data structures"
                            );

                            self.reporter.report_spanned(&err_msg, span);

                            return Err(());
                        }
                    }
                } else {
                    // 2004 dog 2004 television
                    let err_name = self.interner.search(generic.base.id as usize);
                    let err_msg = format!(
                        "Found identifier \"{err_name}\" before generic parameters, but only `List`, `Set`, and `Map` are valid data structures"
                    );

                    self.reporter.report_spanned(&err_msg, span);

                    return Err(());
                }
            }
            TypeExpr::Any(span) => {
                let index = self.table.types.len();

                self.table.types.push(BuiltinType::Any(None));

                Ok(TypedId::Type(PrimitiveId::new(index as u32)))
            }
        }
    }

    fn resolve_cond(&mut self, expr: &Expr) -> Cond {
        match expr {
            Expr::Var(name_id, span) => todo!(),
            Expr::Number(num, span) => todo!(),
            Expr::Literal(name_id, span) => todo!(),
            Expr::Call(call, span) => todo!(),
            Expr::Unary(unary, span) => todo!(),
            Expr::FieldAccess(field_access, span) => todo!(),
        }
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

        unimplemented!();
        Ok(())
    }

    fn resolve_expr(&mut self, exprs: &Vec<Expr>) -> TypedId {
        todo!();
    }

    // Maybe put args earlier if possible since not expr
    fn register_typedef(&mut self, type_def: &AbstractTypeDef, ast_id: AstId) {
        let def_id = TypeDefId::new(self.table.typedefs.len() as u32);

        let check = self
            .table
            .sym_table
            // Name span?
            .insert(type_def.name_id, TypedId::TypeDef(def_id));

        //TODO: Maybe a span can be had if abstract structures held name spans only?
        if check.is_some() {
            let duplicate = self.interner.search(type_def.name_id.id as usize);
            let msg = format!("The symbol '{}' appears more than once", duplicate);

            self.reporter.report_basic(&msg);
            return;
        }

        let ty = TypeDefRepre::new(type_def.name_id, ast_id);

        self.table.typedefs.push(ty);
    }

    fn register_struct(&mut self, structure: &AbstractStruct, ast_id: AstId) {
        let struct_id = StructId::new(self.table.structs.len() as u32);

        let check = self
            .table
            .sym_table
            .insert(structure.name_id, TypedId::Struct(struct_id));

        if check.is_some() {
            let duplicate = self.interner.search(structure.name_id.id as usize);
            let msg = format!("The symbol '{}' appears more than once", duplicate);

            self.reporter.report_basic(&msg);
            return;
        }

        let ty = StructRepre::new(structure.name_id, ast_id);

        self.table.structs.push(ty);
    }

    fn register_enum(&mut self, enumeration: &AbstractEnum, ast_id: AstId) {
        let enum_id = EnumId::new(self.table.enums.len() as u32);

        let check = self
            .table
            .sym_table
            .insert(enumeration.name_id, TypedId::Enum(enum_id));

        if check.is_some() {
            let duplicate = self.interner.search(enumeration.name_id.id as usize);
            let msg = format!("The symbol '{}' appears more than once", duplicate);

            self.reporter.report_basic(&msg);
            return;
        }

        let ty = EnumRepre::new(enumeration.name_id, ast_id);

        self.table.enums.push(ty);
    }
}
