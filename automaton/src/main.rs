use automaton::pda_configuration::*;
use automaton::stack::Stack;
fn main() {
    let rule = PDARule::new(1, '(', 2, '$', vec!['b', '$']);
    println!("{:?}", rule);
    let configuration = PDAConfiguration::new(1, Stack::new(vec!['$']));
    println!("{:?}", configuration);
    //println!("{}", rule.is_applied_to(configuration, '('));
    println!("{:?}", rule.follow(configuration));
}
