use crate::dfa::{DFADesign, DFARulebook};
use crate::fa_rule::FARule;
use crate::state::State;
use std::collections::HashSet;
use std::iter::FromIterator;

pub const FREE_MOVE: char = '\x00';

#[derive(Clone)]
pub struct NFARulebook {
    pub rules: Vec<FARule<State>>,
}

impl NFARulebook {
    pub fn build(args: Vec<(State, char, State)>) -> NFARulebook {
        NFARulebook {
            rules: args.iter().map(|&t| FARule::new(t)).collect(),
        }
    }
    pub fn rules(&self) -> &Vec<FARule<State>> {
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
            .cloned()
            .collect()
    }
    fn rules_for(&self, state: &State, character: char) -> Vec<FARule<State>> {
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
    fn alphabet(&self) -> HashSet<char> {
        self.rules
            .iter()
            .map(|r| r.character())
            .filter(|&c| c != FREE_MOVE)
            .collect()
    }
}

pub struct NFA {
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
    pub fn get_current_states(&self) -> HashSet<State> {
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
    pub fn to_nfa(&self) -> NFA {
        NFA {
            current_states: vec![self.start_state].into_iter().collect(),
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone(),
        }
    }
    fn to_nfa_simulation(&self, current_states: Vec<State>) -> NFA {
        NFA {
            current_states: current_states.iter().cloned().collect(),
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

pub struct NFASimulation {
    nfa_design: NFADesign,
}

impl NFASimulation {
    pub fn new(nfa_design: NFADesign) -> Self {
        NFASimulation {
            nfa_design: nfa_design,
        }
    }
    fn next_state(&self, states: Vec<State>, character: char) -> HashSet<State> {
        let mut nfa = self.nfa_design.to_nfa_simulation(states);
        nfa.read_character(character);
        nfa.get_current_states()
    }
    fn rules_for(&self, states: Vec<State>) -> Vec<FARule<Vec<State>>> {
        self.nfa_design
            .rulebook()
            .alphabet()
            .iter()
            .map(|&c| {
                FARule::new((
                    states.clone(),
                    c,
                    self.next_state(states.clone(), c).iter().cloned().collect(),
                ))
            })
            .collect()
    }
    pub fn discover_states_and_rules(
        &self,
        states: HashSet<Vec<State>>,
    ) -> (HashSet<Vec<State>>, Vec<FARule<Vec<State>>>) {
        let mut sorted_states = HashSet::new();
        for mut s in states {
            s.sort();
            sorted_states.insert(s);
        }
        let states = sorted_states;
        let rules = states
            .iter()
            .flat_map(|s| self.rules_for(s.iter().cloned().collect()))
            .collect::<Vec<_>>();
        let more_states = rules
            .iter()
            .map(|r| {
                let mut s = r.follow().clone();
                s.sort();
                s
            })
            .collect::<HashSet<_>>();
        if more_states.is_subset(&states) {
            (states, rules)
        } else {
            self.discover_states_and_rules(states.union(&more_states).cloned().collect())
        }
    }
    //fn to_dfa_design(&self) -> DFADesign {
    //    let start_state = self
    //        .nfa_design
    //        .to_nfa()
    //        .get_current_states()
    //        .iter()
    //        .cloned()
    //        .collect::<Vec<_>>();
    //    let mut state = HashSet::new();
    //    state.insert(start_state);
    //    let (states, rules) = self.discover_states_and_rules(state);
    //    let accept_states = states
    //        .iter()
    //        .filter(|s| self.nfa_design.to_nfa_simulation(s.to_vec()).is_accept())
    //        .cloned()
    //        .collect();
    //    DFADesign::new((start_state, accept_states, DFARulebook::new(rules)))
    //}
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

    #[test]
    fn to_nfa_simulation_test() {
        let rulebook = NFARulebook::build(vec![
            (1, 'a', 1),
            (1, 'a', 2),
            (1, FREE_MOVE, 2),
            (2, 'b', 3),
            (3, 'b', 1),
            (3, FREE_MOVE, 2),
        ]);
        let nfa_design = NFADesign::new((1, vec![3].iter().cloned().collect(), rulebook));
        assert_eq!(
            nfa_design.to_nfa().get_current_states(),
            vec![1, 2].iter().cloned().collect()
        );
        assert_eq!(
            nfa_design.to_nfa_simulation(vec![2]).get_current_states(),
            vec![2].iter().cloned().collect()
        );
        assert_eq!(
            nfa_design.to_nfa_simulation(vec![3]).get_current_states(),
            vec![2, 3].iter().cloned().collect()
        );
        let mut nfa = nfa_design.to_nfa_simulation(vec![2, 3]);
        nfa.read_character('b');
        assert_eq!(
            nfa.get_current_states(),
            vec![1, 2, 3].iter().cloned().collect()
        );
    }

    #[test]
    fn nfa_simulation_next_state_test() {
        let rulebook = NFARulebook::build(vec![
            (1, 'a', 1),
            (1, 'a', 2),
            (1, FREE_MOVE, 2),
            (2, 'b', 3),
            (3, 'b', 1),
            (3, FREE_MOVE, 2),
        ]);
        let nfa_design = NFADesign::new((1, vec![3].iter().cloned().collect(), rulebook));
        let sim = NFASimulation::new(nfa_design);
        assert_eq!(
            sim.next_state(vec![1, 2], 'a'),
            vec![1, 2].iter().cloned().collect()
        );
    }
}
