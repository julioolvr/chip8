use rand::prelude::*;
use std::convert::TryFrom;

use super::FrameBuffer;
use super::Memory;
use super::OpCode;
use super::Registers;

#[derive(Default)]
pub struct Chip8 {
    memory: Memory,
    index: u16,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    frame_buffer: FrameBuffer,
    v_registers: Registers,
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
        let mut rng = thread_rng();

        while let Some(op_code) = self.next_opcode() {
            match op_code {
                OpCode::Cls => self.frame_buffer.clear(),
                OpCode::Jump(location) => self.program_counter = location,
                OpCode::Random(register, k) => {
                    self.v_registers.set(register, k & rng.gen_range(0..=0xFF))
                }
                OpCode::LoadIndex(value) => self.index = value,
                OpCode::LoadDecimal(register) => {
                    let value = self.v_registers.get(register);

                    let hundreds = value / 100;
                    let tens = value / 10 % 10;
                    let ones = value % 10;

                    self.memory.set(self.index, hundreds);
                    self.memory.set(self.index + 1, tens);
                    self.memory.set(self.index + 2, ones);
                }
            }
        }
    }

    fn next_opcode(&mut self) -> Option<OpCode> {
        let most_significant_byte = self.memory.get(self.program_counter);
        self.program_counter += 1;
        let least_significant_byte = self.memory.get(self.program_counter);
        self.program_counter += 1;

        if most_significant_byte == 0 && least_significant_byte == 0 {
            // Placeholder for now
            return None;
        }

        // TODO: Review how to crash on an invalid program
        Some(OpCode::try_from([most_significant_byte, least_significant_byte]).unwrap())
    }

    pub fn frame_buffer(&self) -> &FrameBuffer {
        &self.frame_buffer
    }
}

#[cfg(test)]
mod tests {
    use crate::chip8::registers::VRegister;

    use super::*;

    #[test]
    fn test_load_index() {
        let mut chip8 = Chip8::new();
        chip8.load(vec![0xA1, 0x23].into_iter());
        chip8.run();
        assert_eq!(chip8.index, 0x0123);
    }

    #[test]
    fn test_load_binary() {
        let mut chip8 = Chip8::new();
        chip8.v_registers.set(VRegister::V2, 234);
        chip8.load(vec![0xA0, 0xFB, 0xF2, 0x33].into_iter());
        chip8.run();
        assert_eq!(chip8.memory.get(0x0FB), 2);
        assert_eq!(chip8.memory.get(0x0FC), 3);
        assert_eq!(chip8.memory.get(0x0FD), 4);
    }
}
