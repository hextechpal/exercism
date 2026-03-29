mod stack {
    use std::fmt::Error;

    pub struct Stack<T: Ord> {
        data: Vec<T>,
        top: usize,
        size: usize,
    }

    impl<T: Ord> Stack<T> {
        pub fn new(size: usize) -> Self {
            Self {
                data: Vec::with_capacity(size),
                top: 0,
                size: size,
            }
        }

        pub fn push(&mut self, el: T) -> Result<(), Error> {
            if self.is_full() {
                return Err(Error);
            }
            self.data.push(el);
            self.top += 1;
            return Ok(());
        }

        pub fn pop(&mut self) -> Result<T, Error> {
            if self.is_empty() {
                return Err(Error);
            }

            self.top -= 1;
            self.data.pop().ok_or(Error)
        }

        pub fn peek(&self) -> Result<&T, Error> {
            if self.is_empty() {
                return Err(Error);
            }

            Ok(&self.data[self.top - 1])
        }

        pub fn is_full(&self) -> bool {
            self.top == self.size
        }

        pub fn is_empty(&self) -> bool {
            self.top == 0
        }
    }
}

use std::fmt::Error;

use stack::Stack;



pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Stack::new(string.len());

    for b in string.as_bytes(){

        if *b == b'{' || *b == b'[' || *b == b'('{
            let _ = stack.push(b);  
        }
        
        if *b == b'}'{
            if let Some(value) = check_compiments(&mut stack, b'{') {
                return value;
            }
        }

        if *b == b']'{
            if let Some(value) = check_compiments(&mut stack, b'[') {
                return value;
            }
        }

        if *b == b')'{
            if let Some(value) = check_compiments(&mut stack, b'(') {
                return value;
            }
        }
    }

    stack.is_empty()
}

fn check_compiments(stack: &mut Stack<&u8>, check: u8) -> Option<bool> {
    let r = stack.pop();
    match r {
        Err(Error) => return Some(false),
        Ok(top) => {
            if *top != check {
                return Some(false);
            }
        }
    }
    None
}
