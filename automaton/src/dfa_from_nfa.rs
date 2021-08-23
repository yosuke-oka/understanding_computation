use crate::fa_rule::FARule;
use std::collections::BTreeSet;

// あまりにもDFAとは型を変換しないと行けないので別ファイルで作成
// ちゃんとやるならStateをジェネリクスにするとかが必要かも？
type State = BTreeSet<u32>;

#[derive(Clone)]
pub struct DFARulebook {
    pub rules: Vec<FARule<State>>,
}

impl DFARulebook {
    pub fn new(rules: Vec<FARule<State>>) -> Self {
        DFARulebook { rules: rules }
    }
    fn next_state(&self, state: State, character: char) -> State {
        self.rule_for(state.clone(), character).follow().clone()
    }
    fn rule_for(&self, state: State, character: char) -> &FARule<State> {
        self.rules
            .iter()
            .find(|r| r.is_applied_to(state.clone(), character))
            .unwrap()
    }
}

struct DFA {
    current_state: State,
    accept_states: BTreeSet<State>,
    rulebook: DFARulebook,
}

impl DFA {
    fn is_accept(&self) -> bool {
        self.accept_states.contains(&self.current_state)
    }
    fn read_character(&mut self, character: char) -> () {
        self.current_state = self
            .rulebook
            .next_state(self.current_state.clone(), character);
    }
    fn read_string(&mut self, string: &str) -> () {
        for c in string.chars() {
            self.read_character(c);
        }
    }
}

pub struct DFADesign {
    current_state: State,
    accept_states: BTreeSet<State>,
    rulebook: DFARulebook,
}

impl DFADesign {
    pub fn new(arg: (State, BTreeSet<State>, DFARulebook)) -> DFADesign {
        DFADesign {
            current_state: arg.0,
            accept_states: arg.1,
            rulebook: arg.2,
        }
    }
    fn to_dfa(&self) -> DFA {
        DFA {
            current_state: self.current_state.clone(),
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
