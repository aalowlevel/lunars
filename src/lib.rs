use parser::Rule;
use pest::iterators::Pairs;

pub mod compiler;
pub mod ffi;
pub mod parser;
pub mod vm;

#[derive(Debug)]
pub enum Error<'a> {
    AstProgramUnexpectedRule(Pairs<'a, Rule>),

    AstStatementUnexpectedRule,
    AstStatementEmpty,

    AstVariableAssignMissingLValue,
    AstVariableAssignMissingExpression,

    AstLValueMissingBase,
    AstLValueMissingMember,

    AstLiteralInvalidNumber,
    AstLiteralUnexpectedRule,

    AstFunctionCallMissingName,
}
