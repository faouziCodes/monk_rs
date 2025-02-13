use std::path::PathBuf;

pub struct Ast {
    pub name: Option<String>,
    pub path: Option<PathBuf>,
    pub prog: Vec<Stmt>,
}

pub enum Stmt {
    Let(String, Option<Type>, Expr),
    Func(String, Params, Option<Type>, Expr),
    For(Expr, Box<Stmt>),
    While(Expr, Box<Stmt>),
    Match(Expr, Cases),
    Expr(Expr),
}

pub enum Expr {
    Value(Value),
    Call(String, Vec<Expr>),
    Binary(Box<Expr>, Operation, Box<Expr>),
    Block(Vec<Stmt>),
}

pub enum Type {
    String,
    Int,
    Float,
}

pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Ident(String),
}

pub enum Operation {
    Add,
    Mul,
    Sub,
    Div,
    EqEq,
    Less,
    LessEq,
    More,
    MoreEq,
}

pub type Param = (String, Option<Type>);
pub type Params = Vec<Param>;

pub type Case = (Expr, Expr);
pub type Cases = Vec<Case>;
