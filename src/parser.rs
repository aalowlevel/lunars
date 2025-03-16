use pest_derive::Parser;

pub mod ast;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct LunarsParser;
