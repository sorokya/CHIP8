use std::io::Read;

const RAM_SIZE: usize = 4096;
const GPR_COUNT: usize = 16;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const STACK_SIZE: usize = 16;
const NUMBER_OF_KEYS: usize = 16;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Debug)]
pub struct CHIP8 {
    ram: Vec<u8>,

    // Current opcode
    opcode: u16,

    // General Purpose Registers
    v: Vec<u8>,

    // Index Register
    i: u16,

    // Program Counter
    pc: u16,

    // Pixels
    gfx: Vec<u8>,

    // Timers
    delay_timer: u8,
    sound_timer: u8,

    // Stack & Stack Pointer
    stack: Vec<u16>,
    sp: u16,

    // Input state
    key: Vec<u8>,
}

impl CHIP8 {
    pub fn tick(&self) {
    }

    pub fn done(&self) -> bool {
        false
    }

    pub fn new(r: &mut Read) -> CHIP8 {
        /*
            0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
            0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
            0x200-0xFFF - Program ROM and work RAM
        */
        let mut ram = vec![0; RAM_SIZE];

        // initialize font set
        for i in 0..80 {
            ram[i] = FONT_SET[i];
        }

        let mut rom: Vec<u8> = Vec::new();
        r.read_to_end(&mut rom).unwrap();

        for i in 0..rom.len() {
            ram[i + 512] = rom[i];
        }

        CHIP8 {
            ram: ram,
            opcode: 0,
            v: vec![0; GPR_COUNT],
            i: 0,
            pc: 0x200,
            gfx: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![0; STACK_SIZE],
            sp: 0,
            key: vec![0; NUMBER_OF_KEYS],
        }
    }
}
