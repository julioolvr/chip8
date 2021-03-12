use super::{FrameBufferMemory, InputKey};

pub struct DrawInstruction(FrameBufferMemory);

impl DrawInstruction {
    pub fn new(buffer: FrameBufferMemory) -> DrawInstruction {
        DrawInstruction(buffer)
    }

    pub fn buffer(&self) -> &FrameBufferMemory {
        &self.0
    }
}

pub struct InputInstruction(InputKey);

impl InputInstruction {
    pub fn new(input_key: InputKey) -> InputInstruction {
        InputInstruction(input_key)
    }
}

impl Into<InputKey> for InputInstruction {
    fn into(self) -> InputKey {
        self.0
    }
}
