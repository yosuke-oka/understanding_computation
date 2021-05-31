use std::fmt;

use crate::expression::{Environment, Expression};

#[derive(Clone, Debug)]
pub enum Statement {
    DoNothing,
    Assignment {
        name: String,
        expression: Expression,
    },
    If {
        condition: Expression,
        consequence: Box<Statement>,
        alternative: Box<Statement>,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::DoNothing => write!(f, "do-nothing"),
            Statement::Assignment { name, expression } => write!(f, "{} = {}", name, expression),
            Statement::If {
                condition,
                consequence,
                alternative,
            } => write!(
                f,
                "if ({}) {{ {} }} else {{ {} }}",
                condition, consequence, alternative
            ),
        }
    }
}

impl Statement {
    pub fn is_reducible(&self) -> bool {
        match self {
            Statement::DoNothing => false,
            _ => true,
        }
    }
    pub fn reduce(&self, environment: &mut Environment) -> (Statement, Environment) {
        match self {
            Statement::Assignment { name, expression } => {
                if expression.is_reducible() {
                    (
                        Statement::Assignment {
                            name: name.clone(),
                            expression: expression.reduce(environment),
                        },
                        environment.clone(),
                    )
                } else {
                    let mut new_env = environment.clone();
                    new_env.insert(String::from(name), expression.clone());
                    (Statement::DoNothing, new_env)
                }
            }
            Statement::If {
                condition,
                consequence,
                alternative,
            } => {
                if condition.is_reducible() {
                    (
                        Statement::If {
                            condition: condition.reduce(environment),
                            consequence: consequence.clone(),
                            alternative: alternative.clone(),
                        },
                        environment.clone(),
                    )
                } else {
                    match condition {
                        Expression::Boolean(true) => (*consequence.clone(), environment.clone()),
                        Expression::Boolean(false) => (*alternative.clone(), environment.clone()),
                        _ => panic!("condition is not bool"),
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
