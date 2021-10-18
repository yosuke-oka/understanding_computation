use crate::stack::Stack;
use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct PDAConfiguration {
    state: State,
    stack: Stack,
}

impl PDAConfiguration {
    pub fn new(state: State, stack: Stack) -> Self {
        PDAConfiguration {
            state: state,
            stack: stack,
        }
    }
}

#[derive(Debug)]
pub struct PDARule {
    state: State,
    character: char,
    next_state: State,
    pop_character: char,
    push_characters: Vec<char>,
}

impl PDARule {
    pub fn new(
        state: State,
        character: char,
        next_state: State,
        pop_character: char,
        push_characters: Vec<char>,
    ) -> Self {
        PDARule {
            state: state,
            character: character,
            next_state: next_state,
            pop_character: pop_character,
            push_characters: push_characters,
        }
    }
    pub fn is_applied_to(&self, configuration: PDAConfiguration, character: char) -> bool {
        self.state == configuration.state
            && Some(&self.pop_character) == configuration.stack.top()
            && self.character == character
    }
    pub fn follow(&self, configuration: PDAConfiguration) -> PDAConfiguration {
        PDAConfiguration::new(self.next_state, self.next_stack(configuration))
    }
    pub fn next_stack(&self, configuration: PDAConfiguration) -> Stack {
        let poped_stack = configuration.stack.pop();
        self.push_characters
            .iter()
            .rev()
            .fold(poped_stack, |stack, &character| stack.push(character))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pda_configuration_test() {
        let rule = PDARule::new(1, '(', 2, '$', vec!['b', '$']);
        let configuration = PDAConfiguration::new(1, Stack::new(vec!['$']));
        let expected = PDAConfiguration::new(2, Stack::new(vec!['b', '$']));
        assert_eq!(rule.follow(configuration), expected);
    }
}
