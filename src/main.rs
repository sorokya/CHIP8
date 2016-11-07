mod chip8;
use chip8::CHIP8;

fn main() {
    let chip = CHIP8::new();
    println!("{:?}", chip);
}
