use std::ops::Deref;

#[derive(Default)]
pub struct FrameBuffer([u64; 32]);

impl FrameBuffer {
    pub fn clear(&mut self) {
        *self = Default::default();
    }
}

impl Deref for FrameBuffer {
    type Target = [u64; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
