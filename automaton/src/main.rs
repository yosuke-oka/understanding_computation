use automaton::nfa::*;
use std::collections::HashSet;

fn main() {
    let rulebook = NFARulebook::build(vec![
        (1, 'a', 1),
        (1, 'a', 2),
        (1, FREE_MOVE, 2),
        (2, 'b', 3),
        (3, 'b', 1),
        (3, FREE_MOVE, 2),
    ]);
    let nfa_design = NFADesign::new((1, vec![3].iter().cloned().collect(), rulebook));
    let start_state = nfa_design
        .to_nfa()
        .get_current_states()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    let sim = NFASimulation::new(nfa_design);
    let mut state = HashSet::new();
    state.insert(start_state);
    println!("{:?}", sim.discover_states_and_rules(state));
}
