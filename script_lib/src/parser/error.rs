use std::fmt::Display;

use crate::token::SpannedToken;

// Has a lifetime because of previous clone concerns in instantiation
#[derive(Debug)]
pub struct Diagnostic {
    // Maybe warns will exist at some point
    pub(super) msg: String,
    pub(super) branch: Branch,
    pub(super) prev_tok: SpannedToken,
    // Maybe help
    // pub(super) help: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branch {
    Searching,
    Bind,
    Var,
    Nest,
    ComplexRules,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Branch::Searching => write!(f, ""),
            Branch::Bind => write!(f, "bind"),
            Branch::Var => write!(f, "var"),
            Branch::Nest => write!(f, "nest"),
            Branch::ComplexRules => write!(f, "complex_rules"),
        }
    }
}

impl Diagnostic {
    pub fn new(msg: String, branch: Branch, prev_tok: &SpannedToken) -> Diagnostic {
        Diagnostic {
            msg,
            branch,
            prev_tok: prev_tok.clone(),
        }
    }
}
