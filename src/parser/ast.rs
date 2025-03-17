pub mod data_structures;
pub mod functions;
pub mod variables;

use pest::iterators::{Pair, Pairs};
use variables::VariableAssign;

use crate::Error;

use super::Rule;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl<'a> TryFrom<Pairs<'a, Rule>> for Program {
    type Error = Error<'a>;

    fn try_from(pairs: Pairs<'a, Rule>) -> Result<Self, Self::Error> {
        let mut statements = vec![];

        for pair in pairs {
            match pair.as_rule() {
                Rule::stmt => statements.push(Statement::try_from(pair)?),
                _ => return Err(Error::AstProgramUnexpectedRule(pair)),
            }
        }

        Ok(Program { statements })
    }
}

#[derive(Debug)]
pub enum Statement {
    VariableAssign(VariableAssign),
    Expr(variables::Expr),
}

impl<'a> TryFrom<Pair<'a, Rule>> for Statement {
    type Error = Error<'a>;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        let inner = pair.into_inner().next().ok_or(Error::AstStatementEmpty)?;
        match inner.as_rule() {
            Rule::assign => Ok(Self::VariableAssign(VariableAssign::try_from(inner)?)),
            Rule::expr => Ok(Self::Expr(variables::Expr::try_from(inner)?)),
            _ => Err(Error::AstStatementUnexpectedRule),
        }
    }
}
