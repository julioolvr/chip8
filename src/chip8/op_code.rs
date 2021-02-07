use std::convert::TryFrom;

use super::registers::VRegister;

#[derive(Debug)]
pub enum OpCode {
    Cls,
    Jump(u16),
    LoadIndex(u16),
    Random(VRegister, u8),
    LoadDecimal(VRegister),
}

impl TryFrom<[u8; 2]> for OpCode {
    type Error = String;

    fn try_from(bytes: [u8; 2]) -> Result<Self, Self::Error> {
        match bytes {
            [0x00, 0xE0] => Ok(OpCode::Cls),
            [msb, _] if (0x10..=0x1F).contains(&msb) => Ok(OpCode::Jump(pack_u8(bytes))),
            [msb, _] if (0xA0..=0xAF).contains(&msb) => {
                Ok(OpCode::LoadIndex(pack_u8(bytes) & 0x0FFF))
            }
            [msb, lsb] if (0xC0..=0xCF).contains(&msb) => {
                let register = VRegister::try_from(msb & 0xF).unwrap();
                Ok(OpCode::Random(register, lsb))
            }
            [msb, 0x33] if (0xF0..=0xFF).contains(&msb) => {
                let register = VRegister::try_from(msb & 0xF).unwrap();
                Ok(OpCode::LoadDecimal(register))
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
}
