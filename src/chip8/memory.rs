pub struct Memory([u8; 0x1000]);

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
        // TODO: This should include the font
        Memory([0; 0x1000])
    }
}
