use std::fmt;

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
                        right: *right,
                    })
                } else if right.is_reducible() {
                    Box::new(Expression::Add {
                        left: *left,
                        right: right.reduce(),
                    })
                } else {
                    Box::new(Expression::Number {
                        value: left.get_value() + right.get_value(),
                    })
                }
            }
            _ => panic!("mada jissou sitenai"),
        }
    }
    fn get_value(&self) -> u32 {
        match *self {
            Expression::Number { ref value } => *value,
            _ => panic!("not a Number"),
        }
    }
}

fn main() {
    let expression = Expression::Add {
        left: Box::new(Expression::Multiply {
            left: Box::new(Expression::Number { value: 1 }),
            right: Box::new(Expression::Number { value: 2 }),
        }),
        right: Box::new(Expression::Multiply {
            left: Box::new(Expression::Number { value: 3 }),
            right: Box::new(Expression::Number { value: 4 }),
        }),
    };
    println!("{}", expression);
    println!("{}", expression.is_reducible());
    println!("{}", Expression::Number { value: 1 }.is_reducible());
}
