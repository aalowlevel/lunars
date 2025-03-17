// --- ast/variables.rs
use super::{data_structures::Literal, functions::FunctionCall};
use crate::{Error, parser::Rule};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct VariableAssign {
    pub target: LValue,
    pub value: Expr,
}

impl<'a> TryFrom<Pair<'a, Rule>> for VariableAssign {
    type Error = Error<'a>;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
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

impl<'a> TryFrom<Pair<'a, Rule>> for LValue {
    type Error = Error<'a>;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        if pair.as_rule() == Rule::ident || pair.as_str() == "self" {
            return Ok(Self::Identifier(pair.as_str().to_string()));
        }

        let mut pairs = pair.into_inner();
        let mut current = LValue::try_from(pairs.next().ok_or(Error::AstLValueMissingBase)?)?;

        for member_pair in pairs {
            let member_str = member_pair.as_str();
            let member = member_str
                .strip_prefix('.')
                .ok_or(Error::AstLValueInvalidMember)?
                .to_string();
            current = LValue::MemberAccess {
                base: Box::new(current),
                member,
            };
        }

        Ok(current)
    }
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
    Literal(Literal),
    FunctionCall(FunctionCall),
    Unimplemented(String),
}

impl<'a> TryFrom<Pair<'a, Rule>> for Expr {
    type Error = Error<'a>;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::ident => Ok(Self::Ident(pair.as_str().to_string())),
            Rule::number | Rule::string | Rule::boolean | Rule::null => {
                Ok(Self::Literal(pair.try_into()?))
            }
            Rule::func_call => Ok(Self::FunctionCall(pair.try_into()?)),
            _ => Ok(Self::Unimplemented(format!("{:?}", pair.as_rule()))),
        }
    }
}
