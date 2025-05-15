pub enum Opcode {
    STOP,         // 0x00
    ADD,          // 0x01
    MUL,          // 0x02
    SUB,          // 0x03
    MLOAD,        // 0x51
    MSTORE,       // 0x52
    JUMP,         // 0x56
    JUMPI,        // 0x57
    JUMPDEST,     // 0x5B
    PUSHn(usize), // 0x60 - 0x7F
    DUPn(usize),  // 0x80 - 0x8F
    SWAPn(usize), // 0x90 - 0x9F
    Unknown(usize),
}

impl Opcode {
    pub fn from_byte(codes: &[u8], pc: usize) -> (Self, usize) {
        match codes.get(pc) {
            Some(0x00) => (Opcode::STOP, 1),
            Some(0x01) => (Opcode::ADD, 1),
            Some(0x02) => (Opcode::MUL, 1),
            Some(0x03) => (Opcode::SUB, 1),
            Some(0x51) => (Opcode::MLOAD, 1),
            Some(0x52) => (Opcode::MSTORE, 1),
            Some(0x56) => (Opcode::JUMP, 0),
            Some(0x57) => (Opcode::JUMPI, 0),
            Some(0x5B) => (Opcode::JUMPDEST, 1),
            Some(&op) if op >= 0x60 && op <= 0x7F => {
                let n = (op - 0x60 + 1).into();
                (Opcode::PUSHn(n), n + 1)
            }
            Some(&op) if op >= 0x80 && op <= 0x8F => (Opcode::DUPn((op - 0x80 + 1).into()), 1),
            Some(&op) if op >= 0x90 && op <= 0x9F => (Opcode::SWAPn((op - 0x90 + 1).into()), 1),
            Some(&other) => (Opcode::Unknown(other.into()), 1),
            None => (Opcode::STOP, 1),
        }
    }
}
