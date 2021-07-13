use crate::fa_rule::FARule;
use crate::state::State;
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
    pub fn rules(&self) -> &Vec<FARule> {
        &self.rules
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
    pub fn start_state(&self) -> &State {
        &self.start_state
    }
    pub fn accept_states(&self) -> &HashSet<State> {
        &self.accept_states
    }
    pub fn rulebook(&self) -> &NFARulebook {
        &self.rulebook
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nfa_test() {
        let rulebook = NFARulebook::build(vec![
            (1, 'a', 1),
            (1, 'b', 1),
            (1, 'b', 2),
            (2, 'a', 3),
            (2, 'b', 3),
            (3, 'a', 4),
            (3, 'b', 4),
        ]);
        let nfa_design = NFADesign::new((1, vec![4].iter().cloned().collect(), rulebook));
        assert!(nfa_design.is_accept("bab"));
        assert!(nfa_design.is_accept("bbbbb"));
        assert!(!nfa_design.is_accept("bbabb"));
    }

    #[test]
    fn free_move_test() {
        let rulebook = NFARulebook::build(vec![
            (1, FREE_MOVE, 2),
            (1, FREE_MOVE, 4),
            (2, 'a', 3),
            (3, 'a', 2),
            (4, 'a', 5),
            (5, 'a', 6),
            (6, 'a', 4),
        ]);
        let nfa_design = NFADesign::new((1, vec![2, 4].iter().cloned().collect(), rulebook));
        assert!(nfa_design.is_accept("aa"));
        assert!(nfa_design.is_accept("aaa"));
        assert!(!nfa_design.is_accept("aaaaa"));
        assert!(nfa_design.is_accept("aaaaaa"));
    }
}
