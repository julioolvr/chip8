pub struct Memory([u8; 0x1000]);

const FONT_BASE: u16 = 0;

impl Memory {
    pub fn get(&self, index: u16) -> u8 {
        self.0[index as usize]
    }

    pub fn set(&mut self, index: u16, value: u8) {
        self.0[index as usize] = value;
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut memory = Memory([0; 0x1000]);

        // Set the font in memory
        // 0
        memory.set(FONT_BASE + 0, 0b01100000);
        memory.set(FONT_BASE + 1, 0b10010000);
        memory.set(FONT_BASE + 2, 0b10010000);
        memory.set(FONT_BASE + 3, 0b10010000);
        memory.set(FONT_BASE + 4, 0b01100000);

        // 1
        memory.set(FONT_BASE + 5, 0b01100000);
        memory.set(FONT_BASE + 6, 0b00100000);
        memory.set(FONT_BASE + 7, 0b00100000);
        memory.set(FONT_BASE + 8, 0b00100000);
        memory.set(FONT_BASE + 9, 0b01110000);

        // 2
        memory.set(FONT_BASE + 10, 0b11100000);
        memory.set(FONT_BASE + 11, 0b00010000);
        memory.set(FONT_BASE + 12, 0b00110000);
        memory.set(FONT_BASE + 13, 0b01100000);
        memory.set(FONT_BASE + 14, 0b11110000);

        // 3
        memory.set(FONT_BASE + 15, 0b11100000);
        memory.set(FONT_BASE + 16, 0b00010000);
        memory.set(FONT_BASE + 17, 0b01100000);
        memory.set(FONT_BASE + 18, 0b00010000);
        memory.set(FONT_BASE + 19, 0b11100000);

        // 4
        memory.set(FONT_BASE + 20, 0b10010000);
        memory.set(FONT_BASE + 21, 0b10010000);
        memory.set(FONT_BASE + 22, 0b11110000);
        memory.set(FONT_BASE + 23, 0b00010000);
        memory.set(FONT_BASE + 24, 0b00010000);

        // 5
        memory.set(FONT_BASE + 25, 0b11110000);
        memory.set(FONT_BASE + 26, 0b10000000);
        memory.set(FONT_BASE + 27, 0b11110000);
        memory.set(FONT_BASE + 28, 0b00010000);
        memory.set(FONT_BASE + 29, 0b11110000);

        // 6
        memory.set(FONT_BASE + 30, 0b11110000);
        memory.set(FONT_BASE + 31, 0b10000000);
        memory.set(FONT_BASE + 32, 0b11110000);
        memory.set(FONT_BASE + 33, 0b10010000);
        memory.set(FONT_BASE + 34, 0b11110000);

        // 7
        memory.set(FONT_BASE + 35, 0b11110000);
        memory.set(FONT_BASE + 36, 0b00010000);
        memory.set(FONT_BASE + 37, 0b00100000);
        memory.set(FONT_BASE + 38, 0b01000000);
        memory.set(FONT_BASE + 39, 0b01000000);

        // 8
        memory.set(FONT_BASE + 40, 0b11110000);
        memory.set(FONT_BASE + 41, 0b10010000);
        memory.set(FONT_BASE + 42, 0b11110000);
        memory.set(FONT_BASE + 43, 0b10010000);
        memory.set(FONT_BASE + 44, 0b11110000);

        // 9
        memory.set(FONT_BASE + 45, 0b11110000);
        memory.set(FONT_BASE + 46, 0b10010000);
        memory.set(FONT_BASE + 47, 0b11110000);
        memory.set(FONT_BASE + 48, 0b00010000);
        memory.set(FONT_BASE + 49, 0b11110000);

        // A
        memory.set(FONT_BASE + 50, 0b11110000);
        memory.set(FONT_BASE + 51, 0b10010000);
        memory.set(FONT_BASE + 52, 0b11110000);
        memory.set(FONT_BASE + 53, 0b10010000);
        memory.set(FONT_BASE + 54, 0b10010000);

        // B
        memory.set(FONT_BASE + 55, 0b11100000);
        memory.set(FONT_BASE + 56, 0b10010000);
        memory.set(FONT_BASE + 57, 0b11100000);
        memory.set(FONT_BASE + 58, 0b10010000);
        memory.set(FONT_BASE + 59, 0b11100000);

        // C
        memory.set(FONT_BASE + 60, 0b11110000);
        memory.set(FONT_BASE + 61, 0b10000000);
        memory.set(FONT_BASE + 62, 0b10000000);
        memory.set(FONT_BASE + 63, 0b10000000);
        memory.set(FONT_BASE + 64, 0b11110000);

        // D
        memory.set(FONT_BASE + 65, 0b11100000);
        memory.set(FONT_BASE + 66, 0b10010000);
        memory.set(FONT_BASE + 67, 0b10010000);
        memory.set(FONT_BASE + 68, 0b10010000);
        memory.set(FONT_BASE + 69, 0b11100000);

        // E
        memory.set(FONT_BASE + 70, 0b11110000);
        memory.set(FONT_BASE + 71, 0b10000000);
        memory.set(FONT_BASE + 72, 0b11110000);
        memory.set(FONT_BASE + 73, 0b10000000);
        memory.set(FONT_BASE + 74, 0b11110000);

        // F
        memory.set(FONT_BASE + 75, 0b11110000);
        memory.set(FONT_BASE + 76, 0b10000000);
        memory.set(FONT_BASE + 77, 0b11100000);
        memory.set(FONT_BASE + 78, 0b10000000);
        memory.set(FONT_BASE + 79, 0b10000000);

        memory
    }
}
