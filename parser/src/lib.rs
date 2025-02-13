use ast::Value;
use std::collections::LinkedList;
use token::{Token, TokenKind};

pub mod lexer;
pub mod token;

pub struct Parser {
    position: usize,
    tokens: Vec<Token>,
}

pub enum ParseError {
    Expected(String, TokenKind),
    UnExpected(String, Token),
    UnExpectedEof,
}

type ParseResult = Result<ast::Stmt, ParseError>;
type ParseResultGen<T> = Result<T, ParseError>;
type ParseFn = fn(parser: &mut Parser) -> ParseResult;
type ParseFnGen<T> = fn(parser: &mut Parser) -> ParseResultGen<T>;

fn param(parser: &mut Parser) -> ParseResultGen<ast::Param> {
    let Some(Token {
        kind: TokenKind::Ident,
        value: name,
    }) = parser.next_token()
    else {
        return Err(ParseError::Expected("Param".into(), TokenKind::Ident));
    };
    let type_anot = type_anot(parser).map_or(None, |x| Some(x));
    let Some(Token {
        kind: TokenKind::Comma,
        ..
    }) = parser.next_token()
    else {
        return Err(ParseError::Expected("Param".into(), TokenKind::Comma));
    };
    return Ok((name, type_anot));
}

fn operation(parser: &mut Parser) -> ParseResultGen<ast::Operation> {
    match parser.next_token() {
        Some(token) => {
            if let TokenKind::Op(op_kind) = token.kind {
                Ok(match op_kind {
                    token::Operator::Add => ast::Operation::Add,
                    token::Operator::Sub => ast::Operation::Sub,
                    token::Operator::Mul => ast::Operation::Mul,
                    token::Operator::Div => ast::Operation::Div,
                    token::Operator::EqEq => ast::Operation::EqEq,
                    token::Operator::More => ast::Operation::More,
                    token::Operator::MoreEq => ast::Operation::MoreEq,
                    token::Operator::Less => ast::Operation::Less,
                    token::Operator::LessEq => ast::Operation::LessEq,
                })
            } else {
                Err(ParseError::UnExpected("Operation".into(), token))
            }
        }
        None => Err(ParseError::UnExpectedEof),
    }
}

fn value(parser: &mut Parser) -> ParseResultGen<Value> {
    match parser.next_token() {
        Some(token) => match token.kind {
            TokenKind::Int => Ok(Value::Int(token.value.parse::<i64>().unwrap())),
            TokenKind::Float => Ok(Value::Float(token.value.parse::<f64>().unwrap())),
            TokenKind::Ident => Ok(Value::Ident(token.value)),
            TokenKind::String => Ok(Value::String(token.value)),
            _ => Err(ParseError::UnExpected("Value".into(), token)),
        },
        None => Err(ParseError::UnExpectedEof),
    }
}

fn binary(parser: &mut Parser, start: ast::Expr) -> ParseResultGen<ast::Expr> {
    // TODO: If there is an error in the binary expr this function will fail, not by error, but by
    // shutting down execution, we must return errors instead (speaks for itself, but I mustn't
    // forget.
    let mut operations = LinkedList::new();
    let mut values = LinkedList::from([start]);

    loop {
        let Ok(op) = parser.try_parse_gen(operation) else {
            break;
        };
        let rhs = ast::Expr::Value(value(parser)?);

        match op {
            ast::Operation::Mul | ast::Operation::Div => {
                let lhs = values.pop_back().unwrap();
                values.push_back(ast::Expr::Binary(lhs.into(), op, rhs.into()));
            }
            op => {
                values.push_back(rhs);
                operations.push_back(op)
            }
        }
    }

    while let Some(op) = operations.pop_back() {
        let rhs = values.pop_back().unwrap();
        let lhs = values.pop_back().unwrap();
        values.push_back(ast::Expr::Binary(lhs.into(), op, rhs.into()));
    }

    assert!(values.len() == 1);
    Ok(values.pop_front().unwrap())
}

fn type_anot(parser: &mut Parser) -> ParseResultGen<ast::Type> {
    todo!()
}

fn function_call(parser: &mut Parser, value: Value) -> ParseResultGen<ast::Expr> {
    match parser.peek() {
        Some(t) => match t.kind {
            TokenKind::LeftBrace => {
                parser.next_token().unwrap();
                todo!()
            }
            _ => unreachable!("This should never happen, function call should only be called if the next token is a leftbrace.")
        },
        _ => unreachable!("This should never happen, function call should only be called if the next token is a leftbrace.")
    }
}

