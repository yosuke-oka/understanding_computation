use automaton::pattern::Pattern;
use automaton::pattern_parser::parse;

fn main() {
    let pattern = Pattern::Repeat(Box::new(Pattern::Literal('a')));
    println!("/{}/", pattern);
    println!("{}", pattern.is_match(""));
    println!("{}", pattern.is_match("a"));
    println!("{}", pattern.is_match("aaa"));
    println!("{}", pattern.is_match("ab"));

    println!("--------");
    println!("{:?}", parse("abbb"));
    let parsed = parse("(a(|b))*");

    println!("{:?}", parsed);
    println!("{}", parsed.is_match("abaab"));
    println!("{}", parsed.is_match("abba"));
}
