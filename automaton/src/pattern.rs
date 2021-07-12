use crate::fa_rule::FARule;
use crate::nfa::NFADesign;
use crate::nfa::NFARulebook;
use crate::nfa::FREE_MOVE;
use crate::state::new_state;
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
                NFADesign::new((start_state, vec![start_state], NFARulebook::build(vec![])))
            }
            Pattern::Literal(c) => {
                let start_state = new_state();
                let accept_state = new_state();
                let rulebook = NFARulebook::build(vec![(start_state, *c, accept_state)]);
                NFADesign::new((start_state, vec![accept_state], rulebook))
            }
            Pattern::Concatnate { first, second } => {
                let mut first_nfa_design = first.to_nfa_design();
                let mut second_nfa_design = second.to_nfa_design();
                let mut extra_rules = first_nfa_design
                    .accept_states()
                    .iter()
                    .map(|s| FARule::new((*s, FREE_MOVE, second_nfa_design.start_state().clone())))
                    .collect::<Vec<_>>();
                let mut rules = Vec::new();
                let first_rules = first_nfa_design.rulebook_mut().rules_mut();
                let second_rules = second_nfa_design.rulebook_mut().rules_mut();
                rules.append(first_rules);
                rules.append(second_rules);
                rules.append(&mut extra_rules);
                //let rulebook = first_nfa_design
                //    .rulebook()
                //    .rules_mut()
                //    .append(second_nfa_design.rulebook().rules_mut());
                NFADesign::new((
                    *first_nfa_design.start_state(),
                    second_nfa_design.accept_states().iter().cloned().collect(),
                    NFARulebook { rules: rules },
                ))
            }
            _ => unreachable!(),
        }
    }
}
