// --- ast/data_structures.rs
use crate::{Error, parser::Rule};
use pest::iterators::Pair;

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl<'a> TryFrom<Pair<'a, Rule>> for Literal {
    type Error = Error<'a>;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::number => {
                let num_str = pair.as_str();
                num_str
                    .parse()
                    .map(Literal::Number)
                    .map_err(|_| Error::AstLiteralInvalidNumber)
            }
            Rule::string => {
                let content = pair.as_str();
                Ok(Literal::String(content[1..content.len() - 1].to_string()))
            }
            Rule::boolean => match pair.as_str() {
                "true" => Ok(Literal::Boolean(true)),
                "false" => Ok(Literal::Boolean(false)),
                _ => Err(Error::AstLiteralInvalidNumber),
            },
            Rule::null => Ok(Literal::Null),
            _ => Err(Error::AstLiteralUnexpectedRule),
        }
    }
}
