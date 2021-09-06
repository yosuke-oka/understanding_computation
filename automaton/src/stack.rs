use std::fmt;

pub struct Stack {
    contents: Vec<char>,
}

impl Stack {
    pub fn new(args: Vec<char>) -> Self {
        Stack { contents: args }
    }
    pub fn push(&self, c: char) -> Self {
        let mut v = self.contents.clone();
        v.insert(0, c);
        Stack::new(v)
    }
    pub fn pop(&self) -> Self {
        let mut v = self.contents.clone();
        v.remove(0);
        Stack::new(v)
    }
    pub fn top(&self) -> Option<&char> {
        self.contents.first()
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.top() {
            None => write!(f, "<Stack ()>"),
            Some(&c) => write!(
                f,
                "<Stack ({}){}>",
                c,
                self.contents[1..].iter().collect::<String>()
            ),
        }
    }
}
impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.top() {
            None => write!(f, "<Stack ()>"),
            Some(&c) => write!(
                f,
                "<Stack ({}){}>",
                c,
                self.contents[1..].iter().collect::<String>()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_test() {
        let stack = Stack::new(vec!['a', 'b', 'c', 'd', 'e']);
        assert_eq!(stack.top(), Some(&'a'));
        assert_eq!(stack.pop().pop().top(), Some(&'c'));
        assert_eq!(stack.push('x').push('y').top(), Some(&'y'));
        assert_eq!(stack.push('x').push('y').pop().top(), Some(&'x'));
    }
}
