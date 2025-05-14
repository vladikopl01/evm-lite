use core::evm::Evm;

mod core;

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

    println!("Final stack: {:?}", evm.stack.dump());
}
