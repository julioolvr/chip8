#[macro_use]
extern crate log;

use console_engine::{pixel, Color, ConsoleEngine, KeyCode};

mod chip8;

use chip8::{Chip8, InputKey};
use std::{fs::File, io::Read, thread};

fn main() {
    env_logger::init();

    let (display_tx, display_rx) = std::sync::mpsc::channel();
    let (input_tx, input_rx) = std::sync::mpsc::channel();

    let mut chip8 = Chip8::new(display_tx, input_rx);

    let rom = File::open("./roms/random_number.ch8").expect("Unable to open file");
    chip8.load(rom.bytes().map(|byte| byte.expect("Unable to read byte")));

    let screen_width = chip8.get_screen_width();
    let screen_height = chip8.get_screen_height();

    let mut engine =
        console_engine::ConsoleEngine::init(screen_width as u32 + 2, screen_height as u32 + 10, 60);
    let mut stopwatch = std::time::Instant::now();
    let mut last_fps = 0;

    let mut last_buffer = (*chip8.frame_buffer()).clone();

    thread::spawn(move || {
        chip8.run();
    });

    loop {
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        if let Some(key) = get_current_key(&engine) {
            input_tx.send(key.into()).unwrap();
        }

        engine.wait_frame();

        let draw_instruction = display_rx.try_recv();

        if let Ok(draw_instruction) = draw_instruction {
            last_buffer = draw_instruction.buffer().clone();
        }

        engine.clear_screen();

        engine.line(
            0,
            0,
            screen_width as i32 + 1,
            0,
            pixel::pxl_fg('#', Color::Grey),
        );

        engine.line(
            0,
            screen_height as i32 + 1,
            screen_width as i32 + 1,
            screen_height as i32 + 1,
            pixel::pxl_fg('#', Color::Grey),
        );

        engine.line(
            0,
            1,
            0,
            screen_height as i32,
            pixel::pxl_fg('#', Color::Grey),
        );

        engine.line(
            screen_width as i32 + 1,
            1,
            screen_width as i32 + 1,
            screen_height as i32,
            pixel::pxl_fg('#', Color::Grey),
        );

        for (y, row) in last_buffer.iter().enumerate() {
            for x in 0..screen_width {
                if row >> (screen_width - x - 1) & 1 == 0 {
                    engine.set_pxl(x as i32 + 1, y as i32 + 1, pixel::pxl_fg(' ', Color::Cyan));
                } else {
                    engine.set_pxl(x as i32 + 1, y as i32 + 1, pixel::pxl_fg('X', Color::Cyan));
                }
            }
        }

        engine.print(0, screen_height as i32 + 2, "         ");
        engine.print(
            0,
            screen_height as i32 + 2,
            format!("FPS: {}", last_fps).as_str(),
        );

        engine.draw();

        if stopwatch.elapsed().as_millis() >= 1000 {
            last_fps = engine.frame_count;
            engine.frame_count = 0;
            stopwatch = std::time::Instant::now();
        }
    }
}

fn get_current_key(engine: &ConsoleEngine) -> Option<InputKey> {
    if engine.is_key_pressed(KeyCode::Char('0')) {
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
    }
}
