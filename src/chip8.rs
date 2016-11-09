#![allow(non_snake_case_types)]

use std::io::Read;

extern crate rand;
use self::rand::Rng;

use instruction::{Opcode, Instruction};

const RAM_SIZE: usize = 4096;
const GPR_COUNT: usize = 16;

const NUMBER_OF_KEYS: usize = 16;

const TIME_STEP: f32 = 1.0 / 30.0;

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
    sp: u16,

    // Input state
    pub key: Vec<u8>,

    jmp: bool,

    time_acc: f32,

    instruction: Instruction,

    pub draw: bool,
}

impl CHIP8 {
    pub fn tick(&mut self, dt: f32) {
        // self.instruction = Instruction::parse(((self.ram[self.pc as usize] as u16) << 8) | (self.ram[self.pc as usize + 1] as u16));
        // match self.instruction.opcode {
        //     Opcode::sys => {},
        //     _ => {},
        // }

        let opcode = ((self.ram[self.pc as usize] as u16) << 8) | (self.ram[self.pc as usize + 1] as u16);
        self.draw = false;

        // panic!("{:b} | {:b} = {:X}", (self.ram[self.pc as usize] as u16) << 8, (self.ram[self.pc as usize + 1] as u16), ((self.ram[self.pc as usize] as u16) << 8) | (self.ram[self.pc as usize + 1] as u16));

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as u16;
        let n = (opcode & 0x000F) as u8;

        self.jmp = false;

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x00FF {
                    0x0000 => self.SYS(nnn),
                    0x00E0 => self.CLS(),
                    0x00EE => self.RET(),
                    _ => panic!("Unrecognized opcode {:X}", opcode),
                }
            },
            0x1000 => self.JUMP(nnn),
            0x2000 => self.CALL(nnn),
            0x3000 => self.SE_VX(kk, x),
            0x4000 => self.SNE_VX(kk, x),
            0x5000 => self.SE_VX_VY(x, y),
            0x6000 => self.LD_VX(kk, x),
            0x7000 => self.ADD_VX(kk, x),
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => self.LD_VX_VY(x, y),
                    0x0001 => self.OR_VX_VY(x, y),
                    0x0002 => self.AND_VX_VY(x, y),
                    0x0003 => self.XOR_VX_VY(x, y),
                    0x0004 => self.ADD_VX_VY(x, y),
                    0x0005 => self.SUB_VX_VY(x, y),
                    0x0006 => self.SHR_VX(x),
                    0x0007 => self.SUBN_VX_VY(x, y),
                    0x000E => self.SHL_VX(x),
                    _ => panic!("Unrecognized opcode {:X}", opcode),
                }
            }
            0x9000 => self.SNE_VX_VY(x, y),
            0xA000 => self.LD_I(nnn),
            0xB000 => self.JUMP_V0(nnn),
            0xC000 => self.RND_VX_B(kk, x),
            0xD000 => self.DRAW(x, y, n),
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => self.SKP_VX(x),
                    0x00A1 => self.SKNP_VX(x),
                    _ => panic!("Unrecognized opcode {:X}", opcode),
                }
            }
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => self.LD_VX_DT(x),
                    0x000A => self.LD_VX_K(x),
                    0x0015 => self.LD_DT_VX(x),
                    0x0018 => self.LD_ST_VX(x),
                    0x001E => self.ADD_I_VX(x),
                    0x0029 => self.LD_F_VX(x),
                    0x0033 => self.LD_B_VX(x),
                    0x0055 => self.LD_I_VX(x),
                    0x0065 => self.LD_VX_I(x),
                    _ => panic!("Unrecognized opcode {:X}", opcode),
                }
            },
            _ => panic!("Unrecognized opcode {:X}", opcode),
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

    fn XOR_VX_VY(&mut self, x_register: usize, y_register:usize) {
        self.v[x_register] ^= self.v[y_register];
    }

    fn JUMP_V0(&mut self, value: u16) {
        self.pc = (self.v[0] as u16) + value;
        self.jmp = true;
    }

    fn SUBN_VX_VY(&mut self, x_register: usize, y_register: usize) {
        self.v[0xF] = (self.v[y_register] > self.v[x_register]) as u8;
        self.v[x_register] = self.v[y_register].wrapping_sub(self.v[x_register]);
    }

    fn SHR_VX(&mut self, x_register: usize) {
        self.v[0xf] = self.v[x_register] & 0x1;
        self.v[x_register] = self.v[x_register] >> 1;
    }

    fn SHL_VX(&mut self, x_register: usize) {
        self.v[0xf] = ((self.v[x_register] & 0xF0) >> 7 == 1) as u8;
        self.v[x_register] *= 2;
    }

    fn SE_VX_VY(&mut self, x_register: usize, y_register: usize) {
        if self.v[x_register] == self.v[y_register] {
            self.pc += 2;
        }
    }

    fn CLS(&mut self) {
        self.gfx = vec![0; (::SCREEN_WIDTH * ::SCREEN_HEIGHT) as usize];
    }

    fn SYS(&mut self, value: u16) {
        self.pc = value;
        self.jmp = true;
    }

    fn RET(&mut self) {
        self.pc = self.stack.pop().unwrap();
    }

    fn JUMP(&mut self, value: u16) {
        self.pc = value;
        self.jmp = true;
    }

    fn CALL(&mut self, value: u16) {
        self.stack.push(self.pc);
        self.pc = value;
        self.jmp = true;
    }

    fn SE_VX(&mut self, value: u8, register: usize) {
        if register == 15 && value == 1 {
            println!("Checking for ball collision VF:{:X}", self.v[0xF]);
        }

        if self.v[register] == value {
            self.pc += 2;
        }
    }

    fn SNE_VX(&mut self, value: u8, register: usize) {
        if self.v[register] != value {
            self.pc += 2;
        }
    }

    fn LD_VX(&mut self, value: u8, register: usize) {
        self.v[register] = value;
    }

    fn ADD_VX(&mut self, value: u8, register: usize) {
        self.v[register] = self.v[register].wrapping_add(value);
    }

    fn LD_VX_VY(&mut self, x_register: usize, y_register: usize) {
        self.v[x_register] = self.v[y_register];
    }

    fn ADD_VX_VY(&mut self, x_register: usize, y_register: usize) {
        let x = self.v[x_register] as u16;
        let y = self.v[y_register] as u16;
        let res = x + y;

        self.v[0xF] = (res > 255) as u8;
        self.v[x_register] = res as u8;
    }

    fn SUB_VX_VY(&mut self, x_register: usize, y_register: usize) {
        self.v[0xF] = (self.v[x_register] > self.v[y_register]) as u8;
        self.v[x_register].wrapping_sub(self.v[y_register]);
    }

    fn OR_VX_VY(&mut self, x_register: usize, y_register: usize) {
        self.v[x_register] |= self.v[y_register];
    }

    fn AND_VX_VY(&mut self, x_register: usize, y_register: usize) {
        self.v[x_register] &= self.v[y_register];
    }

    fn SNE_VX_VY(&mut self, x_register: usize, y_register: usize) {
        if self.v[x_register] != self.v[y_register] {
            self.pc += 2;
        }
    }

    fn LD_I_VX(&mut self, x_register: usize) {
        for i in 0usize..x_register {
            self.ram[self.i as usize + i] = self.v[i];
        }
    }

    fn LD_ST_VX(&mut self, x_register: usize) {
        self.sound_timer = self.v[x_register];
    }

    fn SKP_VX(&mut self, x_register: usize) {
        if self.key[self.v[x_register] as usize] == 1 {
            self.pc += 2;
        }
    }

    fn SKNP_VX(&mut self, x_register: usize) {
        if self.key[self.v[x_register] as usize] == 0 {
            self.pc += 2;
        }
    }

    fn RND_VX_B(&mut self, value: u8, x_register: usize) {
        let mut rng = rand::thread_rng();
        self.v[x_register] = rng.gen::<u8>() & value;
    }

    fn LD_VX_K(&mut self, register: usize) {
        let mut key_pressed = false;

        for i in 0..self.key.len() {
            if self.key[i] == 1 {
                self.ram[register] = i as u8;
                key_pressed = true;
            }
        }

        if !key_pressed {
            self.pc -= 2;
        }
    }

    fn ADD_I_VX(&mut self, register: usize) {
        self.v[0xf] = (self.i + (self.v[register] as u16) > 255) as u8;
        self.i += self.v[register] as u16;
    }

    fn LD_I(&mut self, value: u16) {
        self.i = value;
    }

    fn LD_B_VX(&mut self, register: usize) {
        self.ram[self.i as usize] = self.v[register] / 100;
        self.ram[self.i as usize + 1] = (self.v[register] / 100) % 10;
        self.ram[self.i as usize + 2] = (self.v[register] % 100) % 10;
    }

    fn LD_VX_I(&mut self, end_register: usize) {
        for i in 0usize..end_register {
            self.v[i] = self.ram[self.i as usize + i];
        }
    }

    fn LD_F_VX(&mut self, register: usize) {
        self.i = (self.v[register] * 5) as u16;
    }

    fn LD_DT_VX(&mut self, register: usize) {
        self.delay_timer = self.v[register];
    }

    fn LD_VX_DT(&mut self, register: usize) {
        self.v[register] = self.delay_timer;
    }

    fn DRAW(&mut self, x_register: usize, y_register: usize, number_of_bytes: u8) {
        let x = self.v[x_register] as usize;
        let y = self.v[y_register] as usize;
        let height = number_of_bytes as usize;

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

    pub fn done(&self) -> bool {
        false
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
            sp: 0,
            key: vec![0; NUMBER_OF_KEYS],
            jmp: false,
            time_acc: 0.0,
            instruction: Instruction::new(),
            draw: false,
        }
    }
}
