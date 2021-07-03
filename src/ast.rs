// Program
pub struct Program {
    statements: Vec<StatementNode>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: StatementNode) {
        self.statements.push(statement);
    }

    pub fn statement_iter(&self) -> std::slice::Iter<'_, StatementNode> {
        self.statements.iter()
    }

    pub fn literal(&self) -> String {
        let mut literal = String::new();
        for statement in &self.statements {
            literal.push_str(&statement.literal());
        }
        literal
    }
}

// StatementNode
pub enum StatementNode {
    LetStatement {
        identifier: Box<ExpressionNode>,
        value: Box<ExpressionNode>,
    },
    ReturnStatement {
        return_value: Box<ExpressionNode>,
    },
    ExpressionStatement {
        expression: Box<ExpressionNode>,
    },
    BlockStatement {
        statements: Vec<StatementNode>,
    },
}

impl StatementNode {
    pub fn literal(&self) -> String {
        match &self {
            StatementNode::LetStatement { identifier, value } => {
                format!("let {} = {};", identifier.literal(), value.literal())
            }
            StatementNode::ReturnStatement { return_value } => {
                format!("return {};", return_value.literal())
            }
            StatementNode::ExpressionStatement { expression } => {
                format!("{};", expression.literal())
            }
            StatementNode::BlockStatement { statements } => {
                let mut literal = String::new();
                for statement in statements {
                    literal = format!("{} {}", literal, statement.literal());
                }
                format!("{{{} }}", literal)
            }
        }
    }
}

// ExpressionNode
pub enum BooleanType {
    True,
    False,
}

pub enum PrefixOperatorType {
    Minus,
    Bang,
}

pub enum InfixOperatorType {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Eq,
    NotEq,
    Lt,
    Gt,
}

pub enum ExpressionNode {
    Identifier {
        literal: Vec<char>,
    },
    Integer {
        literal: Vec<char>,
    },
    Boolean {
        boolean_type: BooleanType,
    },
    PrefixOperator {
        operator_type: PrefixOperatorType,
        right: Box<ExpressionNode>,
    },
    InfixOperator {
        operator_type: InfixOperatorType,
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
    },
    IfExpression {
        condition: Box<ExpressionNode>,
        consequence: Box<StatementNode>,
        alternative: Option<Box<StatementNode>>,
    },
    FunctionLiteral {
        parameters: Vec<Box<ExpressionNode>>,
        body: Box<StatementNode>,
    },
    CallExpression {
        function: Box<ExpressionNode>,
        arguments: Vec<Box<ExpressionNode>>,
    },
}

impl ExpressionNode {
    pub fn literal(&self) -> String {
        match &self {
            ExpressionNode::Identifier { literal } => literal.iter().collect::<String>(),
            ExpressionNode::Integer { literal } => literal.iter().collect::<String>(),
            ExpressionNode::Boolean { boolean_type } => match boolean_type {
                BooleanType::True => "true".to_string(),
                BooleanType::False => "false".to_string(),
            },
            ExpressionNode::PrefixOperator {
                operator_type,
                right,
            } => {
                let operator_literal = match operator_type {
                    PrefixOperatorType::Bang => "!".to_string(),
                    PrefixOperatorType::Minus => "-".to_string(),
                };
                format!("({}{})", operator_literal, right.literal())
            }
            ExpressionNode::InfixOperator {
                operator_type,
                left,
                right,
            } => {
                let operator_literal = match operator_type {
                    InfixOperatorType::Plus => "+".to_string(),
                    InfixOperatorType::Minus => "-".to_string(),
                    InfixOperatorType::Asterisk => "*".to_string(),
                    InfixOperatorType::Slash => "/".to_string(),
                    InfixOperatorType::Eq => "==".to_string(),
                    InfixOperatorType::NotEq => "!=".to_string(),
                    InfixOperatorType::Gt => ">".to_string(),
                    InfixOperatorType::Lt => "<".to_string(),
                };
                format!(
                    "({} {} {})",
                    left.literal(),
                    operator_literal,
                    right.literal()
                )
            }
            ExpressionNode::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                let literal = format!("if {} {}", condition.literal(), consequence.literal());
                match &alternative {
                    Some(alternative) => format!("{} else {}", literal, alternative.literal()),
                    None => literal,
                }
            }
            ExpressionNode::FunctionLiteral { parameters, body } => {
                let mut parameters_literal = "".to_string();
                for parameter in parameters.iter() {
                    if parameters_literal == "" {
                        parameters_literal = parameter.literal();
                    } else {
                        parameters_literal =
                            format!("{}, {}", parameters_literal, parameter.literal());
                    }
                }

                format!("fn({}){}", parameters_literal, body.literal())
            }
            ExpressionNode::CallExpression {
                function,
                arguments,
            } => {
                let mut arguments_literal = "".to_string();
                for argument in arguments.iter() {
                    if arguments_literal == "" {
                        arguments_literal = argument.literal();
                    } else {
                        arguments_literal =
                            format!("{}, {}", arguments_literal, argument.literal());
                    }
                }

                format!("{}({})", function.literal(), arguments_literal)
            }
        }
    }
}
