use rand::prelude::*;
use std::{
    convert::TryFrom,
    sync::mpsc::{Receiver, Sender},
};

use super::{
    all_registers,
    io::{DrawInstruction, InputInstruction},
    registers::VRegister,
    FrameBuffer, InputKey, Memory, OpCode, Registers,
};

pub struct Chip8 {
    memory: Memory,
    index: u16,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    frame_buffer: FrameBuffer,
    v_registers: Registers,
    display_tx: Sender<DrawInstruction>,
    input_rx: Receiver<InputInstruction>,
}

impl Chip8 {
    pub fn new(display_tx: Sender<DrawInstruction>, input_rx: Receiver<InputInstruction>) -> Chip8 {
        Chip8 {
            memory: Memory::default(),
            index: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            stack: [0; 16],
            frame_buffer: FrameBuffer::default(),
            v_registers: Registers::default(),
            display_tx,
            input_rx,
        }
    }

    pub fn load(&mut self, rom: impl IntoIterator<Item = u8>) {
        for (offset, byte) in rom.into_iter().enumerate() {
            self.memory.set(0x200 + (offset as u16), byte);
        }
    }

    pub fn run(&mut self) {
        while self.run_instruction(None) {}
    }

    pub fn run_instruction(&mut self, current_input: Option<InputKey>) -> bool {
        let mut rng = thread_rng();

        match self.next_opcode() {
            Some(op_code) => {
                trace!("Op: {:?}", op_code);

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
                    OpCode::Fill(end) => {
                        for register in all_registers().take_while(|register| register <= &end) {
                            self.v_registers.set(register, self.memory.get(self.index));
                            self.index += 1;
                        }
                    }
                    OpCode::LoadCharacter(register) => {
                        let value = self.v_registers.get(register);
                        self.index = self.memory.index_of_char(value);
                    }
                    OpCode::SetRegister(register, value) => self.v_registers.set(register, value),
                    OpCode::Draw((x, y), length) => {
                        let x = self.v_registers.get(x);
                        let y = self.v_registers.get(y);
                        let sprite = self.memory.range(self.index..self.index + length as u16);
                        let turned_bit_off = self.frame_buffer.draw((x, y), sprite);

                        if turned_bit_off {
                            self.v_registers.set(VRegister::VF, 1);
                        } else {
                            self.v_registers.set(VRegister::VF, 0);
                        }

                        self.display_tx
                            .send(DrawInstruction::new(self.frame_buffer.clone()))
                            .unwrap();
                    }
                    OpCode::WaitForKeyPress(register) => {
                        let instruction = self.input_rx.recv().unwrap();
                        let input: InputKey = instruction.into();
                        self.v_registers.set(register, input.into());
                    }
                }

                true
            }
            None => false,
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

    pub fn get_screen_width(&self) -> usize {
        self.frame_buffer().get_screen_width()
    }

    pub fn get_screen_height(&self) -> usize {
        self.frame_buffer().get_screen_height()
    }
}

#[cfg(test)]
mod tests {
    use crate::chip8::registers::VRegister;

    use super::*;

    fn build_chip() -> Chip8 {
        use std::sync::mpsc::channel;

        let (tx, _) = channel();
        let (_, rx) = channel();

        Chip8::new(tx, rx)
    }

    #[test]
    fn test_load_index() {
        let mut chip8 = build_chip();
        chip8.load(vec![0xA1, 0x23].into_iter());
        chip8.run();
        assert_eq!(chip8.index, 0x0123);
    }

    #[test]
    fn test_load_binary() {
        let mut chip8 = build_chip();
        chip8.v_registers.set(VRegister::V2, 234);
        chip8.load(vec![0xA0, 0xFB, 0xF2, 0x33].into_iter());
        chip8.run();
        assert_eq!(chip8.memory.get(0x0FB), 2);
        assert_eq!(chip8.memory.get(0x0FC), 3);
        assert_eq!(chip8.memory.get(0x0FD), 4);
    }

    #[test]
    fn test_fill_registers() {
        let mut chip8 = build_chip();
        chip8.index = 0x300;
        chip8.memory.set(0x300, 12);
        chip8.memory.set(0x301, 34);
        chip8.memory.set(0x302, 56);
        chip8.memory.set(0x303, 78);

        chip8.load(vec![0xF2, 0x65].into_iter());
        chip8.run();

        assert_eq!(chip8.index, 0x303);
        assert_eq!(chip8.v_registers.get(VRegister::V0), 12);
        assert_eq!(chip8.v_registers.get(VRegister::V1), 34);
        assert_eq!(chip8.v_registers.get(VRegister::V2), 56);
        assert_eq!(chip8.v_registers.get(VRegister::V3), 0);
    }

    #[test]
    fn test_load_character() {
        let mut chip8 = build_chip();
        chip8.v_registers.set(VRegister::V2, 0xB);
        chip8.load(vec![0xF2, 0x29].into_iter());
        chip8.run();
        // This test is tightly coupled to the memory addresses currently in use for the font
        assert_eq!(chip8.index, 55);
    }

    #[test]
    fn test_set_register() {
        let mut chip8 = build_chip();
        chip8.load(vec![0x62, 0x29].into_iter());
        chip8.run();
        assert_eq!(chip8.v_registers.get(VRegister::V2), 0x29);
    }
}
