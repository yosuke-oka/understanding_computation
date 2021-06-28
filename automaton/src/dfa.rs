use crate::fa_rule::{FARule, State};
use std::collections::HashSet;

#[derive(Clone)]
pub struct DFARulebook {
    pub rules: Vec<FARule>,
}

impl DFARulebook {
    pub fn build(args: Vec<(State, char, State)>) -> DFARulebook {
        DFARulebook {
            rules: args.iter().map(|&t| FARule::new(t)).collect(),
        }
    }
    fn next_state(&self, state: State, character: char) -> State {
        self.rule_for(state, character).follow()
    }
    fn rule_for(&self, state: State, character: char) -> &FARule {
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
