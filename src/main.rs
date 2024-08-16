use std::{collections::HashMap, fmt, ops::Div};
const MAXIMUM_STACK_SIZE: usize = 1024;
// Stack
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

// Memory
struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    fn new() -> Self {
        Self { memory: vec![] }
    }

    fn access(&self, offset: usize, size: usize) -> Vec<u8> {
        self.memory[offset..(offset + size)].to_vec()
    }

    fn load(&self, offset: usize) -> Vec<u8> {
        self.memory[offset..32].to_vec()
    }

    fn store(&mut self, offset: usize, values: Vec<u8>) {
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

// Storage
struct KeyValue {
    storage: HashMap<u8, u8>,
}

impl KeyValue {
    fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    fn load(&self, key: u8) -> Option<&u8> {
        if let Some(value) = self.storage.get(&key) {
            return Some(value);
        } else {
            None
        }
    }

    fn store(&mut self, key: u8, value: u8) {
        self.storage.insert(key, value);
    }
}

struct Storage {
    storage: KeyValue,
    cache: Vec<u8>,
}

impl Storage {
    fn new() -> Self {
        Self {
            storage: KeyValue::new(),
            cache: Vec::new(),
        }
    }

    fn load(&mut self, key: u8) -> (bool, u8) {
        let warm = self.cache.contains(&key);
        if !warm {
            self.cache.push(key)
        }
        match self.storage.load(key) {
            Some(value) => (warm, *value),
            None => (warm, 0x00),
        }
    }

    fn store(&mut self, key: u8, value: u8) {
        &self.storage.store(key, value);
    }
}

// EVM State
struct EvmState {
    pc: i32,
    stack: Stack,
    memory: Memory,
    storage: Storage,
    sender: String,
    program: String,
    gas: i32,
    value: i32,
    calldata: i32,
    stop_flag: bool,
    revert_flag: bool,
    return_data: Vec<i32>,
    logs: Vec<String>,
}

impl EvmState {
    fn gas_dec(&mut self, value: i32) {
        self.gas -= value;
    }
}

// OPCODES

fn stop(evm: &mut EvmState) -> &mut EvmState {
    evm.stop_flag = true;
    evm
}

fn main() {}
