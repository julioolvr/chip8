use std::convert::TryFrom;

use super::registers::VRegister;

#[derive(Debug)]
pub enum OpCode {
    /// Clear the display (`00E0`)
    Cls,

    /// Set value of V register (`6XNN`)
    SetRegister(VRegister, u8),

    /// Set value of index register (`Annn`)
    LoadIndex(u16),

    /// Jump to location (`Bnnn`)
    Jump(u16),

    /// Set a register to a random value (`Cxkk`)
    ///
    /// The randomly generated value is and-ed with the given `kk` value.
    Random(VRegister, u8),

    /// Draw a sprite to the screen (`Dxyn`)
    ///
    /// Draws an n-bytes sprite at position (x, y). The sprite is read from the memory address
    /// pointed at by the index register. Sprites are XORed onto the screen. If a pixel goes from
    /// to unset, VF is set to 1, otherwise it is set to 0. Sprites wrap around if they go through
    /// the edge of the screen.
    Draw((VRegister, VRegister), u8),

    /// Wait for key press and store the value of the key in the given V register (`Fx0A`)
    WaitForKeyPress(VRegister),

    /// Point index to the character corresponding to the value of the given register (`Fx29`)
    LoadCharacter(VRegister),

    /// Load decimal representation in memory (`Fx33`)
    ///
    /// Takes the decimal representation of the value in the given register and stores it in memory,
    /// starting at the address pointed at by the index register for the hundreds, the next one for
    /// the tens and the following one for the ones.
    LoadDecimal(VRegister),

    /// Loads data from memory to V registers up to the given one (`Fx65`)
    ///
    /// Load values from memory, starting at the address pointed at by the index register, to the V
    /// registers up until the given one.
    Fill(VRegister),
}

impl TryFrom<[u8; 2]> for OpCode {
    type Error = String;

    fn try_from(bytes: [u8; 2]) -> Result<Self, Self::Error> {
        match bytes {
            // OpCode::Cls
            [0x00, 0xE0] => Ok(OpCode::Cls),

            // OpCode::Jump
            [msb, _] if (0x10..=0x1F).contains(&msb) => Ok(OpCode::Jump(pack_u8(bytes))),

            // OpCode::LoadIndex
            [msb, _] if (0xA0..=0xAF).contains(&msb) => {
                Ok(OpCode::LoadIndex(pack_u8(bytes) & 0x0FFF))
            }

            // OpCode::Random
            [msb, lsb] if (0xC0..=0xCF).contains(&msb) => {
                let register = VRegister::try_from(msb & 0x0F).unwrap();
                Ok(OpCode::Random(register, lsb))
            }

            // OpCode::LoadCharacter
            [msb, 0x29] if (0xF0..=0xFF).contains(&msb) => {
                let register = VRegister::try_from(msb & 0x0F).unwrap();
                Ok(OpCode::LoadCharacter(register))
            }

            // OpCode::LoadDecimal
            [msb, 0x33] if (0xF0..=0xFF).contains(&msb) => {
                let register = VRegister::try_from(msb & 0x0F).unwrap();
                Ok(OpCode::LoadDecimal(register))
            }

            // OpCode::Fill
            [msb, 0x65] if (0xF0..=0xFF).contains(&msb) => {
                let register = VRegister::try_from(msb & 0x0F).unwrap();
                Ok(OpCode::Fill(register))
            }

            // OpCode::SetRegister
            [msb, lsb] if (0x60..=0x6F).contains(&msb) => {
                let register = VRegister::try_from(msb & 0x0F).unwrap();
                Ok(OpCode::SetRegister(register, lsb))
            }

            // OpCode::Draw
            [msb, lsb] if (0xD0..=0xDF).contains(&msb) => {
                let register_x = VRegister::try_from(msb & 0x0F).unwrap();
                let register_y = VRegister::try_from((lsb & 0xF0) >> 4).unwrap();
                let sprite_length = lsb & 0x0F;

                Ok(OpCode::Draw((register_x, register_y), sprite_length))
            }

            // OpCode::WaitForKeyPress
            [msb, 0x0A] if (0xF0..=0xFF).contains(&msb) => {
                let register = VRegister::try_from(msb & 0x0F).unwrap();
                Ok(OpCode::WaitForKeyPress(register))
            }

            _ => Err(format!("Invalid OpCode {:#02x}{:02x}", bytes[0], bytes[1])),
        }
    }
}

fn pack_u8(value: [u8; 2]) -> u16 {
    ((value[0] as u16) << 8) | value[1] as u16
}

#[cfg(test)]
mod tests {
    use super::super::registers::VRegister;
    use super::*;

    #[test]
    fn parse_cls_code() {
        let op_code = OpCode::try_from([0x00, 0xE0]).unwrap();
        assert!(matches!(op_code, OpCode::Cls));
    }

    #[test]
    fn parse_jump() {
        let op_code = OpCode::try_from([0x10, 0xAA]).unwrap();
        assert!(matches!(op_code, OpCode::Jump(0x10AA)));
    }

    #[test]
    fn parse_load_index() {
        let op_code = OpCode::try_from([0xAB, 0xCD]).unwrap();
        assert!(matches!(op_code, OpCode::LoadIndex(0x0BCD)));
    }

    #[test]
    fn parse_random_byte() {
        let op_code = OpCode::try_from([0xc2, 0x12]).unwrap();
        assert!(matches!(op_code, OpCode::Random(VRegister::V2, 0x12)));
    }

    #[test]
    fn parse_load_decimal() {
        let op_code = OpCode::try_from([0xf2, 0x33]).unwrap();
        assert!(matches!(op_code, OpCode::LoadDecimal(VRegister::V2)));
    }

    #[test]
    fn parse_draw_sprite() {
        let op_code = OpCode::try_from([0xd4, 0x58]).unwrap();
        assert!(matches!(
            op_code,
            OpCode::Draw((VRegister::V4, VRegister::V5), 0x8)
        ));
    }
}
