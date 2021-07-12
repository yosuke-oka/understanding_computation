use rand::prelude::*;
pub type State = u32;

pub fn new_state() -> State {
    random()
}
