use automaton::pattern::Pattern;

fn main() {
    let pattern = Pattern::Repeat(Box::new(Pattern::Literal('a')));
    println!("/{}/", pattern);
    println!("{}", pattern.is_match(""));
    println!("{}", pattern.is_match("a"));
    println!("{}", pattern.is_match("aaa"));
    println!("{}", pattern.is_match("ab"));
}
