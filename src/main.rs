fn main() {
    let chip = CHIP8::new();
    println!("{:?}", chip);
}

const RAM_SIZE: usize = 4096;
const GPR_COUNT: usize = 16;

#[derive(Debug)]
struct CHIP8 {
    ram: Vec<u8>,

    // Current opcode
    opcode: u16,

    // General Purpose Registers
    v: Vec<u8>,

    // Index Register
    i: u16,

    // Program Counter
    pc: u16,
}

impl CHIP8 {
    pub fn new() -> CHIP8 {
        let ram = vec![0; RAM_SIZE];

        CHIP8 {
            ram: ram,
            opcode: 0,
            v: vec![0; GPR_COUNT],
            i: 0,
            pc: 0,
        }
    }
}
