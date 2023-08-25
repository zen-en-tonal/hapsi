use super::tone::ToneLike;

/// 音高
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pitch<T> {
    tone: T,
    oct: i32,
}

impl<T> Pitch<T> {
    pub fn new(tone: T, oct: i32) -> Self {
        Self { tone, oct }
    }

    pub fn tone(&self) -> &T {
        &self.tone
    }

    pub fn oct(&self) -> i32 {
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
