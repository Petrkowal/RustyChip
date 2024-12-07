use crate::chip8::datatypes::datatypes::*;


const STACK_SIZE: usize = 16;

pub struct Stack {
    stack: [Address; STACK_SIZE],
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [Address(0); STACK_SIZE],
        }
    }

    pub fn push(&mut self, address: Address) {
        let index = self.stack.iter().position(|&x| x == Address(0)).unwrap();
        self.stack[index] = address;
    }

    pub fn pop(&mut self) -> Option<Address> {
        let index = self.stack.iter().rposition(|&x| x != Address(0))?;
        let address = self.stack[index];
        self.stack[index] = Address(0);
        Some(address)
    }
}