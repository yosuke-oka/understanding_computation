use automaton::stack::Stack;
fn main() {
    let stack = Stack::new(vec!['a', 'b', 'c', 'd', 'e']);
    println!("{}", stack);
    println!("{:?}", stack.top());
    println!("{:?}", stack.pop().pop().top());
    println!("{:?}", stack.push('x').push('y').top());
    println!("{:?}", stack.push('x').push('y').pop().top());
}
