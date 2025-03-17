// --- ast/variables.rs
use super::data_structures::Literal;
use crate::{Error, parser::Rule};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct VariableAssign {
    pub target: LValue,
    pub value: Expr,
}

impl TryFrom<Pair<'_, Rule>> for VariableAssign {
    type Error = Error;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut pairs = pair.into_inner();
        let target = pairs
            .next()
            .ok_or(Error::AstVariableAssignMissingLValue)?
            .try_into()?;
        let value = pairs
            .next()
            .ok_or(Error::AstVariableAssignMissingExpression)?
            .try_into()?;

        Ok(Self { target, value })
    }
}

#[derive(Debug)]
pub enum LValue {
    Identifier(String),
    MemberAccess { base: Box<LValue>, member: String },
}

impl TryFrom<Pair<'_, Rule>> for LValue {
    type Error = Error;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        if pair.as_rule() == Rule::ident {
            return Ok(Self::Identifier(pair.as_str().to_string()));
        }

        let mut pairs = pair.into_inner();
        let base = Box::new(
            pairs
                .next()
                .ok_or(Error::AstLValueMissingBase)?
                .try_into()?,
        );
        let member = pairs
            .next()
            .ok_or(Error::AstLValueMissingMember)?
            .as_str()
            .to_string();

        Ok(Self::MemberAccess { base, member })
    }
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
    Literal(Literal),
    Unimplemented(String),
}

impl TryFrom<Pair<'_, Rule>> for Expr {
    type Error = Error;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::ident => Ok(Self::Ident(pair.as_str().to_string())),
            Rule::number | Rule::string | Rule::boolean | Rule::null => {
                Ok(Self::Literal(pair.try_into()?))
            }
            _ => Ok(Self::Unimplemented(format!("{:?}", pair.as_rule()))),
        }
    }
}
