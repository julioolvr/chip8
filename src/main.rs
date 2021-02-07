mod chip8;

use std::{fs::File, io::Read};

use chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();

    let rom = File::open("./roms/random_number.ch8").expect("Unable to open file");
    chip8.load(rom.bytes().map(|byte| byte.expect("Unable to read byte")));

    chip8.run();
    render(chip8.frame_buffer());
}

fn render(buffer: &[u64; 32]) {
    for row in buffer {
        for n in (0..64).rev() {
            print!("{}", if row >> n & 1 == 0 { "." } else { "X" });
        }

        println!("");
    }
}
