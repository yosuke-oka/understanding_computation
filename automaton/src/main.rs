use automaton::nfa::*;

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
    let sim = NFASimulation::new(nfa_design);
    println!("{:?}", sim.rules_for(vec![1, 2]));
    println!("{:?}", sim.rules_for(vec![3, 2]));
}
