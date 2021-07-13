use crate::fa_rule::FARule;
use crate::nfa::NFADesign;
use crate::nfa::NFARulebook;
use crate::nfa::FREE_MOVE;
use crate::state::new_state;
use std::collections::HashSet;
use std::fmt;

pub enum Pattern {
    Empty,
    Literal(char),
    Concatnate {
        first: Box<Pattern>,
        second: Box<Pattern>,
    },
    Choose {
        first: Box<Pattern>,
        second: Box<Pattern>,
    },
    Repeat(Box<Pattern>),
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pattern::Empty => write!(f, ""),
            Pattern::Literal(c) => write!(f, "{}", c),
            Pattern::Concatnate { first, second } => write!(
                f,
                "{}{}",
                first.bracket(self.precedence()),
                second.bracket(self.precedence())
            ),
            Pattern::Choose { first, second } => write!(
                f,
                "{}|{}",
                first.bracket(self.precedence()),
                second.bracket(self.precedence())
            ),
            Pattern::Repeat(p) => write!(f, "{}*", p.bracket(self.precedence())),
        }
    }
}

impl Pattern {
    fn bracket(&self, outer_precedence: u32) -> String {
        if self.precedence() < outer_precedence {
            format!("({})", self)
        } else {
            self.to_string()
        }
    }
    pub fn is_match(&self, str: &str) -> bool {
        self.to_nfa_design().is_accept(str)
    }
    pub fn precedence(&self) -> u32 {
        match self {
            Pattern::Empty => 3,
            Pattern::Literal(_) => 3,
            Pattern::Concatnate {
                first: _,
                second: _,
            } => 1,
            Pattern::Choose {
                first: _,
                second: _,
            } => 0,
            Pattern::Repeat(_) => 2,
        }
    }
    fn to_nfa_design(&self) -> NFADesign {
        match self {
            Pattern::Empty => {
                let start_state = new_state();
                NFADesign::new((
                    start_state,
                    vec![start_state].iter().cloned().collect(),
                    NFARulebook::build(vec![]),
                ))
            }
            Pattern::Literal(c) => {
                let start_state = new_state();
                let accept_state = new_state();
                let rulebook = NFARulebook::build(vec![(start_state, *c, accept_state)]);
                NFADesign::new((
                    start_state,
                    vec![accept_state].iter().cloned().collect(),
                    rulebook,
                ))
            }
            Pattern::Concatnate { first, second } => {
                let first_nfa_design = first.to_nfa_design();
                let second_nfa_design = second.to_nfa_design();
                let mut rules = Vec::new();
                // concat first + second rules
                rules.append(&mut first_nfa_design.rulebook().rules().clone());
                rules.append(&mut second_nfa_design.rulebook().rules().clone());
                // first_accept -> free_move -> second_start
                for s in first_nfa_design.accept_states() {
                    rules.push(FARule::new((
                        *s,
                        FREE_MOVE,
                        second_nfa_design.start_state().clone(),
                    )))
                }
                NFADesign::new((
                    *first_nfa_design.start_state(),
                    second_nfa_design.accept_states().clone(),
                    NFARulebook { rules: rules },
                ))
            }
            Pattern::Choose { first, second } => {
                let first_nfa_design = first.to_nfa_design();
                let second_nfa_design = second.to_nfa_design();
                let start_state = new_state();
                let accept_states = first_nfa_design
                    .accept_states()
                    .union(second_nfa_design.accept_states())
                    .cloned()
                    .collect::<HashSet<_>>();
                let mut rules = Vec::new();
                // concat first + second rules
                rules.append(&mut first_nfa_design.rulebook().rules().clone());
                rules.append(&mut second_nfa_design.rulebook().rules().clone());
                // start_state -> free_move -> first_start
                rules.push(FARule::new((
                    start_state,
                    FREE_MOVE,
                    first_nfa_design.start_state().clone(),
                )));
                // start_state -> free_move -> second_start
                rules.push(FARule::new((
                    start_state,
                    FREE_MOVE,
                    second_nfa_design.start_state().clone(),
                )));
                NFADesign::new((start_state, accept_states, NFARulebook { rules: rules }))
            }
            Pattern::Repeat(pattern) => {
                let pattern_nfa_design = pattern.to_nfa_design();
                let start_state = new_state();
                let mut accept_states = pattern_nfa_design.accept_states().clone();
                accept_states.insert(start_state);
                let mut rules = pattern_nfa_design.rulebook().rules().clone();
                rules.push(FARule::new((
                    start_state,
                    FREE_MOVE,
                    pattern_nfa_design.start_state().clone(),
                )));
                for state in pattern_nfa_design.accept_states() {
                    rules.push(FARule::new((
                        *state,
                        FREE_MOVE,
                        pattern_nfa_design.start_state().clone(),
                    )))
                }
                NFADesign::new((start_state, accept_states, NFARulebook { rules: rules }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_test() {
        let pattern = Pattern::Repeat(Box::new(Pattern::Choose {
            first: Box::new(Pattern::Concatnate {
                first: Box::new(Pattern::Literal('a')),
                second: Box::new(Pattern::Literal('b')),
            }),
            second: Box::new(Pattern::Literal('a')),
        }));

        assert_eq!(pattern.to_string(), "(ab|a)*");
    }

    #[test]
    fn empty_to_nfa_design() {
        assert!(Pattern::Empty.is_match(""));
        assert!(!Pattern::Empty.is_match("a"));
    }

    #[test]
    fn concatnate_to_nfa_design() {
        let pattern = Pattern::Concatnate {
            first: Box::new(Pattern::Literal('a')),
            second: Box::new(Pattern::Literal('b')),
        };

        assert!(pattern.is_match("ab"));
        assert!(!pattern.is_match("aa"));
    }

    #[test]
    fn choose_to_nfa_design() {
        let pattern = Pattern::Choose {
            first: Box::new(Pattern::Literal('a')),
            second: Box::new(Pattern::Literal('b')),
        };

        assert!(pattern.is_match("a"));
        assert!(pattern.is_match("b"));
        assert!(!pattern.is_match("c"));
    }

    #[test]
    fn repeat_to_nfa_design() {
        let pattern = Pattern::Repeat(Box::new(Pattern::Literal('a')));

        assert!(pattern.is_match(""));
        assert!(pattern.is_match("a"));
        assert!(pattern.is_match("aaaa"));
        assert!(!pattern.is_match("ab"));
    }
}
