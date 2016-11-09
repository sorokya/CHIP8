use std::fs::File;
use std::env;

#[macro_use] extern crate enum_primitive;

extern crate sfml;
use sfml::graphics::{Vertex, VertexArray, Color, RenderTarget, RenderWindow,
                     PrimitiveType};
use sfml::window::{Key, VideoMode, Event, window_style};
use sfml::system::{Clock, Time, Vector2f};

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCREEN_SCALE: u32 = 8;

mod instruction;
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

    let mut scene = VertexArray::new_init(PrimitiveType::sfQuads, SCREEN_WIDTH * SCREEN_HEIGHT * 4);
    let mut clock = Clock::new();

    while !chip.done() {
        chip.key[0] = Key::Num1.is_pressed() as u8;
        chip.key[1] = Key::Num2.is_pressed() as u8;
        chip.key[2] = Key::Num3.is_pressed() as u8;
        chip.key[3] = Key::Num4.is_pressed() as u8;
        chip.key[4] = Key::Q.is_pressed() as u8;
        chip.key[5] = Key::W.is_pressed() as u8;
        chip.key[6] = Key::E.is_pressed() as u8;
        chip.key[7] = Key::R.is_pressed() as u8;
        chip.key[8] = Key::A.is_pressed() as u8;
        chip.key[9] = Key::S.is_pressed() as u8;
        chip.key[10] = Key::D.is_pressed() as u8;
        chip.key[11] = Key::F.is_pressed() as u8;
        chip.key[12] = Key::Z.is_pressed() as u8;
        chip.key[13] = Key::X.is_pressed() as u8;
        chip.key[14] = Key::C.is_pressed() as u8;
        chip.key[15] = Key::V.is_pressed() as u8;

        chip.tick(clock.restart().as_seconds());

        for event in window.events() {
            match event {
                Event::Closed |
                Event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }

        window.clear(&Color::black());
        scene.clear();

        for y in 0..SCREEN_HEIGHT as u32 {
            for x in 0..SCREEN_WIDTH as u32 {
                if chip.gfx[(x+y*SCREEN_WIDTH) as usize] == 1 {
                    scene.append(&Vertex::new(&Vector2f {
                        x: x as f32 * SCREEN_SCALE as f32,
                        y: y as f32 * SCREEN_SCALE as f32,
                    },
                    &Color::white(), &Vector2f {x:0.0,y:0.0}));

                    scene.append(&Vertex::new(&Vector2f {
                        x: (x + 1) as f32 * SCREEN_SCALE as f32,
                        y: y as f32 * SCREEN_SCALE as f32,
                    },
                    &Color::white(), &Vector2f {x:0.0,y:0.0}));

                    scene.append(&Vertex::new(&Vector2f {
                        x: (x + 1) as f32 * SCREEN_SCALE as f32,
                        y: (y + 1) as f32 * SCREEN_SCALE as f32,
                    },
                    &Color::white(), &Vector2f {x:0.0,y:0.0}));

                    scene.append(&Vertex::new(&Vector2f {
                        x: x as f32 * SCREEN_SCALE as f32,
                        y: (y + 1) as f32 * SCREEN_SCALE as f32,
                    },
                    &Color::white(), &Vector2f {x:0.0,y:0.0}));
                }
            }
        }

        window.draw(&scene);
        window.display();
    }
}
