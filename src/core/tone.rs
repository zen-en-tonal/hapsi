use super::distance::Interval;

pub trait ChromaLike: Sized {
    type Tone: ToneLike;

    /// Retunes a `ToneLike` that places in given index.
    /// - Returns `None` if given index is out of bounds.
    fn get_exactly(&self, index: usize) -> Option<&Self::Tone>;

    /// Returns a size of this chroma.
    fn size(&self) -> usize;

    /// Returns a `ToneLike` that places in given index.
    /// - If `index > size`, `index` will be `index % size`.
    /// - If `index < 0`, `index` will be `size + index % size`.
    fn get(&self, index: i32) -> &Self::Tone {
        let index = index % self.size() as i32;
        if index >= 0 {
            self.get_exactly(index as usize).unwrap()
        } else {
            self.get_exactly(self.size() + index as usize).unwrap()
        }
    }

    fn tones_with_start(&self, start: &Self::Tone) -> ToneIter<'_, Self> {
        let offset = start.step();
        ToneIter::new(self, offset)
    }

    fn tones(&self) -> ToneIter<'_, Self> {
        ToneIter::new(self, 0)
    }

    /// Returns an `Interval` of `from` and `to`.
    fn get_interval(&self, from: &Self::Tone, to: &Self::Tone) -> Interval {
        if from.step() <= to.step() {
            Interval::new(from.step(), to.step())
        } else {
            Interval::new(from.step(), to.step() + self.size())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ToneIter<'a, C> {
    chroma: &'a C,
    index: usize,
    offset: usize,
}

impl<'a, C> ToneIter<'a, C> {
    pub fn new(chroma: &'a C, offset: usize) -> Self {
        Self {
            chroma,
            index: 0,
            offset,
        }
    }
}

impl<'a, C: ChromaLike> Iterator for ToneIter<'a, C> {
    type Item = &'a C::Tone;

    fn next(&mut self) -> Option<Self::Item> {
        let item = if self.index < self.chroma.size() {
            let step = self.offset + self.index;
            Some(self.chroma.get(step as i32))
        } else {
            None
        };
        self.index += 1;
        item
    }
}

pub trait ToneLike: PartialEq {
    fn step(&self) -> usize;

    fn chroma_size(&self) -> usize;
}
