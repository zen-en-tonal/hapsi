use super::tone::ToneLike;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pitch<T> {
    tone: T,
    oct: usize,
}

impl<T> Pitch<T> {
    pub fn new(tone: T, oct: usize) -> Self {
        Self { tone, oct }
    }

    pub fn tone(&self) -> &T {
        &self.tone
    }

    pub fn oct(&self) -> usize {
        self.oct
    }
}

impl<T: ToneLike> Pitch<T> {
    pub fn frequency(&self, ref_freq: f32) -> f32 {
        ref_freq
            * self.oct() as f32
            * 2.0_f32.powf(self.tone().step() as f32 / self.tone().chroma_size() as f32)
    }
}
