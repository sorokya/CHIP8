use std::fs::File;
use std::env;

extern crate sfml;
use sfml::graphics::{Vertex, VertexArray, Color, ConvexShape, Font, RenderTarget, RenderWindow, Sprite,
                     Text, Texture, Transformable, PrimitiveType};
use sfml::window::{Key, VideoMode, event, window_style};
use sfml::system::Vector2f;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCREEN_SCALE: u32 = 8;

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

    let mut scene = VertexArray::new_init(PrimitiveType::sfQuads, SCREEN_WIDTH * SCREEN_HEIGHT * 4).unwrap();

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
        scene.clear();

        // println!("{:?}", chip.gfx);
        for x in 0..SCREEN_WIDTH as u32 {
            for y in 0..SCREEN_HEIGHT as u32 {
                if chip.gfx[(x+y*SCREEN_WIDTH) as usize] == 1 {
                    scene.append(&Vertex::new(&Vector2f {
                        x: x as f32 * SCREEN_SCALE as f32,
                        y: y as f32 * SCREEN_SCALE as f32,
                    },
                    &Color::green(), &Vector2f {x:0.0,y:0.0}));

                    scene.append(&Vertex::new(&Vector2f {
                        x: (x + 1) as f32 * SCREEN_SCALE as f32,
                        y: y as f32 * SCREEN_SCALE as f32,
                    },
                    &Color::green(), &Vector2f {x:0.0,y:0.0}));

                    scene.append(&Vertex::new(&Vector2f {
                        x: (x + 1) as f32 * SCREEN_SCALE as f32,
                        y: (y + 1) as f32 * SCREEN_SCALE as f32,
                    },
                    &Color::green(), &Vector2f {x:0.0,y:0.0}));

                    scene.append(&Vertex::new(&Vector2f {
                        x: x as f32 * SCREEN_SCALE as f32,
                        y: (y + 1) as f32 * SCREEN_SCALE as f32,
                    },
                    &Color::green(), &Vector2f {x:0.0,y:0.0}));
                }
            }
        }

        window.draw(&scene);
        window.display();
    }
}
