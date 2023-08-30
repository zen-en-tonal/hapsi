/// Represents a note in usize.
/// e.g. MIDI number
pub trait Number: Sized + Clone + Copy {
    fn value(&self) -> usize;
}

impl Number for u8 {
    fn value(&self) -> usize {
        *self as usize
    }
}

impl Number for usize {
    fn value(&self) -> usize {
        *self
    }
}

pub struct Cycle {
    inner: usize,
    len: usize,
}

impl Cycle {
    pub fn new<T: Number>(inner: T, len: usize) -> Cycle {
        Cycle {
            inner: inner.value(),
            len,
        }
    }
}

impl Cycle {
    pub fn value(&self) -> usize {
        self.inner.value() % self.len
    }

    pub fn increment(&mut self, value: usize) {
        self.inner += value;
    }

    pub fn has_cycled(&self) -> bool {
        self.inner.value() >= self.len
    }
}
