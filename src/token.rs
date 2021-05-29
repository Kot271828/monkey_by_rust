#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Ident(Vec<char>),
    Int(Vec<char>),

    Plus,
    Minus,
    Asterisk,
    Slash,

    Bang,
    Assign,
    Eq,
    NotEq,

    Lt,
    Gt,

    Comma,
    SemiColon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // keyword
    Function,
    Let,
    Return,
    True,
    False,
    If,
    Else,
}

impl Token {
    pub fn lookup_keyword(literal: &Vec<char>) -> Option<Self> {
        let literal_str: &str = &literal.iter().collect::<String>();
        match literal_str {
            "fn" => Some(Token::Function),
            "let" => Some(Token::Let),
            "return" => Some(Token::Return),
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            _ => None,
        }
    }
}
