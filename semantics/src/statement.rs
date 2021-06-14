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
    Sequence {
        first: Box<Statement>,
        second: Box<Statement>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
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
            Statement::Sequence { first, second } => write!(f, "{}; {}", first, second),
            Statement::While { condition, body } => {
                write!(f, "while ({}), {{ {} }}", condition, body)
            }
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
            Statement::Sequence { first, second } => {
                if let Statement::DoNothing = **first {
                    (*second.clone(), environment.clone())
                } else {
                    let (reduced_first, reduced_env) = first.reduce(environment);
                    (
                        Statement::Sequence {
                            first: Box::new(reduced_first),
                            second: second.clone(),
                        },
                        reduced_env,
                    )
                }
            }
            Statement::While { condition, body } => (
                Statement::If {
                    condition: condition.clone(),
                    consequence: Box::new(Statement::Sequence {
                        first: body.clone(),
                        second: Box::new(self.clone()),
                    }),
                    alternative: Box::new(Statement::DoNothing),
                },
                environment.clone(),
            ),
            _ => unreachable!(),
        }
    }
    pub fn evaluate(&self, environment: &mut Environment) -> Environment {
        match self {
            Statement::DoNothing => environment.clone(),
            Statement::Assignment { name, expression } => {
                environment.insert(name.to_string(), expression.evaluate(environment));
                environment.clone()
            }
            Statement::If {
                condition,
                consequence,
                alternative,
            } => match condition.evaluate(environment) {
                Expression::Boolean(true) => consequence.evaluate(environment),
                Expression::Boolean(false) => alternative.evaluate(environment),
                _ => panic!("condition is not bool"),
            },
            Statement::While { condition, body } => match condition.evaluate(environment) {
                Expression::Boolean(true) => self.evaluate(&mut body.evaluate(environment)),
                Expression::Boolean(false) => environment.clone(),
                _ => panic!("condition is not bool"),
            },
            Statement::Sequence { first, second } => {
                second.evaluate(&mut first.evaluate(environment))
            }
        }
    }
    pub fn to_ruby(&self) -> String {
        match self {
            Statement::DoNothing => "-> e { e }".to_string(),
            Statement::Assignment { name, expression } => {
                format!(
                    "-> e {{ e.merge({{ :{} => ({}).call(e) }}) }}",
                    name,
                    expression.to_ruby()
                )
            }
            Statement::If {
                condition,
                consequence,
                alternative,
            } => {
                format!(
                    "-> e {{ if({}).call(e) then ({}).call(e) else ({}).call(e) end }}",
                    condition.to_ruby(),
                    consequence.to_ruby(),
                    alternative.to_ruby()
                )
            }
            Statement::Sequence { first, second } => {
                format!(
                    "-> e {{ ({}).call(({}).call(e)) }}",
                    second.to_ruby(),
                    first.to_ruby()
                )
            }
            Statement::While { condition, body } => {
                format!(
                    "-> e {{ while ({}).call(e); e = ({}).call(e); end; e }}",
                    condition.to_ruby(),
                    body.to_ruby()
                )
            }
        }
    }
}
