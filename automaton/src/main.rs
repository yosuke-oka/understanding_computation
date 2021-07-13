use automaton::dfa::{DFADesign, DFARulebook};
use automaton::nfa::{NFADesign, NFARulebook, FREE_MOVE};
use automaton::pattern::Pattern;

fn main() {
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
    println!("{}", dfa_design.is_accept("a"));
    println!("{}", dfa_design.is_accept("baa"));
    println!("{}", dfa_design.is_accept("baba"));

    println!("-- nfa --");

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
    println!("{}", nfa_design.is_accept("bab"));
    println!("{}", nfa_design.is_accept("bbbbb"));
    println!("{}", nfa_design.is_accept("bbabb"));

    println!("-- free move --");
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
    println!("{}", nfa_design.is_accept("aa"));
    println!("{}", nfa_design.is_accept("aaa"));
    println!("{}", nfa_design.is_accept("aaaaa"));
    println!("{}", nfa_design.is_accept("aaaaaa"));

    println!("-- regex --");
    let pattern = Pattern::Repeat(Box::new(Pattern::Choose {
        first: Box::new(Pattern::Concatnate {
            first: Box::new(Pattern::Literal('a')),
            second: Box::new(Pattern::Literal('b')),
        }),
        second: Box::new(Pattern::Literal('a')),
    }));
    println!("{}", pattern);

    println!("{}", Pattern::Empty.is_match(""));
    println!("{}", Pattern::Empty.is_match("a"));

    let pattern = Pattern::Concatnate {
        first: Box::new(Pattern::Literal('a')),
        second: Box::new(Pattern::Literal('b')),
    };
    println!("/{}/", pattern);
    println!("{}", pattern.is_match("ab"));
    println!("{}", pattern.is_match("aa"));

    let pattern = Pattern::Choose {
        first: Box::new(Pattern::Literal('a')),
        second: Box::new(Pattern::Literal('b')),
    };
    println!("/{}/", pattern);
    println!("{}", pattern.is_match("a"));
    println!("{}", pattern.is_match("b"));
    println!("{}", pattern.is_match("c"));

    let pattern = Pattern::Repeat(Box::new(Pattern::Literal('a')));
    println!("/{}/", pattern);
    println!("{}", pattern.is_match(""));
    println!("{}", pattern.is_match("a"));
    println!("{}", pattern.is_match("aaa"));
    println!("{}", pattern.is_match("ab"));
}
