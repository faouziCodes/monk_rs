use crate::token::{Token, TokenKind};

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Token(TokenKind),
    Looking,
}

pub struct Lexer {
    buffer: String,
    input: Vec<char>,
    state: State,
    token: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            buffer: String::new(),
            input: input.chars().collect(),
            state: State::Looking,
            token: Vec::new(),
        }
    }

    fn add_token(&mut self, token: Token) {
        self.token.push(token)
    }

    fn clear_buffer(&mut self) -> String {
        let buffer = self.buffer.clone();
        self.buffer.clear();
        buffer
    }

    fn next_state(&mut self, input: char) {
        match (&self.state, input) {
            (State::Looking, ',') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::Comma, buffer));
            }
            (State::Looking, ':') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::Colon, buffer));
            }
            (State::Looking, '+') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::Add, buffer));
            }
            (State::Looking, '*') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::Mul, buffer));
            }
            (State::Looking, '/') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::Div, buffer));
            }
            (State::Looking, '[') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::LeftBracket, buffer));
            }
            (State::Looking, ']') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::RightBracket, buffer));
            }
            (State::Looking, '{') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::LeftCurlyBracket, buffer));
            }
            (State::Looking, '}') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::RightCurlyBracket, buffer));
            }
            (State::Looking, '(') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::LeftBrace, buffer));
            }
            (State::Looking, ')') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::RightBrace, buffer));
            }
            (State::Looking, '=') => {
                self.buffer.push(input);
                self.state = State::Token(TokenKind::EqEq);
            }
            (State::Token(TokenKind::EqEq), '=') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::EqEq, buffer));
                self.state = State::Looking;
            }
            (State::Token(TokenKind::EqEq), input) => {
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::Eq, buffer));
                self.state = State::Looking;
                self.next_state(input);
            }
            (State::Looking, '>') => {
                self.buffer.push(input);
                self.state = State::Token(TokenKind::MoreEq);
            }
            (State::Token(TokenKind::MoreEq), '=') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::MoreEq, buffer));
                self.state = State::Looking;
            }
            (State::Token(TokenKind::MoreEq), input) => {
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::More, buffer));
                self.state = State::Looking;
                self.next_state(input);
            }
            (State::Looking, '<') => {
                self.buffer.push(input);
                self.state = State::Token(TokenKind::Less);
            }
            (State::Token(TokenKind::LessEq), '=') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::LessEq, buffer));
                self.state = State::Looking;
            }
            (State::Token(TokenKind::Less), input) => {
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::Less, buffer));
                self.state = State::Looking;
                self.next_state(input);
            }
            (State::Looking, '"') => {
                self.buffer.push(input);
                self.state = State::Token(TokenKind::String);
            }
            (State::Token(TokenKind::String), '\"') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::String, buffer));
                self.state = State::Looking;
            }
            (State::Token(TokenKind::String), input) => {
                self.buffer.push(input);
                self.state = State::Token(TokenKind::String);
            }
            (State::Looking, '-') => {
                self.buffer.push(input);
                self.state = State::Token(TokenKind::Sub);
            }
            (State::Token(TokenKind::Sub), '>') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::RightArrow, buffer));
                self.state = State::Looking;
            }
            (State::Token(TokenKind::Sub), '<') => {
                self.buffer.push(input);
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::LeftArrow, buffer));
                self.state = State::Looking;
            }
            (State::Token(TokenKind::Sub), input) => {
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::Sub, buffer));
                self.state = State::Looking;
                self.next_state(input);
            }
            (State::Looking, '0'..='9') => {
                self.buffer.push(input);
                self.state = State::Token(TokenKind::Int);
            }
            (State::Token(TokenKind::Int), '0'..='9') => {
                self.buffer.push(input);
            }
            (State::Token(TokenKind::Float), '0'..='9') => {
                self.buffer.push(input);
            }
            (State::Token(TokenKind::Int), '.') => {
                self.buffer.push(input);
                self.state = State::Token(TokenKind::Float);
            }
            (State::Token(TokenKind::Int), _) => {
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::Int, buffer));
                self.state = State::Looking;
                self.next_state(input);
            }
            (State::Token(TokenKind::Float), _) => {
                let buffer = self.clear_buffer();
                self.add_token(Token::new(TokenKind::Float, buffer));
                self.state = State::Looking;
                self.next_state(input);
            }
            (State::Looking, 'a'..='z' | 'A'..='Z') => {
                self.buffer.push(input);
                self.state = State::Token(TokenKind::Ident);
            }
            (State::Token(TokenKind::Ident), 'a'..='z' | 'A'..='Z' | '0'..='9') => {
                self.buffer.push(input);
            }
            (State::Token(TokenKind::Ident), _) => {
                let buffer = self.clear_buffer();
                self.add_token(Token::kw_or_ident(buffer));
                self.state = State::Looking;
                self.next_state(input);
            }
            (State::Looking, ' ' | '\n' | '\t') => {}
            _ => unreachable!("Should not happen."),
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        while let Some(_) = self.input.get(0) {
            let c = self.input.remove(0);
            self.next_state(c);
        }
        if let State::Token(kind) = self.state {
            self.token.push(Token::new(kind, self.buffer))
        }
        self.token
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        lexer::Lexer,
        token::{Token, TokenKind},
    };

    #[test]
    pub fn test_lexer() {
        let input = "let x = 10 =  : ";
        let lexer = Lexer::new(input.into());
        let tokens = lexer.lex();
        assert_eq!(tokens[0], Token::new(TokenKind::Let, "let".into()));
        assert_eq!(tokens[1], Token::new(TokenKind::Ident, "x".into()));
        assert_eq!(tokens[2], Token::new(TokenKind::Eq, "=".into()));
        assert_eq!(tokens[3], Token::new(TokenKind::Int, "10".into()));
        assert_eq!(tokens[4], Token::new(TokenKind::Eq, "=".into()));
        assert_eq!(tokens[5], Token::new(TokenKind::Colon, ":".into()));
    }
}
