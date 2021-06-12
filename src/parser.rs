use crate::ast::*;
use crate::token::Token;

struct Parser {
    token_vec: Vec<Token>,
    next_read_index: usize,
}

impl Parser {
    fn new(token_vec: Vec<Token>) -> Self {
        Parser {
            token_vec,
            next_read_index: 0,
        }
    }

    fn read_token(&mut self) -> Option<Token> {
        let return_value: Option<Token>;
        if self.next_read_index < self.token_vec.len() {
            return_value = Some(self.token_vec[self.next_read_index].clone());
            self.next_read_index += 1;
        } else {
            return_value = Some(Token::Eof);
        };
        return_value
    }

    fn peek_token(&self) -> Option<Token> {
        if self.next_read_index < self.token_vec.len() {
            Some(self.token_vec[self.next_read_index].clone())
        } else {
            Some(Token::Eof)
        }
    }

    fn expect_token(&self, expect_token: Token) -> Result<(), String> {
        let token_opt = self.peek_token();
        match token_opt {
            Some(token) => {
                if token == expect_token {
                    Ok(())
                } else {
                    Err(format!("expect: {:?}, actual: {:?}", expect_token, token)
                        .chars()
                        .collect())
                }
            }
            None => Err(format!("expect: {:?}, actual: None", expect_token)
                .chars()
                .collect()),
        }
    }

    fn parse_statement(&mut self) -> Result<StatementNode, String> {
        if let Some(token) = self.peek_token() {
            match token {
                Token::Let => self.parse_let_statement(),
                Token::Return => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
            }
        } else {
            Err("".to_string())
        }
    }

    fn parse_let_statement(&mut self) -> Result<StatementNode, String> {
        // Token::Let skip
        self.expect_token(Token::Let)?;
        self.read_token();

        // Identifier 読み込み
        let identifier = self.parse_identifier()?;

        // Token::Assign skip
        self.expect_token(Token::Assign)?;
        self.read_token();

        // ExpressionNode 読み込み
        let value = self.parse_expression(BindingPower::LOWEST)?;

        // Token::SemiColon が存在するならば skip
        if self.expect_token(Token::SemiColon).is_ok() {
            self.read_token();
        }

        // return
        Ok(StatementNode::LetStatement { identifier, value })
    }

    fn parse_return_statement(&mut self) -> Result<StatementNode, String> {
        // Token::Return skip
        self.expect_token(Token::Return)?;
        self.read_token();

        // ExpressionNode 読み込み
        let return_value = self.parse_expression(BindingPower::LOWEST)?;

        // Token::SemiColon が存在するならば skip
        if self.expect_token(Token::SemiColon).is_ok() {
            self.read_token();
        }

        // return
        Ok(StatementNode::ReturnStatement { return_value })
    }

    fn parse_expression_statement(&mut self) -> Result<StatementNode, String> {
        // ExpressionNode 読み込み
        let expression = self.parse_expression(BindingPower::LOWEST)?;

        // Token::SemiColon が存在するならば skip
        if self.expect_token(Token::SemiColon).is_ok() {
            self.read_token();
        }

        // return
        Ok(StatementNode::ExpressionStatement { expression })
    }