fn expr(parser: &mut Parser) -> ParseResultGen<ast::Expr> {
    match parser.peek() {
        Some(token) => match token.kind {
            TokenKind::Int | TokenKind::Float | TokenKind::String | TokenKind::Ident => {
                let value = value(parser)?;

                match parser.peek() {
                    Some(t) => match t.kind {
                        TokenKind::Op(_) => binary(parser, ast::Expr::Value(value)),
                        TokenKind::LeftBrace => function_call(parser, value),
                        _ => Ok(ast::Expr::Value(value)),
                    },
                    None => Ok(ast::Expr::Value(value)),
                }
            }
            _kind => Err(ParseError::UnExpected("Expression".into(), token)),
        },
        None => Err(ParseError::UnExpectedEof),
    }
}

fn params(parser: &mut Parser) -> ParseResultGen<ast::Params> {
    let Some(Token {
        kind: TokenKind::LeftBrace,
        ..
    }) = parser.next_token()
    else {
        return Err(ParseError::Expected("Variable".into(), TokenKind::Ident));
    };

    let mut params = Vec::new();
    loop {
        match parser.peek() {
            Some(Token {
                kind: TokenKind::Comma,
                ..
            }) => {
                parser.next_token();
                params.push(param(parser)?);
            }
            Some(Token {
                kind: TokenKind::RightBrace,
                ..
            }) => break,
            Some(token) => return Err(ParseError::UnExpected("Params".into(), token)),
            None => return Err(ParseError::UnExpectedEof),
        }
    }

    let Some(Token {
        kind: TokenKind::RightBrace,
        ..
    }) = parser.next_token()
    else {
        return Err(ParseError::Expected("Variable".into(), TokenKind::Ident));
    };

    Ok(params)
}

fn var(parser: &mut Parser) -> ParseResult {
    let Some(Token {
        kind: TokenKind::Let,
        ..
    }) = parser.next_token()
    else {
        panic!("Var was called even tho their is no Let.")
    };
    let Some(Token {
        value: var_name,
        kind: TokenKind::Ident,
    }) = parser.next_token()
    else {
        return Err(ParseError::Expected("Variable".into(), TokenKind::Ident));
    };
    let typed = parser.try_parse_gen(type_anot).map_or(None, |x| Some(x));
    let Some(Token {
        kind: TokenKind::Eq,
        ..
    }) = parser.next_token()
    else {
        return Err(ParseError::Expected("Variable".into(), TokenKind::Eq));
    };
    let expr = expr(parser)?;
    Ok(ast::Stmt::Let(var_name, typed, expr))
}

fn func(parser: &mut Parser) -> ParseResult {
    let Some(Token {
        kind: TokenKind::Let,
        ..
    }) = parser.next_token()
    else {
        panic!("Func was called even tho their is no Let.")
    };
    let Some(Token {
        value: func_name,
        kind: TokenKind::Ident,
    }) = parser.next_token()
    else {
        return Err(ParseError::Expected("Variable".into(), TokenKind::Ident));
    };
    let params = params(parser)?;
    let typed = match parser.try_parse_gen(type_anot) {
        Ok(typed) => Some(typed),
        Err(_) => None,
    };
    let expr = expr(parser)?;
    Ok(ast::Stmt::Func(func_name, params, typed, expr))
}

impl Parser {
    pub fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token.cloned()
    }

    pub fn peek(&self) -> Option<Token> {
        let token = self.tokens.get(self.position);
        token.cloned()
    }

    pub fn parse(self) {}

    pub fn parse_stmt(&mut self) -> Option<ParseResult> {
        let token = self.peek()?;
        match token.kind {
            token::TokenKind::String
            | token::TokenKind::Int
            | token::TokenKind::Float
            | token::TokenKind::Ident => {
                let expr = match expr(self) {
                    Ok(expr) => expr,
                    Err(err) => return Some(Err(err)),
                };
                Some(Ok(ast::Stmt::Expr(expr)))
            }
            token::TokenKind::Let => Some(self.parse_let()),
            token::TokenKind::If => Some(self.parse_if()),
            token::TokenKind::Else => todo!(),
            token::TokenKind::For => todo!(),
            token::TokenKind::While => todo!(),
            token::TokenKind::Match => todo!(),
            token::TokenKind::Comma => todo!(),
            _ => todo!(
                "We really shouldn't be handling this every, but I will do some more thinking."
            ),
        }
    }

    pub fn try_parse(&mut self, try_parse: ParseFn) -> ParseResult {
        let position = self.position;
        match try_parse(self) {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.position = position;
                Err(err)
            }
        }
    }

    pub fn try_parse_gen<T>(&mut self, try_parse: ParseFnGen<T>) -> ParseResultGen<T> {
        let position = self.position;
        match try_parse(self) {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.position = position;
                Err(err)
            }
        }
    }

    pub fn parse_let(&mut self) -> ParseResult {
        self.parse_or_try(var, func)
    }

    pub fn parse_if(&mut self) -> ParseResult {
        parse_if(self)
    }

    pub fn parse_or_try(&mut self, try_parse: ParseFn, or_try: ParseFn) -> ParseResult {
        match self.try_parse(try_parse) {
            Ok(stmt) => Ok(stmt),
            Err(_) => self.try_parse(or_try),
        }
    }
}
