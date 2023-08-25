pub trait ChromaLike {
    type Tone: ToneLike;

    fn tone(&self, step: i32) -> Self::Tone;

    fn size(&self) -> usize;

    fn distance(&self, from: &Self::Tone, to: &Self::Tone) -> usize;

    fn tones_with_start(&self, start: &Self::Tone) -> Vec<Self::Tone> {
        (start.step()..start.step() + self.size())
            .into_iter()
            .map(|i| self.tone(i as i32))
            .collect()
    }
}

pub trait ToneLike: PartialEq {
    fn step(&self) -> usize;

    fn chroma_size(&self) -> usize;
}