    fn parse_block_statement(&mut self) -> Result<Box<StatementNode>, String> {
        // Token::LBrace skip
        self.expect_token(Token::LBrace)?;
        self.read_token();

        let mut statements: Vec<StatementNode> = vec![];

        while (!self.expect_token(Token::RBrace).is_ok())
            && (!self.expect_token(Token::Eof).is_ok())
        {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        if self.expect_token(Token::Eof).is_ok() {
            return Err("in parse_block_statement".to_string());
        }

        // Token::RBrace skip
        self.read_token();

        // Token::SemiColon が存在するならば skip
        if self.expect_token(Token::SemiColon).is_ok() {
            self.read_token();
        }

        Ok(Box::new(StatementNode::BlockStatement { statements }))
    }

    fn parse_expression(
        &mut self,
        right_binding_power: BindingPower,
    ) -> Result<Box<ExpressionNode>, String> {
        let mut left = self.parse_nud_expression()?;

        let mut left_binding_power =
            to_binding_power(&self.peek_token().ok_or("None token".to_string())?);
        while right_binding_power < left_binding_power {
            left = self.parse_led_expression(left)?;
            left_binding_power =
                to_binding_power(&self.peek_token().ok_or("None token".to_string())?);
        }
        Ok(left)
    }

    fn parse_nud_expression(&mut self) -> Result<Box<ExpressionNode>, String> {
        let token = self.peek_token().ok_or("None token".to_string())?;
        // match
        let nud_expression: Box<ExpressionNode> = match token {
            Token::Ident(_) => self.parse_identifier()?,
            Token::Int(_) => self.parse_integer()?,
            Token::True | Token::False => self.parse_boolean()?,
            Token::LParen => self.parse_grouped_expression()?,
            Token::Bang | Token::Minus => self.parse_prefix()?,
            Token::If => self.parse_if_expression()?,
            Token::Function => self.parse_function_literal()?,
            _ => panic!("in parse_nud_expression"),
        };
        Ok(nud_expression)
    }

    fn parse_grouped_expression(&mut self) -> Result<Box<ExpressionNode>, String> {
        // Token::LParen skip
        self.expect_token(Token::LParen)?;
        self.read_token();

        let expression = self.parse_expression(BindingPower::LOWEST);

        // Token::RParen skip
        self.expect_token(Token::RParen)?;
        self.read_token();

        expression
    }

    fn parse_function_literal(&mut self) -> Result<Box<ExpressionNode>, String> {
        // Token::Function skip
        self.expect_token(Token::Function)?;
        self.read_token();

        // Token::LParen skip
        self.expect_token(Token::LParen)?;
        self.read_token();

        // parameters の読み込み
        let mut parameters = Vec::new();
        while !self.expect_token(Token::RParen).is_ok() {
            let parameter = self.parse_identifier()?;
            parameters.push(parameter);

            // Token::Comma が存在するならば skip
            if self.expect_token(Token::Comma).is_ok() {
                self.read_token();
            }
        }

        // Token::RParen skip
        self.expect_token(Token::RParen)?;
        self.read_token();

        // body の読み込み
        let body = self.parse_block_statement()?;

        Ok(Box::new(ExpressionNode::FunctionLiteral {
            parameters,
            body,
        }))
    }

    fn parse_prefix(&mut self) -> Result<Box<ExpressionNode>, String> {
        let operator_type = match self.read_token().ok_or("None token".to_string())? {
            Token::Bang => PrefixOperatorType::Bang,
            Token::Minus => PrefixOperatorType::Minus,
            _ => panic!("in parse_prefix"),
        };
        let right = self.parse_expression(BindingPower::PREFIX)?;
        Ok(Box::new(ExpressionNode::PrefixOperator {
            operator_type,
            right,
        }))
    }

    fn parse_if_expression(&mut self) -> Result<Box<ExpressionNode>, String> {
        // Token::If skip
        self.expect_token(Token::If)?;
        self.read_token();

        // Token::LParen skip
        self.expect_token(Token::LParen)?;
        self.read_token();

        let condition = self.parse_expression(BindingPower::LOWEST)?;

        // Token::RParen skip
        self.expect_token(Token::RParen)?;
        self.read_token();

        // BlockStatement 読み込み
        let consequence = self.parse_block_statement()?;

        // Token::Else が存在するなら, さらに読み込み
        let alternative = if self.expect_token(Token::Else).is_ok() {
            // Token::Else skip
            self.read_token();
            Some(self.parse_block_statement()?)
        } else {
            None
        };

        Ok(Box::new(ExpressionNode::IfExpression {
            condition,
            consequence,
            alternative,
        }))
    }

    fn parse_led_expression(
        &mut self,
        left: Box<ExpressionNode>,
    ) -> Result<Box<ExpressionNode>, String> {
        let token = self.peek_token().ok_or("None token".to_string())?;

        let led_expression: Box<ExpressionNode> = match token {
            Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Slash
            | Token::Eq
            | Token::NotEq
            | Token::Gt
            | Token::Lt => self.parse_infix(left)?,
            Token::LParen => self.parse_call_expression(left)?,
            _ => panic!("in parse_led_expression"),
        };

        Ok(led_expression)
    }

    fn parse_infix(&mut self, left: Box<ExpressionNode>) -> Result<Box<ExpressionNode>, String> {
        let token = self.read_token().ok_or("None token".to_string())?;
        let operator_type = match token {
            Token::Plus => InfixOperatorType::Plus,
            Token::Minus => InfixOperatorType::Minus,
            Token::Asterisk => InfixOperatorType::Asterisk,
            Token::Slash => InfixOperatorType::Slash,
            Token::Eq => InfixOperatorType::Eq,
            Token::NotEq => InfixOperatorType::NotEq,
            Token::Gt => InfixOperatorType::Gt,
            Token::Lt => InfixOperatorType::Lt,
            _ => panic!("in parse_infix"),
        };
        let right = self.parse_expression(to_binding_power(&token))?;
        Ok(Box::new(ExpressionNode::InfixOperator {
            operator_type,
            left,
            right,
        }))
    }

    fn parse_call_expression(
        &mut self,
        function: Box<ExpressionNode>,
    ) -> Result<Box<ExpressionNode>, String> {
        // Token::LParen skip
        self.expect_token(Token::LParen)?;
        self.read_token();

        // arguments の読み込み
        let mut arguments = Vec::new();
        while !self.expect_token(Token::RParen).is_ok() {
            let argument = self.parse_expression(BindingPower::LOWEST)?;
            arguments.push(argument);

            // Token::Comma が存在するならば skip
            if self.expect_token(Token::Comma).is_ok() {
                self.read_token();
            }
        }

        // Token::RParen skip
        self.expect_token(Token::RParen)?;
        self.read_token();

        // return
        Ok(Box::new(ExpressionNode::CallExpression {
            function,
            arguments,
        }))
    }

    fn parse_identifier(&mut self) -> Result<Box<ExpressionNode>, String> {
        let token_opt = self.read_token();
        match token_opt {
            Some(Token::Ident(literal)) => Ok(Box::new(ExpressionNode::Identifier { literal })),
            None => Err("not found token".to_string()),
            _ => Err("non-expected token".to_string()),
        }
    }

    fn parse_integer(&mut self) -> Result<Box<ExpressionNode>, String> {
        let token_opt = self.read_token();
        match token_opt {
            Some(Token::Int(literal)) => Ok(Box::new(ExpressionNode::Integer { literal })),
            None => Err("not found token".to_string()),
            _ => Err("non-expected token".to_string()),
        }
    }

    fn parse_boolean(&mut self) -> Result<Box<ExpressionNode>, String> {
        let token = self.read_token().ok_or("None token".to_string())?;
        match token {
            Token::True => Ok(Box::new(ExpressionNode::Boolean {
                boolean_type: BooleanType::True,
            })),
            Token::False => Ok(Box::new(ExpressionNode::Boolean {
                boolean_type: BooleanType::False,
            })),
            _ => Err("in parse_boolean".to_string()),
        }
    }
}

#[derive(PartialOrd, PartialEq)]
enum BindingPower {
    END,
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

fn to_binding_power(token: &Token) -> BindingPower {
    match token {
        Token::Eq | Token::NotEq => BindingPower::EQUALS,
        Token::Lt | Token::Gt => BindingPower::LESSGREATER,
        Token::Plus | Token::Minus => BindingPower::SUM,
        Token::Asterisk | Token::Slash => BindingPower::PRODUCT,
        Token::LParen => BindingPower::CALL,
        _ => BindingPower::END,
    }
}

pub fn parse(token_vec: Vec<Token>) -> Result<Program, String> {
    let mut parser = Parser::new(token_vec);
    let mut program = Program::new();
    while !parser.expect_token(Token::Eof).is_ok() {
        let statement = parser.parse_statement()?;
        program.add_statement(statement);
    }
    Ok(program)
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_parse(expect_strings: Vec<&str>, test_strings: Vec<&str>) {
        for (&test_string, &expect_sting) in test_strings.iter().zip(expect_strings.iter()) {
            let program = parse(crate::lexer::lex(test_string)).unwrap();
            let actual_string = &program.literal();
            assert_eq!(expect_sting, actual_string);
        }
    }
    #[test]
    fn test_let_statements() {
        let test_strings = vec!["let x = 5;", "let y = 10;", "let foobar = 838383;"];
        let expect_strings = vec!["let x = 5;", "let y = 10;", "let foobar = 838383;"];

        test_parse(expect_strings, test_strings);
    }

    #[test]
    fn test_return_statements() {
        let test_strings = vec!["return 5;", "return 10;", "return 993322;"];
        let expect_strings = vec!["return 5;", "return 10;", "return 993322;"];

        test_parse(expect_strings, test_strings);
    }

    #[test]
    fn test_identifier_exprssions() {
        let test_strings = vec!["foobar;"];
        let expect_strings = vec!["foobar;"];

        test_parse(expect_strings, test_strings);
    }

    #[test]
    fn test_integer_exprssions() {
        let test_strings = vec!["5;", "5"];
        let expect_strings = vec!["5;", "5;"];

        test_parse(expect_strings, test_strings);
    }

    #[test]
    fn test_prefix_exprssions() {
        let test_strings = vec!["!5;", "-15;"];
        let expect_strings = vec!["(!5);", "(-15);"];

        test_parse(expect_strings, test_strings);
    }

    #[test]
    fn test_infix_exprssions() {
        let test_strings = vec![
            "5 + 5;", "5 - 5;", "5 * 5;", "5 / 5;", "5 > 5;", "5 < 5;", "5 == 5;", "5 != 5;",
        ];
        let expect_strings = vec![
            "(5 + 5);",
            "(5 - 5);",
            "(5 * 5);",
            "(5 / 5);",
            "(5 > 5);",
            "(5 < 5);",
            "(5 == 5);",
            "(5 != 5);",
        ];

        test_parse(expect_strings, test_strings);
    }

    #[test]
    fn test_operator_binding_power() {
        let test_strings = vec![
            "-a + b;",
            "!-a;",
            "a + b + c;",
            "a + b - c;",
            "a * b * c;",
            "a * b / c;",
            "a + b / c;",
            "a + b * c + d / e - f;",
            "5 > 4 == 3 < 4;",
            "5 < 4 != 3 > 4;",
            "3 + 4 * 5 == 3 * 1 + 4 * 5;",
        ];
        let expect_strings = vec![
            "((-a) + b);",
            "(!(-a));",
            "((a + b) + c);",
            "((a + b) - c);",
            "((a * b) * c);",
            "((a * b) / c);",
            "(a + (b / c));",
            "(((a + (b * c)) + (d / e)) - f);",
            "((5 > 4) == (3 < 4));",
            "((5 < 4) != (3 > 4));",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)));",
        ];

