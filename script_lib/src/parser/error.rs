use std::fmt::Display;

// Has a lifetime because of previous clone concerns in instantiation
#[derive(Debug)]
// Turn to, Type, ie. expected. Have general, expected, found, branch, pre_tok parts.
// I'm new to thinking. Anyone have some beginner thoughts?
pub struct Diagnostic {
    // Maybe warns will exist at some point
    pub(crate) msg: String,
    pub(crate) branch: Branch,
    // Maybe help
    // pub(crate) help: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Branch {
    Searching,
    Bind,
    Var,
    // Test variants
    VarCond,
    VarTypeArgs,
    // Test variants
    Nest,
    ComplexRules,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Branch::Searching => write!(f, "searching..."),
            Branch::Bind => write!(f, "bind"),
            Branch::Var => write!(f, "var"),
            Branch::VarCond => write!(f, "var [conditions]"),
            Branch::VarTypeArgs => write!(f, "var [args]"),
            Branch::Nest => write!(f, "nest"),
            Branch::ComplexRules => write!(f, "complex_rules"),
        }
    }
}

impl Diagnostic {
    pub(crate) fn new(msg: String, branch: Branch) -> Diagnostic {
        Diagnostic { msg, branch }
    }
}
