use std::fmt::Display;

#[derive(Debug)]
// I'm new to thinking. Anyone have some beginner thoughts?
pub(super) struct Diagnostic {
    //FIX:
    pub(super) msg: String,
    pub(super) branch: Branch,
    // Maybe help
    // pub(crate) help: Option<String>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) enum Branch {
    Broken,
    Searching,
    Bind,
    Var,
    VarType,
    VarCond,
    VarFuncArgs,
    VarTypeArgs,
    Nest,
    NestType,
    NestEnum,
    ComplexRules,
    Override,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Branch::Broken => write!(f, "abort"),
            Branch::Searching => write!(f, "searching..."),
            Branch::Bind => write!(f, "bind"),
            Branch::Var => write!(f, "var"),
            Branch::VarType => write!(f, "[type]"),
            Branch::VarCond => write!(f, "[conditions]"),
            Branch::VarFuncArgs => write!(f, "[args]"),
            Branch::VarTypeArgs => write!(f, "[args]"),
            Branch::Nest => write!(f, "nest"),
            Branch::NestType => write!(f, "[type]"),
            Branch::NestEnum => write!(f, "[enum]"),
            Branch::ComplexRules => write!(f, "complex_rules"),
            Branch::Override => write!(f, "override"),
        }
    }
}

impl Diagnostic {
    pub(super) fn new(msg: String, branch: Branch) -> Diagnostic {
        Diagnostic { msg, branch }
    }
}
