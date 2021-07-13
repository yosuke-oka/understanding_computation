use automaton::pattern::Pattern;
use automaton::pattern_parser::choose_parser;

fn main() {
    let pattern = Pattern::Repeat(Box::new(Pattern::Literal('a')));
    println!("/{}/", pattern);
    println!("{}", pattern.is_match(""));
    println!("{}", pattern.is_match("a"));
    println!("{}", pattern.is_match("aaa"));
    println!("{}", pattern.is_match("ab"));

    println!("--------");
    println!("{:?}", choose_parser("abbb"));
    let parsed = choose_parser("(a(|b))*");

    println!("{:?}", parsed);
    println!("{:?}", parsed.unwrap().1.to_string());
}
