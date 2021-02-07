use std::convert::TryFrom;

#[derive(Debug)]
pub enum OpCode {
    Cls,
    Jump(u16),
}

impl TryFrom<[u8; 2]> for OpCode {
    type Error = String;

    fn try_from(bytes: [u8; 2]) -> Result<Self, Self::Error> {
        let bytes: u16 = ((bytes[0] as u16) << 8) | bytes[1] as u16;

        match bytes {
            0x00E0 => Ok(OpCode::Cls),
            // TODO: Is the first 1 part of the jump address
            location if (0x1000..=0x1FFF).contains(&location) => Ok(OpCode::Jump(location)),
            _ => Err(format!("Invalid OpCode {:#04x}", bytes)), // TODO: Show the bytes
        }
    }
}

#[cfg(test)]
mod tests {
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
}
