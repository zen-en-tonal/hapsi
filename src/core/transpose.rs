use super::{
    pitch::Pitch,
    tone::{Chroma, Tone},
};

pub trait Transpose: Sized {
    fn transpose(self, transpose: i32) -> Self;
}

impl Transpose for Tone {
    fn transpose(self, transpose: i32) -> Self {
        Chroma::new(self.chroma_size()).tone(self.step() as i32 + transpose)
    }
}

impl Transpose for Pitch {
    /// 音高を上下させる。
    /// ```rust
    /// # use hapsi::core::pitch::Pitch;
    /// # use hapsi::core::tone::Chroma;
    /// # use crate::hapsi::core::transpose::Transpose;
    ///
    /// // C0
    /// let chroma = Chroma::new(12);
    /// let pitch = Pitch::new(chroma.tone(0), 0);
    /// assert_eq!(
    ///     pitch.clone().transpose(12),
    ///     Pitch::new(chroma.tone(0), 1)
    /// );
    /// assert_eq!(
    ///     pitch.clone().transpose(-12),
    ///     Pitch::new(chroma.tone(0), -1)
    /// );
    /// assert_eq!(
    ///     pitch.clone().transpose(5),
    ///     Pitch::new(chroma.tone(5), 0)
    /// );
    /// assert_eq!(
    ///     pitch.clone().transpose(-5),
    ///     Pitch::new(chroma.tone(7), -1)
    /// );
    /// ```
    fn transpose(self, transpose: i32) -> Self {
        let oct = transpose / self.tone().chroma_size() as i32;
        Pitch::new(self.tone().transpose(transpose), self.oct() + oct)
    }
}
