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
            Formula::Number { value } => write!(f, "{}", value),
            Formula::Add { left, right } => write!(f, "{} + {}", left, right),
            Formula::Multiply { left, right } => write!(f, "{} + {}", left, right),
        }
    }
}

fn main() {
    let multi = Formula::Number { value: 4 };
    println!("{}", multi);
}
