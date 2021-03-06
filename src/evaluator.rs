use crate::ast::*;
use crate::env::*;
use crate::object::*;
use std::cell::RefCell;
use std::iter::Zip;
use std::rc::Rc;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eavl_program(
        &self,
        program: &Program,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let mut result = Object::Null;
        for statement in program.statement_iter() {
            result = self.eval_statement(statement, env)?;
            result = match result {
                Object::ReturnValue { value } => return Ok(*value),
                _ => result,
            }
        }

        Ok(result)
    }

    fn eval_statement(
        &self,
        statement: &StatementNode,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let result: Object;
        result = match statement {
            StatementNode::ReturnStatement { return_value: _ } => {
                self.eval_return_statement(statement, env)?
            }
            StatementNode::LetStatement {
                identifier: _,
                value: _,
            } => self.eval_let_statement(statement, env)?,
            StatementNode::ExpressionStatement { expression: _ } => {
                self.eval_expression_statement(statement, env)?
            }
            StatementNode::BlockStatement { statements: _ } => {
                self.eval_block_statement(statement, env)?
            }
        };

        Ok(result)
    }

    fn eval_let_statement(
        &self,
        statement: &StatementNode,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let (identifier, value) = match statement {
            StatementNode::LetStatement { identifier, value } => (identifier, value),
            _ => return Err("in eval_let_statement".to_string()),
        };
        let literal = match identifier.as_ref() {
            ExpressionNode::Identifier { literal } => literal,
            _ => return Err("in eval_let_statement".to_string()),
        };
        let object = self.eval_expression(value, env)?;
        env.as_ref()
            .borrow_mut()
            .set(&literal.iter().collect::<String>(), object);
        Ok(Object::Null)
    }

    fn eval_return_statement(
        &self,
        statement: &StatementNode,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let result = match statement {
            StatementNode::ReturnStatement { return_value } => {
                self.eval_expression(return_value, env)?
            }
            _ => return Err("in eval_return_statement".to_string()),
        };

        Ok(Object::ReturnValue {
            value: Box::new(result),
        })
    }

    fn eval_expression_statement(
        &self,
        statement: &StatementNode,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let expression = match statement {
            StatementNode::ExpressionStatement { expression } => {
                self.eval_expression(expression, env)?
            }
            _ => return Err("in eval_expression_statement".to_string()),
        };

        Ok(expression)
    }

    fn eval_block_statement(
        &self,
        statement: &StatementNode,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let statements = match statement {
            StatementNode::BlockStatement { statements } => statements,
            _ => return Err("in eval_block_statement".to_string()),
        };

        let mut result = Object::Null;
        for statement in statements {
            result = self.eval_statement(statement, env)?;
            result = match result {
                Object::ReturnValue { value: _ } => return Ok(result),
                _ => result,
            }
        }

        Ok(result)
    }

    fn eval_expression(
        &self,
        expression: &ExpressionNode,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let result = match expression {
            ExpressionNode::Integer { literal: _ } => self.eval_integer(expression)?,
            ExpressionNode::Boolean { boolean_type: _ } => self.eval_boolean(expression)?,
            ExpressionNode::Identifier { literal: _ } => self.eval_identifier(expression, env)?,
            ExpressionNode::PrefixOperator {
                operator_type: _,
                right: _,
            } => self.eval_prefix_operator(expression, env)?,
            ExpressionNode::InfixOperator {
                operator_type: _,
                left: _,
                right: _,
            } => self.eval_infix_operator(expression, env)?,
            ExpressionNode::IfExpression {
                condition: _,
                consequence: _,
                alternative: _,
            } => self.eval_if_expression(expression, env)?,
            ExpressionNode::FunctionLiteral {
                parameters: _,
                body: _,
            } => self.eval_function_literal(expression, env)?,
            ExpressionNode::CallExpression {
                function: _,
                arguments: _,
            } => self.eval_call_expression(expression, env)?,
            _ => panic!(""),
        };

        Ok(result)
    }

    fn eval_integer(&self, expression: &ExpressionNode) -> Result<Object, String> {
        let value = match expression {
            ExpressionNode::Integer { literal } => {
                literal.iter().collect::<String>().parse().unwrap()
            }
            _ => return Err("in eval_interger".to_string()),
        };

        Ok(Object::Integer { value })
    }

    fn eval_boolean(&self, expression: &ExpressionNode) -> Result<Object, String> {
        let boolean_type = match expression {
            ExpressionNode::Boolean { boolean_type } => boolean_type,
            _ => return Err("in eval_boolean".to_string()),
        };

        let value = match boolean_type {
            BooleanType::True => true,
            BooleanType::False => false,
        };

        Ok(Object::Boolean { value })
    }

    fn eval_identifier(
        &self,
        expression: &ExpressionNode,
        env: &Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let literal = match expression {
            ExpressionNode::Identifier { literal } => literal,
            _ => return Err("in eval_identifier".to_string()),
        };

        let literal_string = literal.iter().collect::<String>();
        let object = env
            .borrow()
            .get(&literal_string)
            .ok_or(format!("{} is not founded.", &literal_string))?;
        Ok(object)
    }

    fn eval_prefix_operator(
        &self,
        expression: &ExpressionNode,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let (operator_type, right) = match expression {
            ExpressionNode::PrefixOperator {
                operator_type,
                right,
            } => (operator_type, right),
            _ => return Err("in eval_boolean".to_string()),
        };

        let right_object = self.eval_expression(right, env)?;
        let result = match (operator_type, right_object) {
            (PrefixOperatorType::Minus, Object::Integer { value }) => {
                Object::Integer { value: -1 * value }
            }
            (PrefixOperatorType::Bang, Object::Boolean { value }) => {
                Object::Boolean { value: !value }
            }
            (_, _) => Object::Null,
        };

        Ok(result)
    }

    fn eval_infix_operator(
        &self,
        expression: &ExpressionNode,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let (oprator_type, left, right) = match expression {
            ExpressionNode::InfixOperator {
                operator_type,
                left,
                right,
            } => (operator_type, left, right),
            _ => return Err("in eval_infix_operator".to_string()),
        };

        let left_object = self.eval_expression(left, env)?;
        let right_object = self.eval_expression(right, env)?;

        let result = match (oprator_type, left_object, right_object) {
            (
                InfixOperatorType::Plus,
                Object::Integer { value: left_value },
                Object::Integer { value: right_value },
            ) => Object::Integer {
                value: left_value + right_value,
            },
            (
                InfixOperatorType::Minus,
                Object::Integer { value: left_value },
                Object::Integer { value: right_value },
            ) => Object::Integer {
                value: left_value - right_value,
            },
            (
                InfixOperatorType::Asterisk,
                Object::Integer { value: left_value },
                Object::Integer { value: right_value },
            ) => Object::Integer {
                value: left_value * right_value,
            },
            (
                InfixOperatorType::Slash,
                Object::Integer { value: left_value },
                Object::Integer { value: right_value },
            ) => Object::Integer {
                value: left_value / right_value,
            },
            (
                InfixOperatorType::Lt,
                Object::Integer { value: left_value },
                Object::Integer { value: right_value },
            ) => Object::Boolean {
                value: left_value < right_value,
            },
            (
                InfixOperatorType::Gt,
                Object::Integer { value: left_value },
                Object::Integer { value: right_value },
            ) => Object::Boolean {
                value: left_value > right_value,
            },
            (
                InfixOperatorType::Eq,
                Object::Integer { value: left_value },
                Object::Integer { value: right_value },
            ) => Object::Boolean {
                value: left_value == right_value,
            },
            (
                InfixOperatorType::NotEq,
                Object::Integer { value: left_value },
                Object::Integer { value: right_value },
            ) => Object::Boolean {
                value: left_value != right_value,
            },
            (
                InfixOperatorType::Eq,
                Object::Boolean { value: left_value },
                Object::Boolean { value: right_value },
            ) => Object::Boolean {
                value: left_value == right_value,
            },
            (
                InfixOperatorType::NotEq,
                Object::Boolean { value: left_value },
                Object::Boolean { value: right_value },
            ) => Object::Boolean {
                value: left_value != right_value,
            },
            (_, _, _) => Object::Null,
        };

        Ok(result)
    }

    fn eval_if_expression(
        &self,
        expression: &ExpressionNode,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let (condition, consequence, alternative) = match expression {
            ExpressionNode::IfExpression {
                condition,
                consequence,
                alternative,
            } => (condition, consequence, alternative),
            _ => return Err("in eval_if_expression".to_string()),
        };

        let condition_object = self.eval_expression(condition, env)?;
        let result = if self.is_truthy(&condition_object) {
            self.eval_statement(consequence, env)?
        } else {
            if alternative.is_some() {
                self.eval_statement(alternative.as_ref().unwrap(), env)?
            } else {
                Object::Null
            }
        };

        Ok(result)
    }

    fn eval_function_literal(&self, expression: &ExpressionNode, env: &mut Rc<RefCell<Enviroment>>) -> Result<Object, String> {
        let (parameters, body) = match expression {
            ExpressionNode::FunctionLiteral { parameters, body } => (parameters, body),
            _ => return Err(format!("on eval_function_literal. {}", expression.literal())),
        };

        let mut parameter_strings = Vec::<String>::new();
        for parameter in parameters {
            if let ExpressionNode::Identifier { literal } = parameter.as_ref() {
                parameter_strings.push(literal.iter().collect::<String>());
            } else {
                return Err("in eval_function_literal".to_string());
            }
        }

        Ok(Object::FunctionObject {
            parameters: parameter_strings,
            body: Box::new(body.as_ref().clone()),
            env: Some(Rc::clone(env)),
        })
    }

    fn eval_call_expression(
        &self,
        expression: &ExpressionNode,
        env: &mut Rc<RefCell<Enviroment>>,
    ) -> Result<Object, String> {
        let (function, arguments) = match expression {
            ExpressionNode::CallExpression {
                function,
                arguments,
            } => (function, arguments),
            _ => return Err("in eval_call_expression".to_string()),
        };

        let function = self.eval_expression(function, env)?;
        let (parameters, body, func_env) = match function {
            Object::FunctionObject {
                parameters,
                body,
                env: func_env,
            } => (parameters, body, func_env.unwrap()),
            _ => return Err("".to_string()),
        };

        let mut new_env = Enviroment::new();
        new_env.add_outer(&func_env);

        for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
            let evaled_object = self.eval_expression(argument, env)?;
            new_env.set(parameter, evaled_object);
        }

        let func_evaled_object =
            self.eval_block_statement(body.as_ref(), &mut Rc::new(RefCell::new(new_env)))?;

        match func_evaled_object {
            Object::ReturnValue { value } => Ok(value.as_ref().to_owned()),
            _ => Ok(func_evaled_object),
        }
    }

    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Null => false,
            Object::Boolean { value: true } => true,
            Object::Boolean { value: false } => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_eval(expect_strings: Vec<&str>, test_strings: Vec<&str>) {
        let evaluator = Evaluator::new();
        for (&test_string, &expect_sting) in test_strings.iter().zip(expect_strings.iter()) {
            let program = crate::parser::parse(crate::lexer::lex(test_string)).unwrap();
            let mut global_env = Rc::new(RefCell::new(Enviroment::new()));
            let actual_string = evaluator
                .eavl_program(&program, &mut global_env)
                .unwrap()
                .literal();
            assert_eq!(expect_sting, actual_string);
        }
    }

    #[test]
    fn test_eval_integer_expressions() {
        let test_strings = vec!["5", "10"];
        let expect_strings = vec!["5", "10"];

        test_eval(expect_strings, test_strings);
    }

    #[test]
    fn test_eval_boolean_expressions() {
        let test_strings = vec!["true", "false"];
        let expect_strings = vec!["true", "false"];

        test_eval(expect_strings, test_strings);
    }

    #[test]
    fn test_eval_prefix_operator() {
        let test_strings = vec!["!true", "!false", "-10", "--5"];
        let expect_strings = vec!["false", "true", "-10", "5"];

        test_eval(expect_strings, test_strings);
    }

    #[test]
    fn test_eval_infix_operator() {
        let test_strings = vec![
            "5 + 5 + 5 + 5 - 10",
            "2 * 2 * 2 * 2 * 2",
            "-50 + 100 + -50",
            "5 * 2 + 10",
            "5 + 2 * 10",
            "20 + 2 * -10",
            "50 / 2 * 2 + 10",
            "2 * (5 + 10)",
            "3 * 3 * 3 + 10",
            "3 * (3 * 3) + 10",
            "(5 + 10 * 2 + 15 / 3) * 2 + -10",
            "1 < 2",
            "1 > 2",
            "1 < 1",
            "1 > 1",
            "1 == 1",
            "1 != 1",
            "1 == 2",
            "1 != 2",
            "true == true",
            "false == false",
            "true == false",
            "true != false",
            "false != true",
            "(1 < 2) == true",
            "(1 < 2) == false",
            "(1 > 2) == true",
            "(1 > 2) == false",
        ];
        let expect_strings = vec![
            "10", "32", "0", "20", "25", "0", "60", "30", "37", "37", "50", "true", "false",
            "false", "false", "true", "false", "false", "true", "true", "true", "false", "true",
            "true", "true", "false", "false", "true",
        ];

        test_eval(expect_strings, test_strings);
    }

    #[test]
    fn test_eval_if_expression() {
        let test_strings = vec![
            "if (true) {10}",
            "if (false) {10}",
            "if (1) {10}",
            "if (1 < 2) {10}",
            "if (1 > 2) {10}",
            "if (1 > 2) {10} else {20}",
            "if (1 < 2) {10} else {20}",
        ];
        let expect_strings = vec!["10", "null", "10", "10", "null", "20", "10"];

        test_eval(expect_strings, test_strings);
    }

    #[test]
    fn test_eval_return_statements() {
        let test_strings = vec![
            "return 10",
            "return 10; 9",
            "return 2 * 5; 8;",
            "9; return 2 * 5; 7;",
            "if (10 > 1) { if (10 > 1) { return 10; } return 1; }",
        ];
        let expect_strings = vec!["10", "10", "10", "10", "10"];

        test_eval(expect_strings, test_strings);
    }

    #[test]
    fn test_eval_let_statements() {
        let test_strings = vec![
            "let a = 5; a;",
            "let a = 5 * 5; a;",
            "let a = 5; let b = a; b;",
            "let a = 5; let b = a; let c = a + b + 5; c;",
        ];
        let expect_strings = vec!["5", "25", "5", "15"];

        test_eval(expect_strings, test_strings);
    }

    #[test]
    fn test_eval_function_literals() {
        let test_strings = vec!["fn(x) { x + 2; };"];
        let expect_strings = vec!["fn(x) { (x + 2); }"];

        test_eval(expect_strings, test_strings);
    }

    #[test]
    fn test_eval_function_applications() {
        let test_strings = vec![
            "let identity = fn(x) { x; }; identity(5);",
            "let identity = fn(x) { return x; }; identity(10);",
            "let double = fn(x) { x * 2; }; double(10);",
            "let add = fn(x, y) { x + y; }; add(5, 10);",
            "let add = fn(x, y) { x + y; }; add(5 + 5, add(10, 10));",
            "let add = fn(a, b) { a + b; }; 
            let applyFunc = fn(a, b, func) { func(a, b) };
            applyFunc(10, 2, add);"
        ];
        let expect_strings = vec!["5", "10", "20", "15", "30", "12"];

        test_eval(expect_strings, test_strings);
    }
}
