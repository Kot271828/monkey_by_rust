use crate::token::Token;

struct Lexer {
    input: Vec<char>,
    next_read_index: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            next_read_index: 0,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        let return_value: Option<char>;
        if self.next_read_index < self.input.len() {
            return_value = Some(self.input[self.next_read_index]);
            self.next_read_index += 1;
        } else {
            return_value = None;
        };
        return_value
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.next_read_index < self.input.len() {
            Some(self.input[self.next_read_index])
        } else {
            None
        }
    }

    fn read_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            Some(',') => Token::Comma,
            Some(';') => Token::SemiColon,
            Some('<') => Token::Lt,
            Some('>') => Token::Gt,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(c) => {
                if Lexer::is_letter(c) {
                    self.lex_keyword_iden_token(c)
                } else if Lexer::is_digit(c) {
                    self.lex_int_token(c)
                } else {
                    Token::Illegal
                }
            }
            None => Token::Eof,
        }
    }

    fn lex_keyword_iden_token(&mut self, c: char) -> Token {
        let mut literal: Vec<char> = Vec::new();
        literal.push(c);
        while self
            .peek_char()
            .map(|c| Lexer::is_letter(c))
            .unwrap_or(false)
        {
            literal.push(self.read_char().unwrap());
        }
        if let Some(token) = Token::lookup_keyword(&literal) {
            token
        } else {
            Token::Ident(literal)
        }
    }

    fn lex_int_token(&mut self, c: char) -> Token {
        let mut literal: Vec<char> = Vec::new();
        literal.push(c);
        while self
            .peek_char()
            .map(|c| Lexer::is_digit(c))
            .unwrap_or(false)
        {
            literal.push(self.read_char().unwrap());
        }
        Token::Int(literal)
    }

    fn is_letter(c: char) -> bool {
        ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || (c == '_')
    }

    fn is_digit(c: char) -> bool {
        '0' <= c && c <= '9'
    }

    fn is_whitespace(c: char) -> bool {
        (c == ' ') || (c == '\t') || (c == '\n') || (c == '\r')
    }

    fn skip_whitespace(&mut self) {
        while self
            .peek_char()
            .map(|c| Lexer::is_whitespace(c))
            .unwrap_or(false)
        {
            self.read_char();
        }
    }
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut token_vec: Vec<Token> = Vec::new();
    let mut token = lexer.read_token();
    while token != Token::Eof {
        token_vec.push(token);
        token = lexer.read_token();
    }
    token_vec
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_char_test() {
        let input = "abc";
        let mut lexer = Lexer::new(&input);

        assert_eq!(lexer.read_char().unwrap_or(' '), 'a');
        assert_eq!(lexer.read_char().unwrap_or(' '), 'b');
        assert_eq!(lexer.read_char().unwrap_or(' '), 'c');
        assert_eq!(lexer.read_char(), None);
    }

    #[test]
    fn peek_char_test() {
        let input = "abc";
        let mut lexer = Lexer::new(&input);

        assert_eq!(lexer.read_char().unwrap_or(' '), 'a');
        assert_eq!(lexer.peek_char().unwrap_or(' '), 'b');
        assert_eq!(lexer.read_char().unwrap_or(' '), 'b');
        assert_eq!(lexer.peek_char().unwrap_or(' '), 'c');
        assert_eq!(lexer.read_char().unwrap_or(' '), 'c');
        assert_eq!(lexer.peek_char(), None);
        assert_eq!(lexer.read_char(), None);
    }

    #[test]
    fn skip_whitespace_test() {
        let input = "     a b  c ";
        let mut lexer = Lexer::new(&input);

        lexer.skip_whitespace();
        assert_eq!(lexer.read_char().unwrap_or(' '), 'a');
        lexer.skip_whitespace();
        assert_eq!(lexer.read_char().unwrap_or(' '), 'b');
        lexer.skip_whitespace();
        assert_eq!(lexer.read_char().unwrap_or(' '), 'c');
        lexer.skip_whitespace();
        assert_eq!(lexer.read_char(), None);
    }

    #[test]
    fn lexer_test() {
        let input = "let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        
        10 == 10;
        10 != 9;";

        let tests = vec![
            Token::Let,
            Token::Ident("five".chars().collect()),
            Token::Assign,
            Token::Int("5".chars().collect()),
            Token::SemiColon,
            Token::Let,
            Token::Ident("ten".chars().collect()),
            Token::Assign,
            Token::Int("10".chars().collect()),
            Token::SemiColon,
            Token::Let,
            Token::Ident("add".chars().collect()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".chars().collect()),
            Token::Comma,
            Token::Ident("y".chars().collect()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".chars().collect()),
            Token::Plus,
            Token::Ident("y".chars().collect()),
            Token::SemiColon,
            Token::RBrace,
            Token::SemiColon,
            Token::Let,
            Token::Ident("result".chars().collect()),
            Token::Assign,
            Token::Ident("add".chars().collect()),
            Token::LParen,
            Token::Ident("five".chars().collect()),
            Token::Comma,
            Token::Ident("ten".chars().collect()),
            Token::RParen,
            Token::SemiColon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".chars().collect()),
            Token::SemiColon,
            Token::Int("5".chars().collect()),
            Token::Lt,
            Token::Int("10".chars().collect()),
            Token::Gt,
            Token::Int("5".chars().collect()),
            Token::SemiColon,
            Token::If,
            Token::LParen,
            Token::Int("5".chars().collect()),
            Token::Lt,
            Token::Int("10".chars().collect()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::SemiColon,
            Token::RBrace,
            Token::Int("10".chars().collect()),
            Token::Eq,
            Token::Int("10".chars().collect()),
            Token::SemiColon,
            Token::Int("10".chars().collect()),
            Token::NotEq,
            Token::Int("9".chars().collect()),
            Token::SemiColon,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(&input);

        for expect_token in tests.iter() {
            let actual_token = lexer.read_token();
            println!("{:?}", expect_token);

            assert_eq!(actual_token, *expect_token);
        }
    }
}
