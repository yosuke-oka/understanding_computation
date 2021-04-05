use std::fmt;

enum Formula {
    Number {
        value: u32,
    },
    Add {
        left: Box<Formula>,
        right: Box<Formula>,
    },
    Multiply {
        left: Box<Formula>,
        right: Box<Formula>,
    },
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Formula::Number { ref value } => write!(f, "{}", value),
            Formula::Add {
                ref left,
                ref right,
            } => write!(f, "{} + {}", left, right),
            Formula::Multiply {
                ref left,
                ref right,
            } => write!(f, "{} * {}", left, right),
        }
    }
}

fn main() {
    let formula = Formula::Add {
        left: Box::new(Formula::Multiply {
            left: Box::new(Formula::Number { value: 1 }),
            right: Box::new(Formula::Number { value: 2 }),
        }),
        right: Box::new(Formula::Multiply {
            left: Box::new(Formula::Number { value: 3 }),
            right: Box::new(Formula::Number { value: 4 }),
        }),
    };
    println!("{}", formula);
}
