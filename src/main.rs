#[macro_use]
extern crate log;

use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

mod chip8;

use chip8::{Chip8, InputKey};
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

        let current_key = if engine.is_key_pressed(KeyCode::Char('0')) {
            Some(InputKey::Key0)
        } else if engine.is_key_pressed(KeyCode::Char('1')) {
            Some(InputKey::Key1)
        } else if engine.is_key_pressed(KeyCode::Char('2')) {
            Some(InputKey::Key2)
        } else if engine.is_key_pressed(KeyCode::Char('3')) {
            Some(InputKey::Key3)
        } else if engine.is_key_pressed(KeyCode::Char('4')) {
            Some(InputKey::Key4)
        } else if engine.is_key_pressed(KeyCode::Char('5')) {
            Some(InputKey::Key5)
        } else if engine.is_key_pressed(KeyCode::Char('6')) {
            Some(InputKey::Key6)
        } else if engine.is_key_pressed(KeyCode::Char('7')) {
            Some(InputKey::Key7)
        } else if engine.is_key_pressed(KeyCode::Char('8')) {
            Some(InputKey::Key8)
        } else if engine.is_key_pressed(KeyCode::Char('9')) {
            Some(InputKey::Key9)
        } else if engine.is_key_pressed(KeyCode::Char('a')) {
            Some(InputKey::KeyA)
        } else if engine.is_key_pressed(KeyCode::Char('b')) {
            Some(InputKey::KeyB)
        } else if engine.is_key_pressed(KeyCode::Char('c')) {
            Some(InputKey::KeyC)
        } else if engine.is_key_pressed(KeyCode::Char('d')) {
            Some(InputKey::KeyD)
        } else if engine.is_key_pressed(KeyCode::Char('e')) {
            Some(InputKey::KeyE)
        } else if engine.is_key_pressed(KeyCode::Char('f')) {
            Some(InputKey::KeyF)
        } else {
            None
        };

        chip8.run_instruction(current_key);

        let rows: &[u64] = chip8.frame_buffer().as_ref();

        for (y, row) in rows.iter().enumerate() {
            for x in (0..64).rev() {
                if row >> x & 1 == 0 {
                    engine.set_pxl(64 - x, y as i32, pixel::pxl_fg(' ', Color::Cyan));
                } else {
                    engine.set_pxl(64 - x, y as i32, pixel::pxl_fg('X', Color::Cyan));
                }
            }
        }

        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        engine.draw();
    }
}
