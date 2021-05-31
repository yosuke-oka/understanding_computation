use std::collections::HashMap;
use std::fmt;

pub type Environment = HashMap<String, Expression>;

#[derive(Clone, Debug)]
pub enum Expression {
    Number(u32),
    Boolean(bool),
    Add {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Multiply {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Variable(String),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Number(value) => write!(f, "{}", value),
            Expression::Boolean(value) => write!(f, "{}", value),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Add {
                ref left,
                ref right,
            } => write!(f, "{} + {}", left, right),
            Expression::Multiply {
                ref left,
                ref right,
            } => write!(f, "{} * {}", left, right),
            Expression::LessThan {
                ref left,
                ref right,
            } => write!(f, "{} < {}", left, right),
        }
    }
}

impl Expression {
    pub fn is_reducible(&self) -> bool {
        match self {
            Expression::Number(_) => false,
            Expression::Boolean(_) => false,
            _ => true,
        }
    }
    pub fn reduce(&self, environment: &Environment) -> Expression {
        match self {
            Expression::Add {
                ref left,
                ref right,
            } => {
                if left.is_reducible() {
                    Expression::Add {
                        left: Box::new(left.reduce(environment)),
                        right: right.clone(),
                    }
                } else if right.is_reducible() {
                    Expression::Add {
                        left: left.clone(),
                        right: Box::new(right.reduce(environment)),
                    }
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (Expression::Number(left_value), Expression::Number(right_value)) => {
                            Expression::Number(left_value + right_value)
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Expression::Multiply {
                ref left,
                ref right,
            } => {
                if left.is_reducible() {
                    Expression::Multiply {
                        left: Box::new(left.reduce(environment)),
                        right: right.clone(),
                    }
                } else if right.is_reducible() {
                    Expression::Multiply {
                        left: left.clone(),
                        right: Box::new(right.reduce(environment)),
                    }
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (Expression::Number(left_value), Expression::Number(right_value)) => {
                            Expression::Number(left_value * right_value)
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Expression::LessThan {
                ref left,
                ref right,
            } => {
                if left.is_reducible() {
                    Expression::LessThan {
                        left: Box::new(left.reduce(environment)),
                        right: right.clone(),
                    }
                } else if right.is_reducible() {
                    Expression::LessThan {
                        left: left.clone(),
                        right: Box::new(right.reduce(environment)),
                    }
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (Expression::Number(left_value), Expression::Number(right_value)) => {
                            Expression::Boolean(left_value < right_value)
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Expression::Variable(name) => {
                if let Some(expression) = environment.get(name) {
                    expression.clone()
                } else {
                    panic!("undefined variable")
                }
            }
            _ => unreachable!(),
        }
    }
}
