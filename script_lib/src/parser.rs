use crate::token::{ActualType, Cond, InnerArgs, Token};

pub struct Context<'a> {
    tokens: &'a [Token],
    pos: usize,
}

#[derive(Debug)]
pub struct Word {
    // May be integer idk
    id: String,
    ty: ActualType,
    args: Vec<InnerArgs>,
    cond: Vec<Cond>,
}

fn parse() -> Vec<Word> {
    unimplemented!()
}
