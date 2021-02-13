#[macro_use]
extern crate log;

use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

mod chip8;

use chip8::Chip8;
use std::{fs::File, io::Read};

fn main() {
    env_logger::init();

    let mut chip8 = Chip8::new();

    let rom = File::open("./roms/random_number.ch8").expect("Unable to open file");
    chip8.load(rom.bytes().map(|byte| byte.expect("Unable to read byte")));

    let mut engine = console_engine::ConsoleEngine::init(64, 32, 60);

    loop {
        engine.wait_frame();
        engine.clear_screen();

        chip8.run_instruction();

        let rows: &[u64] = chip8.frame_buffer().as_ref();

        for (y, row) in rows.iter().enumerate() {
            for x in (0..64).rev() {
                if row >> x & 1 == 0 {
                    engine.set_pxl(64 - x, y as i32, pixel::pxl_fg('O', Color::Cyan));
                } else {
                    engine.set_pxl(64 - x, y as i32, pixel::pxl_fg('.', Color::Cyan));
                }
            }
        }

        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        engine.draw();
    }
}
