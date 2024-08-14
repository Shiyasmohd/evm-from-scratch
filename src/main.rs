use std::{fmt, mem};

const MAXIMUM_STACK_SIZE: usize = 1024;

#[derive(Debug)]
struct Stack {
    items: Vec<i32>,
}

struct SimpleMemory {
    memory: Vec<i32>,
}

impl SimpleMemory {
    fn new() -> Self {
        Self { memory: vec![] }
    }

    fn access(&self, offset: usize, size: usize) -> Vec<i32> {
        self.memory[offset..(offset + size)].to_vec()
    }

    fn load(&self, offset: usize) -> Vec<i32> {
        self.memory[offset..32].to_vec()
    }

    fn store(&mut self, offset: usize, values: Vec<i32>) {
        let mut memory_expansion_cost: usize;

        if self.memory.len() <= offset + values.len() {
            let mut expansion_size: usize = 0;

            // initialize memory with 32 zeros if it is empty
            if self.memory.len() == 0 {
                expansion_size = 32;
                self.memory = vec![0x00; 32];
            }

            // extend more memory if needed
            if self.memory.len() < offset + values.len() {
                expansion_size = offset + values.len() - self.memory.len();
                self.memory.resize(expansion_size, 0x00);
            }
            memory_expansion_cost = expansion_size ^ 2;
        }

        self.memory[offset..offset + values.len()].copy_from_slice(&values);
    }
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
    let mut memory = SimpleMemory::new();
    memory.store(0, vec![0x01, 0x02, 0x03, 0x04]);
    println!("{:?}", memory.load(0));
}
