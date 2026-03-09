// Suspicious hash
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId {
    pub id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstId {
    pub id: u32,
}

impl AstId {
    pub fn new(id: u32) -> AstId {
        AstId { id }
    }
}

impl SymbolId {
    pub fn new(id: u32) -> SymbolId {
        SymbolId { id }
    }
}

// Because of TypeId in Crust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeIdent {
    pub id: u32,
}

impl TypeIdent {
    pub fn new(id: u32) -> TypeIdent {
        TypeIdent { id }
    }
}

impl From<u32> for TypeIdent {
    fn from(v: u32) -> Self {
        TypeIdent::new(v)
    }
}

// FIXME: Does not seem needed since there are no functions only definitions.
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct ScopeId {
//     pub id: u32,
// }
// impl ScopeId {
//     pub fn new(id: u32) -> ScopeId {
//         ScopeId { id }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NameId {
    pub id: u32,
}

impl NameId {
    pub fn new(id: u32) -> NameId {
        NameId { id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuncId {
    pub id: u32,
}

impl FuncId {
    pub fn new(id: u32) -> FuncId {
        FuncId { id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnumId {
    pub id: u32,
}

impl EnumId {
    pub fn new(id: u32) -> EnumId {
        EnumId { id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructId {
    pub id: u32,
}

impl StructId {
    pub fn new(id: u32) -> StructId {
        StructId { id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimitiveId {
    pub id: u32,
}

impl PrimitiveId {
    pub fn new(id: u32) -> PrimitiveId {
        PrimitiveId { id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeDefId {
    pub id: u32,
}

impl TypeDefId {
    pub fn new(id: u32) -> TypeDefId {
        TypeDefId { id }
    }
}

//FIX: Should maybe be somewhere else but fine for now
#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }
}

#[derive(Debug)]
pub enum Cond {
    //FIX:
    Func(FuncId),
    // Maybe this shouldn't be a function
    IsEmpty,
    IsWhitespace,
    // Probably should just attach bool
    // should likely be removed
    Not(Box<Cond>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InnerArgs {
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
            "scient" => Ok(InnerArgs::Scientific),
            "hex" => Ok(InnerArgs::Hex),
            "bin" => Ok(InnerArgs::Binary),
            "octo" => Ok(InnerArgs::Octo),
            v => Err(v),
        }
    }
}
