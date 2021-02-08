mod chip8;
pub use chip8::Chip8;

mod frame_buffer;
mod memory;
mod op_code;
mod registers;

use frame_buffer::FrameBuffer;
use memory::Memory;
use op_code::OpCode;
use registers::{all_registers, Registers};
