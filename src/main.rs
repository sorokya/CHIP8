use std::fs::File;
use std::io::{stdin, stdout};
use std::io::prelude::*;

#[macro_use] extern crate enum_primitive;

extern crate sfml;
use sfml::graphics::{Vertex, VertexArray, Color, RenderTarget, RenderWindow,
                     PrimitiveType};
use sfml::window::{Key, VideoMode, Event, window_style};
use sfml::system::{Clock, Time, Vector2f};

extern crate clap;
use clap::{Arg, App};

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;
const SCREEN_SCALE: u32 = 8;

mod instruction;
mod chip8;
use chip8::CHIP8;

fn main() {
    let matches = App::new("CHIP-8")
                    .version("1.0")
                    .author("Richard Leek <thisisdigitx@gmail.com>")
                    .about("CHIP-8 virtual machine in Rust")
                    .arg(Arg::with_name("INPUT")
                        .help("The path to the rom to load")
                        .required(true)
                        .index(1))
                    .arg(Arg::with_name("debug")
                        .help("Runs the rom in debug mode")
                        .short("d"))
                    .get_matches();

    let rom_name = matches.value_of("INPUT").unwrap();
    let debug = matches.is_present("debug");

    let mut chip = CHIP8::new(&mut File::open(&rom_name).unwrap());



    if !debug {
        let mut window = RenderWindow::new(VideoMode::new_init(SCREEN_WIDTH * SCREEN_SCALE, SCREEN_HEIGHT * SCREEN_SCALE, 32),
                                           "CHIP8",
                                           window_style::CLOSE,
                                           &Default::default())
            .unwrap();

        window.set_vertical_sync_enabled(true);

        let mut scene = VertexArray::new_init(PrimitiveType::sfQuads, SCREEN_WIDTH * SCREEN_HEIGHT * 4);
        let mut clock = Clock::new();
        
        while !chip.done() {

            update(&mut chip, &window);
            chip.tick(clock.restart().as_seconds());
            draw(&chip, &mut window, &mut scene);
        }
    } else {
        println!("Debug Mode.. Press return to step");
        loop {
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            chip.tick(0.0);
            println!("{}", chip);
        }
    }

    fn update(chip: &mut CHIP8, window: &RenderWindow) {
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

        for event in window.events() {
            match event {
                Event::Closed |
                Event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }
    }

    fn draw(chip: &CHIP8, window: &mut RenderWindow, scene: &mut VertexArray) {
        if chip.draw {
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

            window.draw(scene);
            window.display();
        }
    }
}
