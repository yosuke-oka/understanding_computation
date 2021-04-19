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
}
