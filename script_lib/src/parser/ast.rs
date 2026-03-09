use common::symbols::{InnerArgs, NameId};

// Going for convention...
// Aliases too.

//TEST: Storing bind or constants in the future could use this maybe?
#[derive(Debug)]
pub struct Program {
    // MAYBE SHOULDN't BE A NAME ID I DONT KNOW
    //FIX: Inconsistent
    pub bind: Option<NameId>,
    pub items: Vec<Item>,
}

impl Program {
    pub(crate) fn new() -> Program {
        Program {
            bind: None,
            items: Vec::new(),
        }
    }

    //TEST: Oh my java.
    pub(crate) fn set_bind(&mut self, bind: NameId) {
        self.bind = Some(bind);
    }

    pub(crate) fn has_bind(&self) -> bool {
        self.bind.is_some()
    }
}

#[derive(Debug)]
pub enum Item {
    //FIX: WILL REMOVE
    // DO I SECTION THIS?
    Var(AbstractTypeDef),
    Struct(AbstractStruct),
    Enum(AbstractEnum),
    // Func(AbstractFunc),
}

#[derive(Debug)]
pub enum Expr {
    Var(NameId),
    // isize?
    Number(i64),
    Literal(NameId),
    Call(Call),
    FieldAccess(FieldAccess),
    Unary(Unary),
}

#[derive(Debug)]
pub struct Call {
    pub(crate) callee: Box<Expr>,
    // Vec?
    pub(crate) exprs: Vec<Expr>,
}

impl Call {
    pub fn new(callee: Box<Expr>, exprs: Vec<Expr>) -> Call {
        Call { callee, exprs }
    }
}

// WHAT IS A TUPLE I HAVE NOT HEARD OF THAT BEFORE I AM NEW TO THINKING HAS ANYONE THOUGHT BEFORE?
#[derive(Debug)]
pub enum TypeExpr {
    Var(NameId),
    //_Generic
    Generic(Generic),
    Any,
}

// Maybe put in enum exclusively if not needed outside

#[derive(Debug)]
pub struct AbstractGeneric {
    pub(crate) name_id: NameId,
    pub(crate) args: Box<TypeExpr>,
}

impl AbstractGeneric {
    pub fn new(name_id: NameId, args: Box<TypeExpr>) -> AbstractGeneric {
        AbstractGeneric { name_id, args }
    }
}

// public abstract class AbstractBind {}
#[derive(Debug)]
pub struct AbstractBind {
    pub(crate) name_id: NameId,
}

impl AbstractBind {
    pub fn new(name_id: NameId) -> AbstractBind {
        AbstractBind { name_id }
    }
}

#[derive(Debug)]
pub struct AbstractTypeDef {
    pub(crate) name_id: NameId,
    pub(crate) ty: TypeExpr,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) conds: Vec<Expr>,
}

impl AbstractTypeDef {
    pub fn new(
        name_id: NameId,
        ty: TypeExpr,
        args: Vec<InnerArgs>,
        conds: Vec<Expr>,
    ) -> AbstractTypeDef {
        AbstractTypeDef {
            name_id,
            ty,
            args,
            conds,
        }
    }
}

#[derive(Debug)]
pub struct AbstractStruct {
    pub(crate) name_id: NameId,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) conds: Vec<Expr>,
    pub(crate) fields: Vec<AbstractTypeDef>,
}

impl AbstractStruct {
    pub fn new(
        name_id: NameId,
        args: Vec<InnerArgs>,
        conds: Vec<Expr>,
        //TODO: Change both enum and struct of field
        fields: Vec<AbstractTypeDef>,
    ) -> AbstractStruct {
        AbstractStruct {
            name_id,
            args,
            conds,
            fields,
        }
    }
}

#[derive(Debug)]
pub struct AbstractEnum {
    // Would be SymbolId in symbol table anyways
    pub(crate) name_id: NameId,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) conds: Vec<Expr>,
    pub(crate) variants: Vec<AbstractVariant>,
}

impl AbstractEnum {
    pub fn new(
        name_id: NameId,
        args: Vec<InnerArgs>,
        // I'm scared
        conds: Vec<Expr>,
        variants: Vec<AbstractVariant>,
    ) -> AbstractEnum {
        AbstractEnum {
            name_id,
            args,
            conds,
            variants,
        }
    }
}

// Hold that thought
#[derive(Debug)]
pub struct AbstractVariant {
    pub(crate) name_id: NameId,
    // I think this is right?
    pub(crate) ty: Option<TypeExpr>,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) conds: Vec<Expr>,
}

impl AbstractVariant {
    pub fn new(
        name_id: NameId,
        // I think this is right?
        ty: Option<TypeExpr>,
        args: Vec<InnerArgs>,
        conds: Vec<Expr>,
    ) -> AbstractVariant {
        AbstractVariant {
            name_id,
            ty,
            args,
            conds,
        }
    }
}

#[derive(Debug)]
pub struct AbstractFunc {
    pub(crate) name_id: NameId,
    pub(crate) params: Vec<Expr>,
}

impl AbstractFunc {
    pub fn new(name_id: NameId, params: Vec<Expr>) -> AbstractFunc {
        AbstractFunc { name_id, params }
    }
}

#[derive(Debug)]
pub struct FieldAccess {
    pub(crate) base: Box<Expr>,
    pub(crate) field: NameId,
}

impl FieldAccess {
    pub fn new(base: Box<Expr>, field: NameId) -> FieldAccess {
        FieldAccess { base, field }
    }
}

#[derive(Debug)]
pub struct Unary {
    pub(crate) op: UnaryOp,
    pub(crate) expr: Box<Expr>,
}

impl Unary {
    pub fn new(op: UnaryOp, expr: Box<Expr>) -> Unary {
        Unary { op, expr }
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Not,
}

#[derive(Debug)]
pub struct Generic {
    pub(crate) base: NameId,
    // Change to tuple or something alike since max 2?
    pub(crate) args: Vec<TypeExpr>,
}

impl Generic {
    pub fn new(base: NameId, args: Vec<TypeExpr>) -> Generic {
        Generic { base, args }
    }
}
