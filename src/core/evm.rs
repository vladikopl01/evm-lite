use alloy_primitives::U256;

use super::memory::Memory;
use super::opcode::Opcode;
use super::stack::Stack;

pub struct Evm {
    pub stack: Stack,
    pc: usize,
    code: Vec<u8>,
    memory: Memory,
}

impl Evm {
    pub fn new(code: Vec<u8>) -> Self {
        Evm {
            code,
            pc: 0,
            stack: Stack::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.code.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        let (opcode, consumed_bytes) = Opcode::from_byte(&self.code, self.pc);
        self.pc += consumed_bytes;

        match opcode {
            Opcode::STOP => self.pc = self.code.len(),
            Opcode::ADD => self.bin_op(|a, b| a + b),
            Opcode::MUL => self.bin_op(|a, b| a * b),
            Opcode::SUB => self.bin_op(|a, b| a - b),
            Opcode::MLOAD => {
                let offset = self.stack.pop().try_into().expect("offset too large");
                let data = self.memory.mload(offset);
                self.stack.push(data);
            }
            Opcode::MSTORE => {
                let offset = self.stack.pop().try_into().expect("offset too large");
                let data = self.stack.pop();
                self.memory.mstore(offset, data);
            }
            Opcode::JUMP => {
                let dest = self.stack.pop().try_into().expect("dest too large");
                match self.code.get(dest) {
                    Some(&0x5B) => self.pc = dest,
                    _ => panic!("Invalid jump destination"),
                }
            }
            Opcode::JUMPI => {
                let cond = self.stack.pop();
                let dest = self.stack.pop().try_into().expect("dest too large");

                if cond != U256::from(0) {
                    match self.code.get(dest) {
                        Some(&0x5B) => self.pc = dest,
                        _ => panic!("Invalid jump destination"),
                    }
                } else {
                    self.pc += 1;
                }
            }
            Opcode::JUMPDEST => {}
            Opcode::PUSHn(n) => {
                let bytes = &self.code[self.pc - n..self.pc];
                self.stack.push(U256::from_be_slice(bytes));
            }
            Opcode::DUPn(n) => {
                let n = n.try_into().expect("dest too large");
                self.stack.push(self.stack.peek(n));
            }
            Opcode::SWAPn(n) => self.stack.swap(n.try_into().expect("dest too large")),
            Opcode::Unknown(byte) => panic!("Unknown opcode: {:?}", byte),
        }
    }

    fn bin_op<F>(&mut self, func: F)
    where
        F: Fn(U256, U256) -> U256,
    {
        let a = self.stack.pop();
        let b = self.stack.pop();
        self.stack.push(func(b, a));
    }
}

#[test]
fn test_stop() {
    let code: Vec<u8> = vec![0x00];
    let mut evm = Evm::new(code);
    evm.run();
    assert_eq!(evm.stack, Stack::new());
}

#[test]
fn test_add() {
    let code: Vec<u8> = vec![0x60, 0x01, 0x60, 0x02, 0x01, 0x00];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(3));

    assert_eq!(evm.stack, expected);
}

#[test]
fn test_mul() {
    let code: Vec<u8> = vec![0x60, 0x02, 0x60, 0x03, 0x02, 0x00];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(6));
    assert_eq!(evm.stack, expected);
}

#[test]
fn test_sub() {
    let code: Vec<u8> = vec![0x60, 0x06, 0x60, 0x01, 0x03, 0x00];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(5));
    assert_eq!(evm.stack, expected);
}

#[test]
fn test_mstore_mload() {
    let code: Vec<u8> = vec![
        0x7f, // PUSH32
        // 0x000000...000001 (U256::from(1))
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, // 32 bytes
        0x60, 0x00, // PUSH1 0x00 - offset
        0x52, // MSTORE
        0x60, 0x00, // PUSH1 0x00 - offset
        0x51, // MLOAD
        0x00, // STOP
    ];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(1));
    assert_eq!(evm.stack, expected);
}

#[test]
fn test_mload_uninitialized() {
    let code: Vec<u8> = vec![
        0x60, 0x40, // PUSH1 0x40
        0x51, // MLOAD (memory[64])
        0x00,
    ];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(0));
    assert_eq!(evm.stack, expected);
}

#[test]
fn test_push() {
    let code: Vec<u8> = vec![0x60, 0x01, 0x61, 0x01, 0x00, 0x62, 0x01, 0x00, 0x00, 0x00];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(1));
    expected.push(U256::from(256));
    expected.push(U256::from(65536));
    assert_eq!(evm.stack, expected);
}

#[test]
fn test_jump() {
    let code: Vec<u8> = vec![
        0x60, 0x01, // PUSH1 1
        0x60, 0x02, // PUSH1 2
        0x60, 0x09, // PUSH1 9 - JUMPDEST index
        0x56, // JUMP
        0x60, 0x09, // PUSH1 9 - passed
        0x5B, // JUMPDEST
        0x01, // ADD
        0x00, // STOP
    ];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(3));
    assert_eq!(evm.stack, expected);
}

#[test]
fn test_jumpi_false() {
    let code: Vec<u8> = vec![
        0x60, 0x01, // PUSH1 1
        0x60, 0x02, // PUSH1 2
        0x60, 0x0B, // PUSH1 11 - JUMPDEST index
        0x60, 0x00, // PUSH1 0 - JUMPI condition
        0x57, // JUMPI
        0x60, 0x09, // PUSH1 9 - not passed as no jump
        0x5B, // JUMPDEST
        0x01, // ADD
        0x00, // STOP
    ];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(1));
    expected.push(U256::from(11));
    assert_eq!(evm.stack, expected);
}

#[test]
fn test_jumpi_true() {
    let code: Vec<u8> = vec![
        0x60, 0x01, // PUSH1 1
        0x60, 0x02, // PUSH1 2
        0x60, 0x0B, // PUSH1 11 - JUMPDEST index
        0x60, 0x01, // PUSH1 1 - JUMPI condition
        0x57, // JUMPI
        0x60, 0x09, // PUSH1 9 - passed as no jump
        0x5B, // JUMPDEST
        0x01, // ADD
        0x00, // STOP
    ];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(3));
    assert_eq!(evm.stack, expected);
}

#[test]
fn test_dup() {
    let code: Vec<u8> = vec![0x60, 0x01, 0x60, 0x02, 0x60, 0x03, 0x81, 0x00];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(1));
    expected.push(U256::from(2));
    expected.push(U256::from(3));
    expected.push(U256::from(2));
    assert_eq!(evm.stack, expected);
}

#[test]
fn test_swap() {
    let code: Vec<u8> = vec![0x60, 0x01, 0x60, 0x02, 0x60, 0x03, 0x91, 0x00];
    let mut evm = Evm::new(code);
    evm.run();

    let mut expected = Stack::new();
    expected.push(U256::from(3));
    expected.push(U256::from(2));
    expected.push(U256::from(1));
    assert_eq!(evm.stack, expected);
}

#[test]
#[should_panic(expected = "Stack underflow")]
fn test_add_underwflow() {
    let code: Vec<u8> = vec![0x01];
    let mut evm = Evm::new(code);
    evm.run();
}
