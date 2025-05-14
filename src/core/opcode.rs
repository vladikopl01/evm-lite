pub enum Opcode {
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
    pub fn from_byte(codes: &[u8], pc: usize) -> (Self, usize) {
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
