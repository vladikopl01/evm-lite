fn main() {
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

    println!("Final stack: {:?}", evm.stack);
}

enum Opcode {
    STOP,        // 0x00
    ADD,         // 0x01
    MUL,         // 0x02
    SUB,         // 0x03
    JUMP,        // 0x56
    JUMPI,       // 0x57
    JUMPDEST,    // 0x5B
    PUSHn(u128), // 0x60 - 0x7F
    DUPn(u8),    // 0x80 - 0x8F
    SWAPn(u8),   // 0x90 - 0x9F
    Unknown(u128),
}

impl Opcode {
    fn from_byte(codes: &[u8], pc: usize) -> (Self, usize) {
        match codes.get(pc) {
            Some(0x00) => (Opcode::STOP, 1),
            Some(0x01) => (Opcode::ADD, 1),
            Some(0x02) => (Opcode::MUL, 1),
            Some(0x03) => (Opcode::SUB, 1),
            Some(0x56) => (Opcode::JUMP, 0),
            Some(0x57) => (Opcode::JUMPI, 0),
            Some(0x5B) => (Opcode::JUMPDEST, 1),
            Some(&op) if op >= 0x60 && op <= 0x7F => {
                let n = (op - 0x60 + 1) as usize;
                let mut val: u128 = 0;
                for i in 0..n {
                    val = (val << 8) | *codes.get(pc + 1 + i).unwrap_or(&0) as u128;
                }
                (Opcode::PUSHn(val), n + 1)
            }
            Some(&op) if op >= 0x80 && op <= 0x8F => {
                let n = op - 0x80 + 1;
                (Opcode::DUPn(n), 1)
            }
            Some(&op) if op >= 0x90 && op <= 0x9F => {
                let n = op - 0x90 + 1;
                (Opcode::SWAPn(n), 1)
            }
            Some(&other) => (Opcode::Unknown(other as u128), 1),
            None => (Opcode::STOP, 1),
        }
    }
}

struct Evm {
    stack: Vec<u128>,
    pc: usize,
    code: Vec<u8>,
}

impl Evm {
    fn new(code: Vec<u8>) -> Self {
        Evm {
            code,
            pc: 0,
            stack: vec![],
        }
    }

    fn run(&mut self) {
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
            Opcode::JUMP => {
                let dest = self.stack.pop().expect("Stack underflow") as usize;
                match self.code.get(dest) {
                    Some(&0x5B) => self.pc = dest,
                    _ => panic!("Invalid jump destination"),
                }
            }
            Opcode::JUMPI => {
                let cond = self.stack.pop().expect("Stack underflow");
                let dest = self.stack.pop().expect("Stack underflow") as usize;

                if cond != 0 {
                    match self.code.get(dest) {
                        Some(&0x5B) => self.pc = dest,
                        _ => panic!("Invalid jump destination"),
                    }
                } else {
                    self.pc += 1;
                }
            }
            Opcode::JUMPDEST => {}
            Opcode::PUSHn(value) => self.stack.push(value),
            Opcode::DUPn(n) => {
                let n = n as usize;
                if let Some(&value) = self.stack.get(self.stack.len() - n) {
                    self.stack.push(value);
                    return;
                }
                panic!("Stack underflow");
            }
            Opcode::SWAPn(n) => {
                let n = n as usize;
                let len = self.stack.len();
                if n < len {
                    self.stack.swap(len - 1, len - n - 1);
                    return;
                }
                panic!("Stack underflow");
            }
            Opcode::Unknown(byte) => panic!("Unknown opcode: {:?}", byte),
        }
    }

    fn bin_op<F>(&mut self, func: F)
    where
        F: Fn(u128, u128) -> u128,
    {
        let a = self.stack.pop().expect("Stack underflow");
        let b = self.stack.pop().expect("Stack underflow");
        self.stack.push(func(b, a));
    }
}

#[test]
fn test_stop() {
    let code: Vec<u8> = vec![0x00];
    let mut evm = Evm::new(code);
    evm.run();
    assert_eq!(evm.stack, vec![]);
}

#[test]
fn test_add() {
    let code: Vec<u8> = vec![0x60, 0x01, 0x60, 0x02, 0x01, 0x00];
    let mut evm = Evm::new(code);
    evm.run();
    assert_eq!(evm.stack, vec![3]);
}

#[test]
fn test_mul() {
    let code: Vec<u8> = vec![0x60, 0x02, 0x60, 0x03, 0x02, 0x00];
    let mut evm = Evm::new(code);
    evm.run();
    assert_eq!(evm.stack, vec![6]);
}

#[test]
fn test_push() {
    let code: Vec<u8> = vec![0x60, 0x01, 0x61, 0x01, 0x00, 0x62, 0x01, 0x00, 0x00, 0x00];
    let mut evm = Evm::new(code);
    evm.run();
    assert_eq!(evm.stack, vec![1, 256, 65536]);
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
    assert_eq!(evm.stack, vec![3]);
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
    assert_eq!(evm.stack, vec![1, 11]);
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
    assert_eq!(evm.stack, vec![3]);
}

#[test]
fn test_dup() {
    let code: Vec<u8> = vec![0x60, 0x01, 0x60, 0x02, 0x60, 0x03, 0x81, 0x00];
    let mut evm = Evm::new(code);
    evm.run();
    assert_eq!(evm.stack, vec![1, 2, 3, 2]);
}

#[test]
fn test_swap() {
    let code: Vec<u8> = vec![0x60, 0x01, 0x60, 0x02, 0x60, 0x03, 0x91, 0x00];
    let mut evm = Evm::new(code);
    evm.run();
    assert_eq!(evm.stack, vec![3, 2, 1]);
}

#[test]
#[should_panic(expected = "Stack underflow")]
fn test_add_underwflow() {
    let code: Vec<u8> = vec![0x01];
    let mut evm = Evm::new(code);
    evm.run();
}
