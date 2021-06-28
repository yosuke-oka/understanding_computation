use std::fmt;

#[derive(Clone)]
pub struct FARule {
    state: State,
    character: char,
    next_state: State,
}

pub type State = u32;

impl FARule {
    pub fn new(arg: (State, char, State)) -> FARule {
        FARule {
            state: arg.0,
            character: arg.1,
            next_state: arg.2,
        }
    }
    pub fn is_applied_to(&self, state: State, character: char) -> bool {
        self.state == state && self.character == character
    }
    pub fn follow(&self) -> State {
        self.next_state
    }
}

impl fmt::Display for FARule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<FARule {} --{}--> {}>",
            self.state, self.character, self.next_state
        )
    }
}
