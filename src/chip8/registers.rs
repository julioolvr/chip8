use std::convert::TryFrom;

#[derive(Default)]
pub struct Registers([u8; 16]);

impl Registers {
    pub fn get(&self, register: VRegister) -> u8 {
        let index: usize = register.into();
        self.0[index]
    }

    pub fn set(&mut self, register: VRegister, value: u8) {
        let index: usize = register.into();
        self.0[index] = value;
    }
}

#[derive(Debug)]
pub enum VRegister {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl TryFrom<u8> for VRegister {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(VRegister::V0),
            0x1 => Ok(VRegister::V1),
            0x2 => Ok(VRegister::V2),
            0x3 => Ok(VRegister::V3),
            0x4 => Ok(VRegister::V4),
            0x5 => Ok(VRegister::V5),
            0x6 => Ok(VRegister::V6),
            0x7 => Ok(VRegister::V7),
            0x8 => Ok(VRegister::V8),
            0x9 => Ok(VRegister::V9),
            0xA => Ok(VRegister::VA),
            0xB => Ok(VRegister::VB),
            0xC => Ok(VRegister::VC),
            0xD => Ok(VRegister::VD),
            0xE => Ok(VRegister::VE),
            0xF => Ok(VRegister::VF),
            _ => Err(()),
        }
    }
}

impl From<VRegister> for usize {
    fn from(register: VRegister) -> Self {
        use VRegister::*;

        match register {
            V0 => 0x0,
            V1 => 0x1,
            V2 => 0x2,
            V3 => 0x3,
            V4 => 0x4,
            V5 => 0x5,
            V6 => 0x6,
            V7 => 0x7,
            V8 => 0x8,
            V9 => 0x9,
            VA => 0xA,
            VB => 0xB,
            VC => 0xC,
            VD => 0xD,
            VE => 0xE,
            VF => 0xF,
        }
    }
}
