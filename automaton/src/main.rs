use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;
type State = u32;

#[derive(Clone)]
struct FARule {
    state: State,
    character: char,
    next_state: State,
}

impl FARule {
    fn new(arg: (State, char, State)) -> FARule {
        FARule {
            state: arg.0,
            character: arg.1,
            next_state: arg.2,
        }
    }
    fn is_applied_to(&self, state: State, character: char) -> bool {
        self.state == state && self.character == character
    }
    fn follow(&self) -> State {
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

#[derive(Clone)]
struct DFARulebook {
    rules: Vec<FARule>,
}

impl DFARulebook {
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

struct DFADesign {
    current_state: State,
    accept_states: HashSet<State>,
    rulebook: DFARulebook,
}

impl DFADesign {
    fn new(arg: (State, HashSet<State>, DFARulebook)) -> DFADesign {
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
    fn is_accept(&self, string: &str) -> bool {
        let mut dfa = self.to_dfa();
        dfa.read_string(string);
        dfa.is_accept()
    }
}

#[derive(Clone)]
struct NFARulebook {
    rules: Vec<FARule>,
}

impl NFARulebook {
    fn next_states(&self, states: HashSet<State>, character: char) -> HashSet<State> {
        HashSet::from_iter(
            states
                .into_iter()
                .flat_map(|state| self.follow_rules_for(state, character)),
        )
    }
    fn follow_rules_for(&self, state: State, character: char) -> HashSet<State> {
        self.rules_for(state, character)
            .iter()
            .map(|r| r.follow())
            .collect()
    }
    fn rules_for(&self, state: State, character: char) -> Vec<FARule> {
        self.rules
            .iter()
            .filter(|r| r.is_applied_to(state, character))
            .cloned()
            .collect()
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
            .current_states
            .intersection(&self.accept_states)
            .collect::<HashSet<_>>()
            .is_empty()
    }
    fn read_character(&mut self, character: char) -> () {
        self.current_states = self
            .rulebook
            .next_states(self.current_states.clone(), character)
    }
    fn read_string(&mut self, string: &str) -> () {
        for c in string.chars() {
            self.read_character(c);
        }
    }
}

struct NFADesign {
    start_state: State,
    accept_states: HashSet<State>,
    rulebook: NFARulebook,
}

impl NFADesign {
    fn new(arg: (State, HashSet<State>, NFARulebook)) -> NFADesign {
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
    fn is_accept(&self, string: &str) -> bool {
        let mut nfa = self.to_nfa();
        nfa.read_string(string);
        nfa.is_accept()
    }
}

fn main() {
    let rulebook = DFARulebook {
        rules: vec![
            FARule::new((1, 'a', 2)),
            FARule::new((1, 'b', 1)),
            FARule::new((2, 'a', 2)),
            FARule::new((2, 'b', 3)),
            FARule::new((3, 'a', 3)),
            FARule::new((3, 'b', 3)),
        ],
    };

    let accept_states = vec![3].into_iter().collect();
    let dfa_design = DFADesign::new((1, accept_states, rulebook));
    println!("{}", dfa_design.is_accept("a"));
    println!("{}", dfa_design.is_accept("baa"));
    println!("{}", dfa_design.is_accept("baba"));

    println!("-- nfa --");

    let rulebook = NFARulebook {
        rules: vec![
            FARule::new((1, 'a', 1)),
            FARule::new((1, 'b', 1)),
            FARule::new((1, 'b', 2)),
            FARule::new((2, 'a', 3)),
            FARule::new((2, 'b', 3)),
            FARule::new((3, 'a', 4)),
            FARule::new((3, 'b', 4)),
        ],
    };
    let nfa_design = NFADesign::new((1, vec![4].into_iter().collect(), rulebook));
    println!("{}", nfa_design.is_accept("bab"));
    println!("{}", nfa_design.is_accept("bbbbb"));
    println!("{}", nfa_design.is_accept("bbabb"));
}
