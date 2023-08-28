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

    /// Returns a distance as `Degree`.
    /// - If `from` either `to` are not on this scale, returns `None`.
    fn get_distance(&self, from: &Self::ToneLike, to: &Self::ToneLike) -> Option<Degree> {
        let from_interval = self.chroma().get_interval(self.key(), from);
        let to_interval = self.chroma().get_interval(self.key(), to);
        let iter = self.intervals().iter();
        match (
            iter.clone().position(|&i| i == from_interval),
            iter.clone().position(|&i| i == to_interval),
        ) {
            (Some(from), Some(to)) => Some(if from <= to {
                Degree::new(to - from + 1).unwrap()
            } else {
                Degree::new(to + self.intervals().len() - from + 1).unwrap()
            }),
            (_, _) => None,
        }
    }

    /// Returns a distance from `self.key()`.
    /// - if `to` is not on this scale, returns `None`.
    fn get_distance_from_key(&self, to: &Self::ToneLike) -> Option<Degree> {
        self.get_distance(self.key(), to)
    }

    /// Returns a `Tone` by `Degree` that represents a distance.
    /// - if `from` is not on this scale, returns `None`.
    fn get_tone_by_degree(
        &self,
        from: &Self::ToneLike,
        degree: &Degree,
    ) -> Option<&Self::ToneLike> {
        let Some(interval) = self.degree_to_interval(from, degree) else {
            return None;
        };
        Some(self.chroma().get((from.step() + interval.value()) as i32))
    }

    /// Returns a `Tone` by `Degree` that represents a distance from `self.key()`.
    fn get_tone_by_degree_from_key(&self, degree: &Degree) -> &Self::ToneLike {
        self.get_tone_by_degree(self.key(), degree).unwrap()
    }

    fn degree_to_interval(&self, from: &Self::ToneLike, degree: &Degree) -> Option<&Interval> {
        let interval_from_key = self.chroma().get_interval(self.key(), from);
        let Some(index) = self.intervals().iter().position(|&i| i == interval_from_key) else {
            return None;
        };
        self.intervals()
            .get((index + degree.value() - 1) % self.intervals().len())
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
