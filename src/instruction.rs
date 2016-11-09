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
        sub_vv = 0x8005,
        shr = 0x8006,
        subn_vv = 0x8007,
        shl = 0x800E,
        sne_vv = 0x9000,
        ld_i = 0xA000,
        jp_v = 0xB000,
        rnd = 0xC000,
        drw = 0xD000,
        skp = 0xE09E,
        sknp = 0xE0A1,
        ld_v_dt = 0xF007,
        ld_v_k = 0xF00A,
        ld_dt_v = 0xF015,
        ld_st_v = 0xF018,
        add_i_v = 0xF01E,
        ld_f_v = 0xF029,
        ld_b_v = 0xF033,
        ld_i_v = 0xF055,
        ld_v_i = 0xF065,
        invalid,
    }
}

pub struct Instruction {
    pub opcode: Opcode,
    x: usize,
    y: usize,
    kk: u8,
    nnn: u16,
    n: u8,
}

impl Instruction {
    pub fn parse(op: u16) -> Instruction {
        println!("{:X}", op);

        Instruction {
            opcode: Opcode::invalid,
            x: 0,
            y: 0,
            kk: 0,
            nnn: 0,
            n: 0,
        }
    }

    pub fn new() -> Instruction {
        Instruction {
            opcode: Opcode::invalid,
            x: 0,
            y: 0,
            kk: 0,
            nnn: 0,
            n: 0,
        }
    }
}
