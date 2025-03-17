pub mod data_structures;
pub mod variables;

use pest::iterators::{Pair, Pairs};
use variables::VariableAssign;

use crate::Error;

use super::Rule;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl TryFrom<Pairs<'_, Rule>> for Program {
    type Error = Error;

    fn try_from(pairs: Pairs<Rule>) -> Result<Self, Self::Error> {
        let mut statements = vec![];

        for pair in pairs {
            match pair.as_rule() {
                Rule::stmt => statements.push(Statement::try_from(pair)?),
                _ => return Err(Error::AstProgramUnexpectedRule),
            }
        }

        Ok(Program { statements })
    }
}

#[derive(Debug)]
pub enum Statement {
    VariableAssign(VariableAssign),
}

impl TryFrom<Pair<'_, Rule>> for Statement {
    type Error = Error;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::assign => Ok(Statement::VariableAssign(VariableAssign::try_from(pair)?)),
            _ => Err(Error::AstStatementUnexpectedRule),
        }
    }
}
