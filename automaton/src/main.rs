use automaton::dfa::{DFADesign, DFARulebook};
use automaton::fa_rule::FARule;
use automaton::nfa::{NFADesign, NFARulebook, FREE_MOVE};

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

    println!("-- free move --");
    let rulebook = NFARulebook {
        rules: vec![
            FARule::new((1, FREE_MOVE, 2)),
            FARule::new((1, FREE_MOVE, 4)),
            FARule::new((2, 'a', 3)),
            FARule::new((3, 'a', 2)),
            FARule::new((4, 'a', 5)),
            FARule::new((5, 'a', 6)),
            FARule::new((6, 'a', 4)),
        ],
    };
    let nfa_design = NFADesign::new((1, vec![2, 4].into_iter().collect(), rulebook));
    println!("{}", nfa_design.is_accept("aa"));
    println!("{}", nfa_design.is_accept("aaa"));
    println!("{}", nfa_design.is_accept("aaaaa"));
    println!("{}", nfa_design.is_accept("aaaaaa"));
}
