use std::ops::Deref;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const SPRITE_WIDTH: usize = 8;

pub type FrameBufferMemory = [u64; DISPLAY_HEIGHT];

#[derive(Default)]
pub struct FrameBuffer(FrameBufferMemory);

impl FrameBuffer {
    pub fn draw(&mut self, (x, y): (u8, u8), sprite: &[u8]) -> bool {
        let mut turned_bit_off = false;

        let mut y = y as usize;

        for byte in sprite {
            let byte = ((*byte as u64) << (DISPLAY_WIDTH - SPRITE_WIDTH)).rotate_right(x as u32);
            let previous = self.0[y];
            let updated = previous ^ byte;

            if previous & !updated != 0 {
                turned_bit_off = true;
            }

            self.0[y] = updated;

            y += 1;
            y %= DISPLAY_HEIGHT;
        }

        turned_bit_off
    }

    pub fn clear(&mut self) {
        *self = Default::default();
    }

    pub fn get_screen_width(&self) -> usize {
        DISPLAY_WIDTH
    }

    pub fn get_screen_height(&self) -> usize {
        DISPLAY_HEIGHT
    }
}

impl Deref for FrameBuffer {
    type Target = [u64; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_single_byte_sprite() {
        let mut buffer = FrameBuffer::default();
        buffer.draw((1, 2), &[0b10011001]);
        assert_eq!(
            buffer.0[2],
            0b0100110010000000000000000000000000000000000000000000000000000000
        );
    }

    #[test]
    fn draw_multiple_byte_sprite() {
        let mut buffer = FrameBuffer::default();
        buffer.draw((1, 2), &[0b10011001, 0b01100110]);
        assert_eq!(
            &buffer.0[2..=3],
            &[
                0b0100110010000000000000000000000000000000000000000000000000000000,
                0b0011001100000000000000000000000000000000000000000000000000000000,
            ]
        );
    }

    #[test]
    fn draw_byte_wrapping_around_horizontally() {
        let mut buffer = FrameBuffer::default();
        buffer.draw((60, 2), &[0b10011001]);
        assert_eq!(
            buffer.0[2],
            0b1001000000000000000000000000000000000000000000000000000000001001,
        );
    }

    #[test]
    fn draw_sprite_wrapping_around_vertically() {
        let mut buffer = FrameBuffer::default();
        buffer.draw((0, 31), &[0b10011001, 0b01100110]);
        assert_eq!(
            buffer.0[31],
            0b1001100100000000000000000000000000000000000000000000000000000000,
        );
        assert_eq!(
            buffer.0[0],
            0b0110011000000000000000000000000000000000000000000000000000000000,
        );
    }

    #[test]
    fn returns_false_if_no_bit_was_flipped_off() {
        let mut buffer = FrameBuffer::default();
        assert!(!buffer.draw((0, 0), &[0b11111111]));
    }

    #[test]
    fn returns_true_if_some_bit_was_flipped_off() {
        let mut buffer = FrameBuffer::default();
        buffer.draw((0, 0), &[0b11111111]);
        assert!(buffer.draw((0, 0), &[0b11111110]));
    }
}
