use std::fs::File;
use std::env;

mod chip8;
use chip8::CHIP8;

fn main() {
    let rom_name = match env::args().nth(1) {
        Some(name) => name,
        None => panic!("Please enter a rom name.."),
    };

    println!("Loading ROM {}", rom_name);

    let chip = CHIP8::new(&mut File::open(&rom_name).unwrap());

    while !chip.done() {
        chip.tick();
    }
}
