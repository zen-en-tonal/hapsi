/// 1オクターブに何音属するかを表す。
/// 例えば、C -> C# -> D -> ... -> A# -> Bの12種。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Chroma(usize);

impl Chroma {
    /// 1オクターブに何音属すか。
    pub fn new(size: usize) -> Self {
        Self(size)
    }

    /// step番目の音は何か。
    ///
    /// Chromaは循環するため、stepが負の場合は基準から逆順の音を返す。
    /// ```rust
    /// # use hapsi::core::tone::Chroma;
    /// // [C, C#, ..., A#, B]の12種。
    /// let chroma = Chroma::new(12);
    ///
    /// // 0は基準音を返す。
    /// assert_eq!(chroma.tone(0).step(), 0);
    ///
    /// // 1は基準音の右となり。
    /// assert_eq!(chroma.tone(1).step(), 1);
    ///
    /// // -1は基準音の左となり。
    /// assert_eq!(chroma.tone(-1).step(), 11);
    ///
    /// // 12は基準音から一周。
    /// assert_eq!(chroma.tone(12).step(), 0);
    ///
    /// // -12は基準音から一周。
    /// assert_eq!(chroma.tone(-12).step(), 0);
    /// ```
    pub fn tone(&self, step: i32) -> Tone {
        let step = step % self.size() as i32;
        if step < 0 {
            return self.tone(self.size() as i32 + step);
        }
        Tone::new(step as usize, self.size())
    }

    pub fn size(&self) -> usize {
        self.0
    }

    pub fn tones(&self) -> Vec<Tone> {
        self.tones_with_start(&Tone::new(0, self.size()))
    }

    pub fn tones_with_start(&self, start: &Tone) -> Vec<Tone> {
        (start.step()..start.step() + self.size())
            .into_iter()
            .map(|i| self.tone(i as i32))
            .collect()
    }

    pub fn distance(&self, from: &Tone, to: &Tone) -> usize {
        match to >= from {
            true => to.step() - from.step(),
            false => to.step() + self.size() - from.step(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Tone {
    step: usize,
    chroma_size: usize,
}

impl Tone {
    fn new(step: usize, chroma_size: usize) -> Self {
        Self { step, chroma_size }
    }

    pub fn step(&self) -> usize {
        self.step
    }

    pub fn chroma_size(&self) -> usize {
        self.chroma_size
    }

    pub fn rate(&self) -> f32 {
        self.step() as f32 / self.chroma_size() as f32
    }
}

#[cfg(test)]
mod tests {}
