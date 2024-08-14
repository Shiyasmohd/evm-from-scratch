use std::fmt;

const MAXIMUM_STACK_SIZE: usize = 1024;

#[derive(Debug)]
struct Stack {
    items: Vec<i32>,
}

impl Stack {
    fn new() -> Stack {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, value: i32) -> Result<(), &'static str> {
        if self.items.len() == MAXIMUM_STACK_SIZE {
            Err("Stack Overflow")
        } else {
            self.items.push(value);
            Ok(())
        }
    }

    fn pop(&mut self) -> Result<i32, &'static str> {
        if self.items.is_empty() {
            Err("Stack Underflow")
        } else {
            Ok(self.items.pop().unwrap())
        }
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("Working");
        let mut display_arr: Vec<String> = vec![];
        for i in 0..self.items.len() {
            if i == 0 {
                display_arr.push(format!("{}<first", self.items[i]));
            } else if i == self.items.len() - 1 {
                display_arr.push(format!("{}<last", self.items[i]));
            } else {
                display_arr.push(format!("{}", self.items[i]));
            }
        }
        write!(f, "{}", display_arr.join("\n"))
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("{}", stack);
    stack.pop();
    println!("{}", stack);
}
