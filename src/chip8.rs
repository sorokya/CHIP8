use std::io::Read;
use std::fmt;

extern crate rand;
use self::rand::Rng;

use instruction::{Opcode, Instruction};
use enum_primitive::FromPrimitive;

const RAM_SIZE: usize = 4096;
const GPR_COUNT: usize = 16;

const NUMBER_OF_KEYS: usize = 16;

// 60Hz
const TIME_STEP: f32 = 1.0 / 60.0;

const FONT_SET: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0x90, 0x90, 0xf0, 0x10, 0x10, // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
    0xf0, 0x80, 0x80, 0x80, 0x80, // C
    0xe0, 0x90, 0x90, 0x90, 0xe0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80, // F
];

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

    // Input state
    pub key: Vec<u8>,

    jmp: bool,

    time_acc: f32,

    instruction: Instruction,

    pub draw: bool,

    pub done: bool,
}

impl fmt::Display for CHIP8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PC: {}\nInstruction: {}\nRegisters: {:?}\nDelay Timer: {}\nSound Timer: {}\nI: {}", self.pc, self.instruction, self.v, self.delay_timer, self.sound_timer, self.i)
    }
}

impl CHIP8 {
    pub fn tick(&mut self, dt: f32) {
        self.jmp = false;
        self.draw = false;

        self.instruction.parse(self.ram[self.pc as usize], self.ram[self.pc as usize + 1]);

        // println!("{}", self);

        match Opcode::from_u16(self.instruction.opcode).unwrap() {
            Opcode::sys | Opcode::noop => {},
            Opcode::cls => self.cls(),
            Opcode::ret => self.ret(),
            Opcode::jmp => self.jmp(),
            Opcode::call => self.call(),
            Opcode::se_vb => self.se_vb(),
            Opcode::sne_vb => self.sne_vb(),
            Opcode::se_vv => self.se_vv(),
            Opcode::ld_vb => self.ld_vb(),
            Opcode::add_vb => self.add_vb(),
            Opcode::ld_vv => self.ld_vv(),
            Opcode::or => self.or(),
            Opcode::and => self.and(),
            Opcode::xor => self.xor(),
            Opcode::add_vv => self.add_vv(),
            Opcode::sub => self.sub(),
            Opcode::shr => self.shr(),
            Opcode::subn => self.subn(),
            Opcode::shl => self.shl(),
            Opcode::sne_vv => self.sne_vv(),
            Opcode::ld_i => self.ld_i(),
            Opcode::jmp_v => self.jmp_v(),
            Opcode::rnd => self.rnd(),
            Opcode::drw => self.drw(),
            Opcode::skp => self.skp(),
            Opcode::sknp => self.sknp(),
            Opcode::ld_vdt => self.ld_vdt(),
            Opcode::ld_vk => self.ld_vk(),
            Opcode::ld_dtv => self.ld_dtv(),
            Opcode::ld_stv => self.ld_stv(),
            Opcode::add_iv => self.add_iv(),
            Opcode::ld_fv => self.ld_fv(),
            Opcode::ld_bv => self.ld_bv(),
            Opcode::ld_iv => self.ld_iv(),
            Opcode::ld_vi => self.ld_vi(),
            _ => panic!("Unrecognized opcode {:X}", self.instruction.opcode),
        }

        if !self.jmp {
            self.pc += 2;
        }

        self.time_acc += dt;
        while self.time_acc >= TIME_STEP {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }

            self.time_acc -= TIME_STEP;
        }
    }

    fn cls(&mut self) {
        self.gfx = vec![0; (::SCREEN_WIDTH * ::SCREEN_HEIGHT) as usize];
    }

    fn ret(&mut self) {
        self.pc = self.stack.pop().unwrap();
    }

    fn jmp(&mut self) {
        self.pc = self.instruction.nnn;
        self.jmp = true;
    }

    fn call(&mut self) {
        self.stack.push(self.pc);
        self.pc = self.instruction.nnn;
        self.jmp = true;
    }

    fn se_vb(&mut self) {
        if self.v[self.instruction.x] == self.instruction.kk {
            self.pc += 2;
        }
    }

    fn sne_vb(&mut self) {
        if self.v[self.instruction.x] != self.instruction.kk {
            self.pc += 2;
        }
    }

    fn se_vv(&mut self) {
        if self.v[self.instruction.x] == self.v[self.instruction.y] {
            self.pc += 2;
        }
    }

    fn ld_vb(&mut self) {
        self.v[self.instruction.x] = self.instruction.kk;
    }

    fn add_vb(&mut self) {
        self.v[self.instruction.x] = self.v[self.instruction.x].wrapping_add(self.instruction.kk);
    }

    fn ld_vv(&mut self) {
        self.v[self.instruction.x] = self.v[self.instruction.y];
    }

    fn or(&mut self) {
        self.v[self.instruction.x] |= self.v[self.instruction.y];
    }

    fn and(&mut self) {
        self.v[self.instruction.x] &= self.v[self.instruction.y];
    }

    fn xor(&mut self) {
        self.v[self.instruction.x] ^= self.v[self.instruction.y];
    }

    fn add_vv(&mut self) {
        let x = self.v[self.instruction.x] as u16;
        let y = self.v[self.instruction.y] as u16;
        let res = x + y;

        // println!("add_vv result: {}", res);

        self.v[0xF] = (res > 255) as u8;
        self.v[self.instruction.x] = res as u8;

        // println!("{} + {}; VF is {}", self.v[self.instruction.y], self.v[self.instruction.x], self.v[0xF]);
    }

    fn sub(&mut self) {
        // println!("sub x:{} y:{}", self.v[self.instruction.x], self.v[self.instruction.y])
        self.v[0xF] = (self.v[self.instruction.x] > self.v[self.instruction.y]) as u8;
        self.v[self.instruction.x] = self.v[self.instruction.x].wrapping_sub(self.v[self.instruction.y]);

        // println!("{} - {}; VF is {}", self.v[self.instruction.x], self.v[self.instruction.y], self.v[0xF]);
    }

    fn shr(&mut self) {
        self.v[0xf] = ((self.v[self.instruction.x] & 0xF0) >> 7 == 1) as u8;
        self.v[self.instruction.x] = self.v[self.instruction.x] / 2;
    }

    fn subn(&mut self) {
        self.v[0xF] = (self.v[self.instruction.y] > self.v[self.instruction.x]) as u8;
        self.v[self.instruction.x] = self.v[self.instruction.y].wrapping_sub(self.v[self.instruction.x]);
        // println!("{} - {}; VF is {}", self.v[self.instruction.y], self.v[self.instruction.x], self.v[0xF]);
    }

    fn shl(&mut self) {
        self.v[0xf] = (self.v[self.instruction.x] & 0xF == 1) as u8;
        self.v[self.instruction.x] *= 2;
    }

    fn sne_vv(&mut self) {
        if self.v[self.instruction.x] != self.v[self.instruction.y] {
            self.pc += 2;
        }
    }

    fn ld_i(&mut self) {
        self.i = self.instruction.nnn;
    }

    fn jmp_v(&mut self) {
        self.pc = (self.v[0] as u16) + self.instruction.nnn;
        self.jmp = true;
    }

    fn rnd(&mut self) {
        let mut rng = rand::thread_rng();
        self.v[self.instruction.x] = rng.gen::<u8>() & self.instruction.kk;
    }

    fn drw(&mut self) {
        let x = self.v[self.instruction.x] as usize;
        let y = self.v[self.instruction.y] as usize;
        let height = self.instruction.n as usize;

        self.v[0xF] = 0;

        for yline in 0..height {
            let pixel = self.ram[(self.i + yline as u16) as usize];
            for xline in 0..8 {
                if pixel & (0x80 >> xline) != 0 {
                    let mut index = (x + xline + (y + yline) * 64) as u16;

                    index = if index >= 64*32 {
                        64*32-1
                    } else {
                        index
                    };

                    if self.gfx[index as usize] == 1 {
                        self.v[0xF] = 1;
                    }

                    self.gfx[index as usize] ^= 1;
                }
            }
        }

        self.draw = true;
    }

    fn skp(&mut self) {
        if self.key[self.v[self.instruction.x] as usize] == 1 {
            self.pc += 2;
        }
    }

    fn sknp(&mut self) {
        if self.key[self.v[self.instruction.x] as usize] == 0 {
            self.pc += 2;
        }
    }

    fn ld_vdt(&mut self) {
        self.v[self.instruction.x] = self.delay_timer;
    }

    fn ld_vk(&mut self) {
        let mut key_pressed = false;

        for i in 0..self.key.len() {
            if self.key[i] == 1 {
                self.v[self.instruction.x] = i as u8;
                key_pressed = true;
            }
        }

        if !key_pressed {
            self.pc -= 2;
        }
    }

    fn ld_dtv(&mut self) {
        self.delay_timer = self.v[self.instruction.x];
    }

    fn ld_stv(&mut self) {
        self.sound_timer = self.v[self.instruction.x];
    }

    fn add_iv(&mut self) {
        self.i += self.v[self.instruction.x] as u16;
    }

    fn ld_fv(&mut self) {
        self.i = (self.v[self.instruction.x] * 5) as u16;
    }

    fn ld_bv(&mut self) {
        self.ram[self.i as usize] = self.v[self.instruction.x] / 100;
        self.ram[self.i as usize + 1] = (self.v[self.instruction.x] / 10) % 10;
        self.ram[self.i as usize + 2] = (self.v[self.instruction.x] % 100) % 10;
    }

    fn ld_iv(&mut self) {
        for i in 0usize..self.instruction.x + 1 {
            self.ram[self.i as usize + i] = self.v[i];
        }
    }

    fn ld_vi(&mut self) {
        for i in 0usize..self.instruction.x + 1 {
            self.v[i] = self.ram[self.i as usize + i];
        }
    }

    pub fn new(r: &mut Read) -> CHIP8 {
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
            key: vec![0; NUMBER_OF_KEYS],
            jmp: false,
            time_acc: 0.0,
            instruction: Instruction::new(),
            draw: false,
            done: false,
        }
    }
}
