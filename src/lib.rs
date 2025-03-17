use parser::Rule;
use pest::iterators::Pair;

pub mod compiler;
pub mod ffi;
pub mod parser;
pub mod vm;

#[derive(Debug)]
pub enum Error<'a> {
    AstProgramAsLast,
    AstProgramUnexpectedRule(Pair<'a, Rule>),

    AstStatementEmpty,
    AstStatementUnexpectedRule,

    AstVariableAssignMissingLValue,
    AstVariableAssignMissingExpression,

    AstLValueInvalidMember,
    AstLValueMissingBase,

    AstLiteralInvalidNumber,
    AstLiteralUnexpectedRule(Pair<'a, Rule>),

    AstFunctionCallMissingName,
}
