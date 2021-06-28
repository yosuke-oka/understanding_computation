use crate::fa_rule::{FARule, State};
use std::collections::HashSet;
use std::iter::FromIterator;

pub const FREE_MOVE: char = '\x00';

#[derive(Clone)]
pub struct NFARulebook {
    pub rules: Vec<FARule>,
}

impl NFARulebook {
    pub fn build(args: Vec<(State, char, State)>) -> NFARulebook {
        NFARulebook {
            rules: args.iter().map(|&t| FARule::new(t)).collect(),
        }
    }
    fn next_states(&self, states: &HashSet<State>, character: char) -> HashSet<State> {
        HashSet::from_iter(
            states
                .into_iter()
                .flat_map(|state| self.follow_rules_for(state, character)),
        )
    }
    fn follow_rules_for(&self, state: &State, character: char) -> HashSet<State> {
        self.rules_for(state, character)
            .iter()
            .map(|r| r.follow())
            .collect()
    }
    fn rules_for(&self, state: &State, character: char) -> Vec<FARule> {
        self.rules
            .iter()
            .filter(|r| r.is_applied_to(*state, character))
            .cloned()
            .collect()
    }
    fn follow_free_moves(&self, states: &HashSet<State>) -> HashSet<State> {
        let more_states = self.next_states(states, FREE_MOVE);
        if more_states.is_subset(states) {
            states.clone()
        } else {
            self.follow_free_moves(&states.union(&more_states).cloned().collect())
        }
    }
}

struct NFA {
    current_states: HashSet<State>,
    accept_states: HashSet<State>,
    rulebook: NFARulebook,
}

impl NFA {
    fn is_accept(&self) -> bool {
        !self
            .get_current_states()
            .intersection(&self.accept_states)
            .collect::<HashSet<_>>()
            .is_empty()
    }
    fn read_character(&mut self, character: char) -> () {
        self.current_states = self
            .rulebook
            .next_states(&self.get_current_states(), character)
    }
    fn read_string(&mut self, string: &str) -> () {
        for c in string.chars() {
            self.read_character(c);
        }
    }
    fn get_current_states(&self) -> HashSet<State> {
        self.rulebook.follow_free_moves(&self.current_states)
    }
}

pub struct NFADesign {
    start_state: State,
    accept_states: HashSet<State>,
    rulebook: NFARulebook,
}

impl NFADesign {
    pub fn new(arg: (State, HashSet<State>, NFARulebook)) -> NFADesign {
        NFADesign {
            start_state: arg.0,
            accept_states: arg.1,
            rulebook: arg.2,
        }
    }
    fn to_nfa(&self) -> NFA {
        NFA {
            current_states: vec![self.start_state].into_iter().collect(),
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone(),
        }
    }
    pub fn is_accept(&self, string: &str) -> bool {
        let mut nfa = self.to_nfa();
        nfa.read_string(string);
        nfa.is_accept()
    }
}
