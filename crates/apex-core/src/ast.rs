use crate::token::TokenKind;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, ty: Option<String>, value: Expr },
    Function { name: String, params: Vec<Param>, body: Vec<Stmt> },
    Return(Option<Expr>),
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> },
    While { condition: Expr, body: Vec<Stmt> },
    Break,
    Continue,
    Import { path: ImportPath, alias: Option<String> },
    Block(Vec<Stmt>),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ImportPath {
    Module(Vec<String>),
    File(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Unary { op: TokenKind, right: Box<Expr> },
    Binary { left: Box<Expr>, op: TokenKind, right: Box<Expr> },
    Logical { left: Box<Expr>, op: TokenKind, right: Box<Expr> },
    Grouping(Box<Expr>),
    Assign { target: Box<Expr>, op: AssignOp, value: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Index { object: Box<Expr>, index: Box<Expr> },
    Member { object: Box<Expr>, name: String },
    Array(Vec<Expr>),
    Object { name: Option<String>, fields: Vec<(String, Expr)> },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
}

#[derive(Debug, Clone, Copy)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}
