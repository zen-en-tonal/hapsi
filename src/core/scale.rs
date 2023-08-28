use super::{
    distance::{Degree, Interval},
    tone::{ChromaLike, ToneLike},
};

pub trait ScaleLike: Sized {
    type ToneLike: ToneLike;
    type ChromaLike: ChromaLike<Tone = Self::ToneLike>;

    /// Returns a key tone of this scale.
    fn key(&self) -> &Self::ToneLike;

    /// Returns an array of all possible intervals from key.
    fn intervals(&self) -> &[Interval];

    /// Returns a `Chroma` that this scale depends on.
    fn chroma(&self) -> &Self::ChromaLike;

    /// Returns an Iterator that enumerates tones which is belongs to this scale.
    fn tones(&self) -> ToneIter<'_, Self> {
        ToneIter::new(self)
    }

    /// Returns a condition what `tone` is belongs to this scale or not.
    fn is_on_scale(&self, tone: &Self::ToneLike) -> bool {
        self.tones().any(|t| t == tone)
    }

    /// Returns a distance of `key` to `other` as `Degree`.
    /// - If `other` is not on this scale, returns `None`.
    fn get_distance(&self, other: &Self::ToneLike) -> Option<Degree> {
        let interval = self.chroma().get_interval(self.key(), other);
        self.intervals()
            .iter()
            .position(|&i| i == interval)
            .map(|i| Degree::new(i + 1))
    }

    /// Returns a `Tone` by `Degree` that represents a distance from the `self.key()`.
    fn get_tone_by_degree(&self, degree: &Degree) -> &Self::ToneLike {
        let interval = self
            .intervals()
            .get((degree.value() - 1) % self.intervals().len())
            .unwrap();
        self.chroma()
            .get((self.key().step() + interval.value()) as i32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ToneIter<'a, S> {
    scale: &'a S,
    index: usize,
}

impl<'a, S> ToneIter<'a, S> {
    pub fn new(scale: &'a S) -> Self {
        Self { scale, index: 0 }
    }
}

impl<'a, S: ScaleLike> Iterator for ToneIter<'a, S> {
    type Item = &'a S::ToneLike;

    fn next(&mut self) -> Option<Self::Item> {
        let tone = if self.index < self.scale.intervals().len() {
            let interval = self.scale.intervals().get(self.index).unwrap();
            let index = self.scale.key().step() + interval.value();
            Some(self.scale.chroma().get(index as i32))
        } else {
            None
        };
        self.index += 1;
        tone
    }
}
