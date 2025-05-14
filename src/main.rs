enum Opcode {
    PUSH1(u8),
    ADD,
    STOP,
    Unknown(u8),
}

impl Opcode {
    fn from_byte(codes: &[u8], pc: usize) -> (Self, usize) {
        match codes.get(pc) {
            Some(0x60) => {
                let val = *codes.get(pc + 1).unwrap_or(&0);
                (Opcode::PUSH1(val), 2)
            }
            Some(0x01) => (Opcode::ADD, 1),
            Some(0x00) => (Opcode::STOP, 1),
            Some(&byte) => (Opcode::Unknown(byte), 1),
            None => (Opcode::STOP, 1),
        }
    }
}

struct Evm {
    stack: Vec<u8>,
    pc: usize,
    codes: Vec<u8>,
}

impl Evm {
    fn new(codes: Vec<u8>) -> Self {
        Evm {
            codes,
            pc: 0,
            stack: vec![],
        }
    }

    fn run(&mut self) {
        while self.pc < self.codes.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        let (opcode, consumed_bytes) = Opcode::from_byte(&self.codes, self.pc);
        self.pc += consumed_bytes;

        match opcode {
            Opcode::PUSH1(value) => self.stack.push(value),
            Opcode::ADD => {
                let a = self.stack.pop().expect("Stack underflow");
                let b = self.stack.pop().expect("Stack underflow");
                self.stack.push(a + b);
            }
            Opcode::STOP => self.pc = self.codes.len(),
            Opcode::Unknown(byte) => panic!("Unknown opcode: {:?}", byte),
        }
    }
}

fn main() {
    // PUSH1, 0x01, PUSH1, 0x02, ADD, STOP
    let codes: Vec<u8> = vec![0x60, 0x01, 0x60, 0x02, 0x01, 0x00];
    let mut evm = Evm::new(codes);
    evm.run();

    println!("Final stack: {:?}", evm.stack);
}
