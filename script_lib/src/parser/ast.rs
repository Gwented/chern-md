use common::symbols::{InnerArgs, NameId, Span};

// Going for convention...
// Aliases too.

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
    //                                                 name: str [!IsEmpty, Range(0,5)]
    //TODO: Should these have spans? Do we REALLY want ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //
    Var(AbstractTypeDef),
    Struct(AbstractStruct),
    Enum(AbstractEnum),
    // Func(AbstractFunc),
}

#[derive(Debug)]
pub enum Expr {
    Var(NameId, Span),
    // isize?
    Number(i64, Span),
    Literal(NameId, Span),
    Call(Call, Span),
    FieldAccess(AbstractFieldAccess, Span),
    Unary(Unary, Span),
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
    Var(NameId, Span),
    //_Generic
    Generic(Generic, Span),
    Any(Span),
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

// #[derive(Debug)]
// pub struct AbstractBind {
//     pub(crate) name_id: NameId,
// }
//
// impl AbstractBind {
//     pub fn new(name_id: NameId) -> AbstractBind {
//         AbstractBind { name_id }
//     }
// }
//
#[derive(Debug)]
// ONE HUNDRED TWENTY BYTES WHAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAT
// WHAT
// FIX:FIX:FIX:FIX:FIX:
//  This is fine, none of this is bad. None of it.
pub struct AbstractTypeDef {
    pub(crate) name_id: NameId,
    pub(crate) name_span: Span,
    pub(crate) ty: TypeExpr,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) conds: Vec<Expr>,
}

impl AbstractTypeDef {
    pub fn new(
        name_id: NameId,
        name_span: Span,
        ty: TypeExpr,
        args: Vec<InnerArgs>,
        conds: Vec<Expr>,
    ) -> AbstractTypeDef {
        AbstractTypeDef {
            name_id,
            name_span,
            ty,
            args,
            conds,
        }
    }
}

#[derive(Debug)]
pub struct AbstractStruct {
    pub(crate) name_id: NameId,
    pub(crate) name_span: Span,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) conds: Vec<Expr>,
    pub(crate) fields: Vec<AbstractTypeDef>,
}

impl AbstractStruct {
    pub fn new(
        name_id: NameId,
        name_span: Span,
        args: Vec<InnerArgs>,
        conds: Vec<Expr>,
        //TODO: Change both enum and struct of field
        fields: Vec<AbstractTypeDef>,
    ) -> AbstractStruct {
        AbstractStruct {
            name_id,
            name_span,
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
    pub(crate) name_span: Span,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) conds: Vec<Expr>,
    pub(crate) variants: Vec<AbstractVariant>,
}

impl AbstractEnum {
    pub fn new(
        name_id: NameId,
        name_span: Span,
        args: Vec<InnerArgs>,
        // I'm scared
        conds: Vec<Expr>,
        variants: Vec<AbstractVariant>,
    ) -> AbstractEnum {
        AbstractEnum {
            name_id,
            name_span,
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
    pub(super) name_span: Span,
    // I think this is right?
    pub(crate) ty: Option<TypeExpr>,
    pub(crate) args: Vec<InnerArgs>,
    pub(crate) conds: Vec<Expr>,
}

impl AbstractVariant {
    pub fn new(
        name_id: NameId,
        name_span: Span,
        // I think this is right?
        ty: Option<TypeExpr>,
        args: Vec<InnerArgs>,
        conds: Vec<Expr>,
    ) -> AbstractVariant {
        AbstractVariant {
            name_id,
            name_span,
            ty,
            args,
            conds,
        }
    }
}

#[derive(Debug)]
pub struct AbstractFunc {
    pub(crate) name_id: NameId,
    pub(super) name_span: Span,
    pub(crate) params: Vec<Expr>,
}

impl AbstractFunc {
    pub fn new(name_id: NameId, name_span: Span, params: Vec<Expr>) -> AbstractFunc {
        AbstractFunc {
            name_id,
            name_span,
            params,
        }
    }
}

// Please no SpannedNameId
#[derive(Debug)]
pub struct AbstractFieldAccess {
    pub(crate) base: Box<Expr>,
    pub(crate) field: NameId,
}

impl AbstractFieldAccess {
    pub fn new(base: Box<Expr>, field: NameId) -> AbstractFieldAccess {
        AbstractFieldAccess { base, field }
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
    // Negate
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
