use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub struct FARule<T> {
    state: T,
    character: char,
    next_state: T,
}

impl<T: PartialEq> FARule<T> {
    pub fn new(arg: (T, char, T)) -> FARule<T> {
        FARule {
            state: arg.0,
            character: arg.1,
            next_state: arg.2,
        }
    }
    pub fn character(&self) -> char {
        self.character
    }
    pub fn is_applied_to(&self, state: T, character: char) -> bool {
        self.state == state && self.character == character
    }
    pub fn follow(&self) -> &T {
        &self.next_state
    }
}

impl<T: Debug> fmt::Display for FARule<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<FARule {:?} --{}--> {:?}>",
            self.state, self.character, self.next_state
        )
    }
}
