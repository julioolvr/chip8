mod chip8;
pub use self::chip8::Chip8;
pub use self::input_key::InputKey;

mod frame_buffer;
mod input_key;
mod memory;
mod op_code;
mod registers;

use self::frame_buffer::FrameBuffer;
use self::memory::Memory;
use self::op_code::OpCode;
use self::registers::{all_registers, Registers};
