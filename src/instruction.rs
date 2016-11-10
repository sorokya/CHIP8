#![allow(non_camel_case_types)]

use std::fmt;
use enum_primitive::FromPrimitive;

enum_from_primitive! {
    #[derive(Debug,PartialEq)]
    pub enum Opcode {
        sys = 0x0000,
        cls = 0x00E0,
        ret = 0x00EE,
        jmp = 0x1000,
        call = 0x2000,
        se_vb = 0x3000,
        sne_vb = 0x4000,
        se_vv = 0x5000,
        ld_vb = 0x6000,
        add_vb = 0x7000,
        ld_vv = 0x8000,
        or = 0x8001,
        and = 0x8002,
        xor = 0x8003,
        add_vv = 0x8004,
        sub = 0x8005,
        shr = 0x8006,
        subn = 0x8007,
        shl = 0x800E,
        sne_vv = 0x9000,
        ld_i = 0xA000,
        jmp_v = 0xB000,
        rnd = 0xC000,
        drw = 0xD000,
        skp = 0xE09E,
        sknp = 0xE0A1,
        ld_vdt = 0xF007,
        ld_vk = 0xF00A,
        ld_dtv = 0xF015,
        ld_stv = 0xF018,
        add_iv = 0xF01E,
        ld_fv = 0xF029,
        ld_bv = 0xF033,
        ld_iv = 0xF055,
        ld_vi = 0xF065,
        noop = 0xFFFF,
        invalid,
    }
}

#[derive(Default)]
pub struct Instruction {
    pub opcode: u16,
    pub x: usize,
    pub y: usize,
    pub kk: u8,
    pub nnn: u16,
    pub n: u8,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} X: {} Y: {} KK: {} NNN: {} N: {}", Opcode::from_u16(self.opcode).unwrap(), self.x, self.y, self.kk, self.nnn, self.n)
    }
}

impl Instruction {
    pub fn parse(&mut self, first: u8, second: u8) {
        self.x = 0;
        self.y = 0;
        self.kk = 0;
        self.nnn = 0;
        self.n = 0;

        let op = first as u16 >> 4;
        match op {
            0x00 => {
                self.opcode = op << 12 | second as u16;
            },
            0x01 | 0x02 | 0x0A | 0x0B => {
                self.opcode = op << 12;
                self.nnn = (first as u16 & 0x0F) << 8 | second as u16;
            },
            0x03 | 0x04 | 0x06 | 0x07 | 0x0C => {
                self.opcode = op << 12;
                self.x = (first & 0x0F) as usize;
                self.kk = second;
            },
            0x05 | 0x08 | 0x09 => {
                self.opcode = op << 12 | second as u16 & 0x0F;
                self.x = (first & 0x0F) as usize;
                self.y = ((second & 0xF0) >> 4) as usize;
            }
            0x0E | 0x0F => {
                self.opcode = op << 12 | second as u16;
                self.x = (first & 0x0F) as usize;
            }
            0x0D => {
                self.opcode = op << 12;
                self.x = (first & 0x0F) as usize;
                self.y = ((second & 0xF0) >> 4) as usize;
                self.n = second & 0x0F;
            }
            _ => {
                self.opcode = 0xFFFF;
            }
        }
    }

    pub fn new() -> Instruction {
        Instruction::default()
    }
}
