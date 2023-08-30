use super::{distance::Interval, number::Number, Pitch};

pub trait Keyboard: Sized {
    type PitchClass;

    /// Retunes a `ToneLike` that places in given index.
    /// - Returns `None` if given index is out of bounds.
    fn get_class<T: Number>(&self, number: &T) -> Option<&Self::PitchClass>;

    fn get_class_cycly(&self, index: usize) -> &Self::PitchClass {
        self.get_class(index % self.size()).unwrap()
    }

    /// Returns a size of this chroma.
    fn size(&self) -> usize;

    /// Returns a `ToneLike` that places in given index.
    /// - If `index > size`, `index` will be `index % size`.
    /// - If `index < 0`, `index` will be `size + index % size`.
    fn get_pitch(&self, index: usize) -> Pitch<&Self::PitchClass> {
        let oct = index / self.size();
        let index = index % self.size();
        let class = self.get_class(index).unwrap();
        Pitch::new(class, oct)
    }

    fn iter(&self) -> ClassIter<'_, Self> {
        ClassIter::new(self)
    }

    /// Returns an `Interval` of `from` and `to`.
    fn get_interval(
        &self,
        from: &Pitch<Self::PitchClass>,
        to: &Pitch<Self::PitchClass>,
    ) -> Interval {
        let from_step = from.oct() * self.size() + from.class().step();
        let to_step = to.oct() * self.size() + to.class().step();
        Interval::new(from_step, to_step)
    }

    fn get_step(&self, pitch: &Pitch<Self::PitchClass>) -> usize {
        pitch.oct() * self.size() + pitch.class().step()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ClassIter<'a, C> {
    chroma: &'a C,
    index: usize,
    offset: usize,
}

impl<'a, C> ClassIter<'a, C> {
    pub fn new(chroma: &'a C) -> Self {
        Self {
            chroma,
            index: 0,
            offset: 0,
        }
    }

    pub fn offset(&mut self, offset: usize) {
        self.offset = offset
    }
}

impl<'a, C: Keyboard> Iterator for ClassIter<'a, C> {
    type Item = &'a C::PitchClass;

    fn next(&mut self) -> Option<Self::Item> {
        let item = if self.index < self.chroma.size() {
            let step = self.offset + self.index;
            Some(self.chroma.get_class(step).unwrap())
        } else {
            None
        };
        self.index += 1;
        item
    }
}
