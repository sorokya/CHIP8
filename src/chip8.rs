use std::io::Read;

const RAM_SIZE: usize = 4096;
const GPR_COUNT: usize = 16;

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

    // General Purpose Registers
    v: Vec<u8>,

    // Index Register
    i: u16,

    // Program Counter
    pc: u16,

    // Pixels
    pub gfx: Vec<u8>,

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
    pub fn tick(&mut self) {
        let opcode = (((self.ram[self.pc as usize] as u16) << 8) | (self.ram[self.pc as usize + 1] as u16));

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as u16;
        let n = (opcode & 0x000F) as u8;

        // println!("OPCODE: {:X} PC: {} X: {} Y: {} KK: {} NNN: {} N: {}", opcode, self.pc, x, y, kk, nnn, n);

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x00FF {
                    0x00EE => self.ret(),
                    _ => panic!("Unrecognized opcode {:X}", opcode),
                }
            },
            0x2000 => self.call(nnn),
            0x6000 => self.ldvx(kk, x),
            0x7000 => self.addvx(kk, x),
            0xA000 => self.ldi(nnn),
            0xD000 => self.drw(x, y, n),
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => self.ldvxdt(x),
                    0x0015 => self.lddtvx(x),
                    0x0029 => self.ldfvx(x),
                    0x0033 => self.ldbvx(x),
                    0x0065 => self.ldvxi(x),
                    _ => panic!("Unrecognized opcode {:X}", opcode),
                }
            },
            _ => panic!("Unrecognized opcode {:X}", opcode),
        }

        self.pc += 2;
    }

    fn ldvx(&mut self, value: u8, register: usize) {
        self.v[register] = value;
    }

    fn addvx(&mut self, value: u8, register: usize) {
        self.v[register] += value;
    }

    fn ldi(&mut self, value: u16) {
        self.i = value;
    }

    fn call(&mut self, value: u16) {
        self.stack.push(self.pc);
        self.pc = value - 2;
    }

    fn ret(&mut self) {
        self.pc = self.stack.pop().unwrap() - 2;
    }

    fn ldbvx(&mut self, register: usize) {
        self.ram[self.i as usize] = self.v[register] / 100;
        self.ram[(self.i + 1) as usize] = (self.v[register] / 10) % 10;
        self.ram[(self.i + 2) as usize] = (self.v[register] % 100) % 10;
    }

    fn ldvxi(&mut self, end_register: usize) {
        for i in 0..end_register {
            self.ram[self.i as usize] = self.v[i];
        }
    }

    fn ldfvx(&mut self, register: usize) {
        self.i = FONT_SET[self.v[register] as usize] as u16;
    }

    fn lddtvx(&mut self, register: usize) {
        self.delay_timer = self.v[register];
    }

    fn ldvxdt(&mut self, register: usize) {
        self.v[register] = self.delay_timer;
    }

    fn drw(&mut self, x_register: usize, y_register: usize, number_of_bytes: u8) {
        let x = self.v[x_register] as u16;
        let y = self.v[y_register] as u16;
        let height = number_of_bytes as u16;
        let mut pixel: u16;

        self.v[0xF] = 0;
        for yline in 0..height {

            pixel = self.ram[(self.i + yline) as usize] as u16;
            for xline in 0..8 {
                if pixel & (0x80 >> xline) != 0 {
                    let index = (x + xline + (y + yline) * 64) as usize;

                    if self.gfx[index] == 1 {
                        self.v[0xF] = 1;
                    }

                    self.gfx[index] ^= 1;
                }
            }
        }
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
            v: vec![0; GPR_COUNT],
            i: 0,
            pc: 0x200,
            gfx: vec![0; (::SCREEN_WIDTH * ::SCREEN_HEIGHT) as usize],
            delay_timer: 0,
            sound_timer: 0,
            stack: Vec::new(),
            sp: 0,
            key: vec![0; NUMBER_OF_KEYS],
        }
    }
}
