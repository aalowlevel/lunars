pub mod compiler;
pub mod ffi;
pub mod parser;
pub mod vm;

#[derive(Debug)]
pub enum Error {
    AstProgramUnexpectedRule,

    AstStatementUnexpectedRule,
    AstStatementEmpty,

    AstVariableAssignMissingLValue,
    AstVariableAssignMissingExpression,

    AstLValueMissingBase,
    AstLValueMissingMember,

    AstLiteralInvalidNumber,
    AstLiteralUnexpectedRule,
}
