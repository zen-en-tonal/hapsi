use super::{
    pitch::Pitch,
    tone::{Chroma, Tone},
};

pub trait Transpose: Sized {
    fn transpose(self, transpose: i32) -> Self;
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
    /// assert_eq!(
    ///     pitch.clone().transpose(-24),
    ///     Pitch::new(chroma.tone(0), -2)
    /// );
    /// ```
    fn transpose(self, transpose: i32) -> Self {
        if transpose >= 0 {
            let oct = transpose / self.tone().chroma_size() as i32;
            Pitch::new(self.tone().transpose(transpose), self.oct() + oct)
        } else {
            let oct = transpose / self.tone().chroma_size() as i32;
            let oct = if oct == 0 { -1 } else { oct };
            Pitch::new(self.tone().transpose(transpose), self.oct() + oct)
        }
    }
}

impl<T: From<Tone> + Into<Tone>> Transpose for T {
    fn transpose(self, transpose: i32) -> Self {
        let tone: Tone = self.into();
        Chroma::new(tone.chroma_size())
            .tone(tone.step() as i32 + transpose)
            .into()
    }
}
