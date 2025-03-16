#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Assign(Assign),
    If(IfStmt),
    While(WhileLoop),
    FuncDef(FuncDef),
    FuncCall(FuncCall),
}

#[derive(Debug, PartialEq)]
pub struct Assign {
    pub ident: String,
    pub value: Expr,
}

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Vec<Stmt>,
    pub else_branch: Option<ElseBranch>,
}

#[derive(Debug, PartialEq)]
pub enum ElseBranch {
    Block(Vec<Stmt>),
    If(Box<IfStmt>),
}

#[derive(Debug, PartialEq)]
pub struct WhileLoop {
    pub condition: Expr,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct FuncDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Ident(String),
    List(Vec<Expr>),
    Dict(Vec<(DictKey, Expr)>),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    FuncCall(FuncCall),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DictKey {
    String(String),
    Ident(String),
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, PartialEq)]
pub struct FuncCall {
    pub callee: String,
    pub args: Vec<Expr>,
}
