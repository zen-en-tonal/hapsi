use super::tone::Tone;

/// 音高
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pitch {
    tone: Tone,
    oct: i32,
}

impl Pitch {
    pub fn new(tone: Tone, oct: i32) -> Self {
        Self { tone, oct }
    }

    pub fn tone(&self) -> &Tone {
        &self.tone
    }

    pub fn oct(&self) -> i32 {
        self.oct
    }

    pub fn frequency(&self, ref_freq: f32) -> f32 {
        ref_freq * self.oct() as f32 * 2.0_f32.powf(self.tone().rate())
    }
}
