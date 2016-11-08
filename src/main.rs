use std::fs::File;
use std::env;

extern crate sfml;
use sfml::graphics::{CircleShape, Color, ConvexShape, Font, RenderTarget, RenderWindow, Sprite,
                     Text, Texture, Transformable};
use sfml::window::{Key, VideoMode, event, window_style};
use sfml::system::Vector2f;

static SCREEN_WIDTH: u32 = 64;
static SCREEN_HEIGHT: u32 = 32;
static SCREEN_SCALE: u32 = 8;

mod chip8;
use chip8::CHIP8;

fn main() {
    let rom_name = match env::args().nth(1) {
        Some(name) => name,
        None => panic!("Please enter a rom name.."),
    };

    println!("Loading ROM {}", rom_name);

    let mut chip = CHIP8::new(&mut File::open(&rom_name).unwrap());

    let mut window = RenderWindow::new(VideoMode::new_init(SCREEN_WIDTH * SCREEN_SCALE, SCREEN_HEIGHT * SCREEN_SCALE, 32),
                                       "CHIP8",
                                       window_style::CLOSE,
                                       &Default::default())
        .unwrap();
    window.set_vertical_sync_enabled(true);

    while !chip.done() {
        chip.tick();

        for event in window.events() {
            match event {
                event::Event::Closed |
                event::Event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }

        window.clear(&Color::black());
        window.display();
    }
}
