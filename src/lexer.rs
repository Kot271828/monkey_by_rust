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
}

// pub fn lex(input: &str) -> Result<Vec<Token>, Error> {

// }

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

    
}
