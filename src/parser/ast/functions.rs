use pest::iterators::Pair;

use crate::{Error, parser::Rule};

use super::variables::Expr;

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expr>,
}

impl<'a> TryFrom<Pair<'a, Rule>> for FunctionCall {
    type Error = Error<'a>;

    fn try_from(pair: Pair<'a, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = pair.into_inner();
        let name = pairs
            .next()
            .ok_or(Error::AstFunctionCallMissingName)?
            .as_str()
            .to_string();

        let args = pairs
            .next()
            .map(|p| p.into_inner().map(|e| e.try_into()).collect())
            .transpose()?
            .unwrap_or_default();

        Ok(Self { name, args })
    }
}
