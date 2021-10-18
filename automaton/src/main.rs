use automaton::dpda::*;
use automaton::pda_configuration::*;
fn main() {
    let rulebook = DPDARulebook::build(vec![
        (1, '(', 2, '$', vec!['b', '$']),
        (2, '(', 2, 'b', vec!['b', 'b']),
        (2, ')', 2, 'b', vec![]),
        (2, '0', 1, '$', vec!['$']),
    ]);
}
