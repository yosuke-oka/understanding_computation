use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
enum Expression {
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

type Environment = HashMap<String, Box<Expression>>;

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
    fn is_reducible(&self) -> bool {
        match self {
            Expression::Number(_) => false,
            Expression::Boolean(_) => false,
            _ => true,
        }
    }
    fn reduce(&self, environment: &Environment) -> Box<Expression> {
        match self {
            Expression::Add {
                ref left,
                ref right,
            } => {
                if left.is_reducible() {
                    Box::new(Expression::Add {
                        left: left.reduce(environment),
                        right: right.clone(),
                    })
                } else if right.is_reducible() {
                    Box::new(Expression::Add {
                        left: left.clone(),
                        right: right.reduce(environment),
                    })
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (Expression::Number(left_value), Expression::Number(right_value)) => {
                            Box::new(Expression::Number(left_value + right_value))
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
                    Box::new(Expression::Multiply {
                        left: left.reduce(environment),
                        right: right.clone(),
                    })
                } else if right.is_reducible() {
                    Box::new(Expression::Multiply {
                        left: left.clone(),
                        right: right.reduce(environment),
                    })
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (Expression::Number(left_value), Expression::Number(right_value)) => {
                            Box::new(Expression::Number(left_value * right_value))
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
                    Box::new(Expression::LessThan {
                        left: left.reduce(environment),
                        right: right.clone(),
                    })
                } else if right.is_reducible() {
                    Box::new(Expression::LessThan {
                        left: left.clone(),
                        right: right.reduce(environment),
                    })
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (Expression::Number(left_value), Expression::Number(right_value)) => {
                            Box::new(Expression::Boolean(left_value < right_value))
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

enum Statement {
    DoNothing,
    Assignment {
        name: String,
        expression: Box<Expression>,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::DoNothing => write!(f, "do-nothing"),
            Statement::Assignment { name, expression } => write!(f, "{} = {}", name, expression),
        }
    }
}

impl Statement {
    fn is_reducible(&self) -> bool {
        match self {
            Statement::DoNothing => false,
            _ => true,
        }
    }
    fn reduce(&self, environment: &mut Environment) -> (Statement, Environment) {
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
            _ => unreachable!(),
        }
    }
}

struct Machine {
    expression: Box<Expression>,
    environment: Environment,
}

impl Machine {
    fn step(&mut self) {
        self.expression = self.expression.reduce(&self.environment)
    }
    fn run(&mut self) {
        while self.expression.is_reducible() {
            println!("{}", self.expression);
            self.step();
        }
        println!("{}", self.expression);
    }
}

fn main() {
    let expression = Box::new(Expression::Add {
        left: Box::new(Expression::Multiply {
            left: Box::new(Expression::Number(1)),
            right: Box::new(Expression::Number(2)),
        }),
        right: Box::new(Expression::Multiply {
            left: Box::new(Expression::Number(3)),
            right: Box::new(Expression::Number(4)),
        }),
    });

    let mut machine = Machine {
        expression: expression,
        environment: HashMap::new(),
    };
    machine.run();

    println!("--");
    let expression = Box::new(Expression::LessThan {
        left: Box::new(Expression::Number(5)),
        right: Box::new(Expression::Add {
            left: Box::new(Expression::Number(2)),
            right: Box::new(Expression::Number(2)),
        }),
    });

    let mut machine = Machine {
        expression: expression,
        environment: HashMap::new(),
    };
    machine.run();

    println!("--");

    let expression = Box::new(Expression::Add {
        left: Box::new(Expression::Variable(String::from("x"))),
        right: Box::new(Expression::Variable(String::from("y"))),
    });
    let mut environment = HashMap::new();
    environment.insert(String::from("x"), Box::new(Expression::Number(3)));
    environment.insert(String::from("y"), Box::new(Expression::Number(4)));

    let mut machine = Machine {
        expression: expression,
        environment: environment,
    };
    machine.run();

    println!("--");

    let expression = Box::new(Expression::Add {
        left: Box::new(Expression::Variable(String::from("x"))),
        right: Box::new(Expression::Number(1)),
    });
    let statement = Statement::Assignment {
        name: String::from("x"),
        expression: expression,
    };

    let mut environment = HashMap::new();
    environment.insert(String::from("x"), Box::new(Expression::Number(2)));

    println!("{}", statement);

    let mut x = statement.reduce(&mut environment);
    println!("{}, {:?}", x.0, x.1);
    let mut x = x.0.reduce(&mut x.1);
    println!("{}, {:?}", x.0, x.1);
    let x = x.0.reduce(&mut x.1);
    println!("{}, {:?}", x.0, x.1);
    println!("{}", x.0.is_reducible());
}