        test_parse(expect_strings, test_strings);
    }

    #[test]
    fn test_boolean() {
        let test_strings = vec!["true;", "false;", "3 > 5 == false;", "3 < 5 == true;"];
        let expect_strings = vec![
            "true;",
            "false;",
            "((3 > 5) == false);",
            "((3 < 5) == true);",
        ];

        test_parse(expect_strings, test_strings);
    }

    #[test]
    fn test_if_expression() {
        let test_strings = vec!["if (x < y) { x; y };", "if (x < y) { x; y; } else { z }"];
        let expect_strings = vec!["if (x < y) { x; y; };", "if (x < y) { x; y; } else { z; };"];

        test_parse(expect_strings, test_strings);
    }

    #[test]
    fn test_function_literal() {
        let test_strings = vec!["fn(x, y){ x + y; };", "fn(){ 5 };"];
        let expect_strings = vec!["fn(x, y){ (x + y); };", "fn(){ 5; };"];

        test_parse(expect_strings, test_strings);
    }
    #[test]
    fn test_call_expression() {
        let test_strings = vec![
            "add(x, y);",
            "a + add(b * c) + d;",
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8));",
            "add(a + b + c * d / f + g);",
        ];
        let expect_strings = vec![
            "add(x, y);",
            "((a + add((b * c))) + d);",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)));",
            "add((((a + b) + ((c * d) / f)) + g));",
        ];

        test_parse(expect_strings, test_strings);
    }
}
