use std::convert::TryFrom;

use super::registers::VRegister;

#[derive(Debug)]
pub enum OpCode {
    Cls,
    Jump(u16),
    LoadIndex(u16),
    Random(VRegister, u8),
}

impl TryFrom<[u8; 2]> for OpCode {
    type Error = String;

    fn try_from(bytes: [u8; 2]) -> Result<Self, Self::Error> {
        let bytes: u16 = ((bytes[0] as u16) << 8) | bytes[1] as u16;

        match bytes {
            0x00E0 => Ok(OpCode::Cls),
            // TODO: Is the first 1 part of the jump address?
            0x1000..=0x1FFF => Ok(OpCode::Jump(bytes)),
            0xA000..=0xAFFF => Ok(OpCode::LoadIndex(bytes & 0x0FFF)),
            0xC000..=0xCFFF => {
                let register = VRegister::try_from((bytes >> 8 & 0xF) as u8);
                let k = bytes as u8;
                Ok(OpCode::Random(register.unwrap(), k))
            }
            _ => Err(format!("Invalid OpCode {:#04x}", bytes)),
        }
    }
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
        println!("{:?}", op_code);
        assert!(matches!(op_code, OpCode::Random(VRegister::V2, 0x12)));
    }
}
