use std::fmt;

pub enum Pattern {
    Empty,
    Literal(char),
    Concatnate {
        first: Box<Pattern>,
        second: Box<Pattern>,
    },
    Choose {
        first: Box<Pattern>,
        second: Box<Pattern>,
    },
    Repeat(Box<Pattern>),
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pattern::Empty => write!(f, ""),
            Pattern::Literal(c) => write!(f, "{}", c),
            Pattern::Concatnate { first, second } => write!(
                f,
                "{}{}",
                first.bracket(self.precedence()),
                second.bracket(self.precedence())
            ),
            Pattern::Choose { first, second } => write!(
                f,
                "{}|{}",
                first.bracket(self.precedence()),
                second.bracket(self.precedence())
            ),
            Pattern::Repeat(p) => write!(f, "{}*", p.bracket(self.precedence())),
        }
    }
}

impl Pattern {
    pub fn precedence(&self) -> u32 {
        match self {
            Pattern::Empty => 3,
            Pattern::Literal(_) => 3,
            Pattern::Concatnate {
                first: _,
                second: _,
            } => 1,
            Pattern::Choose {
                first: _,
                second: _,
            } => 0,
            Pattern::Repeat(_) => 2,
        }
    }
    pub fn bracket(&self, outer_precedence: u32) -> String {
        if self.precedence() < outer_precedence {
            format!("({})", self)
        } else {
            self.to_string()
        }
    }
}
