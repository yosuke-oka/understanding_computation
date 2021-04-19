use std::fmt;

#[derive(Clone)]
enum Expression {
    Number {
        value: u32,
    },
    Add {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Multiply {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Number { ref value } => write!(f, "{}", value),
            Expression::Add {
                ref left,
                ref right,
            } => write!(f, "{} + {}", left, right),
            Expression::Multiply {
                ref left,
                ref right,
            } => write!(f, "{} * {}", left, right),
        }
    }
}

impl Expression {
    fn is_reducible(&self) -> bool {
        match *self {
            Expression::Number { value: _ } => false,
            _ => true,
        }
    }
    fn reduce(&self) -> Box<Expression> {
        match *self {
            Expression::Add {
                ref left,
                ref right,
            } => {
                if left.is_reducible() {
                    Box::new(Expression::Add {
                        left: left.reduce(),
                        right: right.clone(),
                    })
                } else if right.is_reducible() {
                    Box::new(Expression::Add {
                        left: left.clone(),
                        right: right.reduce(),
                    })
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (
                            Expression::Number { value: left_value },
                            Expression::Number { value: right_value },
                        ) => Box::new(Expression::Number {
                            value: left_value + right_value,
                        }),
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
                        left: left.reduce(),
                        right: right.clone(),
                    })
                } else if right.is_reducible() {
                    Box::new(Expression::Multiply {
                        left: left.clone(),
                        right: right.reduce(),
                    })
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (
                            Expression::Number { value: left_value },
                            Expression::Number { value: right_value },
                        ) => Box::new(Expression::Number {
                            value: left_value * right_value,
                        }),
                        _ => unreachable!(),
                    }
                }
            }
            _ => panic!("mada jissou sitenai"),
        }
    }
}

struct Machine {
    expression: Box<Expression>,
}

impl Machine {
    fn step(&mut self) {
        self.expression = self.expression.reduce()
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
            left: Box::new(Expression::Number { value: 1 }),
            right: Box::new(Expression::Number { value: 2 }),
        }),
        right: Box::new(Expression::Multiply {
            left: Box::new(Expression::Number { value: 3 }),
            right: Box::new(Expression::Number { value: 4 }),
        }),
    });
    println!("{}", expression);
    println!("{}", expression.is_reducible());
    println!("{}", Expression::Number { value: 1 }.is_reducible());
    println!("{}", expression.reduce());
    println!("{}", expression.reduce().reduce());
    println!("{}", expression.reduce().reduce().reduce());
    println!("--");

    let mut machine = Machine {
        expression: expression,
    };
    machine.run();
}
