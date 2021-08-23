use crate::fa_rule::FARule;
use crate::state::State;
use std::collections::HashSet;

#[derive(Clone)]
pub struct DFARulebook {
    pub rules: Vec<FARule<State>>,
}

impl DFARulebook {
    pub fn new(rules: Vec<FARule<State>>) -> Self {
        DFARulebook { rules: rules }
    }
    pub fn build(args: Vec<(State, char, State)>) -> Self {
        DFARulebook {
            rules: args.iter().map(|&t| FARule::new(t)).collect(),
        }
    }
    fn next_state(&self, state: State, character: char) -> State {
        *self.rule_for(state, character).follow()
    }
    fn rule_for(&self, state: State, character: char) -> &FARule<State> {
        self.rules
            .iter()
            .find(|r| r.is_applied_to(state, character))
            .unwrap()
    }
}

struct DFA {
    current_state: State,
    accept_states: HashSet<State>,
    rulebook: DFARulebook,
}

impl DFA {
    fn is_accept(&self) -> bool {
        self.accept_states.contains(&self.current_state)
    }
    fn read_character(&mut self, character: char) -> () {
        self.current_state = self.rulebook.next_state(self.current_state, character);
    }
    fn read_string(&mut self, string: &str) -> () {
        for c in string.chars() {
            self.read_character(c);
        }
    }
}

pub struct DFADesign {
    current_state: State,
    accept_states: HashSet<State>,
    rulebook: DFARulebook,
}

impl DFADesign {
    pub fn new(arg: (State, HashSet<State>, DFARulebook)) -> DFADesign {
        DFADesign {
            current_state: arg.0,
            accept_states: arg.1,
            rulebook: arg.2,
        }
    }
    fn to_dfa(&self) -> DFA {
        DFA {
            current_state: self.current_state,
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone(),
        }
    }
    pub fn is_accept(&self, string: &str) -> bool {
        let mut dfa = self.to_dfa();
        dfa.read_string(string);
        dfa.is_accept()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfa_test() {
        let rulebook = DFARulebook::build(vec![
            (1, 'a', 2),
            (1, 'b', 1),
            (2, 'a', 2),
            (2, 'b', 3),
            (3, 'a', 3),
            (3, 'b', 3),
        ]);

        let accept_states = vec![3].into_iter().collect();
        let dfa_design = DFADesign::new((1, accept_states, rulebook));
        assert!(!dfa_design.is_accept("a"));
        assert!(!dfa_design.is_accept("baa"));
        assert!(dfa_design.is_accept("baba"));
    }
}
