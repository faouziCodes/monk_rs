#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    RightArrow,
    LeftArrow,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    RightCurlyBracket,
    LeftCurlyBracket,
    String,
    Int,
    Float,
    Ident,
    Colon,
    Comma,
    Let,
    If,
    Else,
    For,
    While,
    Match,
    Eq,
    Op(Operator),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    EqEq,
    More,
    MoreEq,
    Less,
    LessEq,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Self {
        Self { kind, value }
    }

    pub fn kw_or_ident(value: String) -> Self {
        let kind = match value.as_str() {
            "let" => TokenKind::Let,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "match" => TokenKind::Match,
            _ => TokenKind::Ident,
        };
        Self { kind, value }
    }
}
