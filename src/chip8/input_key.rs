use super::InputInstruction;

pub enum InputKey {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

impl Into<u8> for InputKey {
    fn into(self) -> u8 {
        match self {
            InputKey::Key0 => 0x0,
            InputKey::Key1 => 0x1,
            InputKey::Key2 => 0x2,
            InputKey::Key3 => 0x3,
            InputKey::Key4 => 0x4,
            InputKey::Key5 => 0x5,
            InputKey::Key6 => 0x6,
            InputKey::Key7 => 0x7,
            InputKey::Key8 => 0x8,
            InputKey::Key9 => 0x9,
            InputKey::KeyA => 0xA,
            InputKey::KeyB => 0xB,
            InputKey::KeyC => 0xC,
            InputKey::KeyD => 0xD,
            InputKey::KeyE => 0xE,
            InputKey::KeyF => 0xF,
        }
    }
}

impl Into<InputInstruction> for InputKey {
    fn into(self) -> InputInstruction {
        InputInstruction::new(self)
    }
}
