use std::convert::TryFrom;

use super::FrameBuffer;
use super::Memory;
use super::OpCode;

#[derive(Default)]
pub struct Chip8 {
    memory: Memory,
    index: u16,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    frame_buffer: FrameBuffer,
    v_0: u8,
    v_1: u8,
    v_2: u8,
    v_3: u8,
    v_4: u8,
    v_5: u8,
    v_6: u8,
    v_7: u8,
    v_8: u8,
    v_9: u8,
    v_a: u8,
    v_b: u8,
    v_c: u8,
    v_d: u8,
    v_e: u8,
    v_f: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            program_counter: 0x200,
            ..Default::default()
        }
    }

    pub fn load(&mut self, rom: impl IntoIterator<Item = u8>) {
        for (offset, byte) in rom.into_iter().enumerate() {
            self.memory.set(0x200 + (offset as u16), byte);
        }
    }

    pub fn run(&mut self) {
        println!("Running");
        while let Some(op_code) = self.next_opcode() {
            match op_code {
                OpCode::Cls => self.frame_buffer.clear(),
                OpCode::Jump(location) => self.program_counter = location,
            }
        }
    }

    fn next_opcode(&mut self) -> Option<OpCode> {
        let most_significant_byte = self.memory.get(self.program_counter);
        self.program_counter += 1;
        let least_significant_byte = self.memory.get(self.program_counter);
        self.program_counter += 1;

        // TODO: Review how to crash on an invalid program
        Some(OpCode::try_from([most_significant_byte, least_significant_byte]).unwrap())
    }

    pub fn frame_buffer(&self) -> &FrameBuffer {
        &self.frame_buffer
    }
}
