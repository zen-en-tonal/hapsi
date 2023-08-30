use std::ops::{AddAssign, SubAssign};

/// Represents a note in usize.
/// e.g. MIDI number
pub trait Number: AddAssign + SubAssign + Sized + Clone + Copy {
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

pub struct Cycle<T> {
    inner: T,
    len: usize,
}

impl<T> Cycle<T> {
    pub fn new(inner: T, len: usize) -> Cycle<T> {
        Cycle { inner, len }
    }
}

impl<T: Number> Cycle<T> {
    pub fn value(&self) -> usize {
        self.inner.value() % self.len
    }

    pub fn increment(&mut self, value: T) {
        self.inner += value;
    }

    pub fn has_cycled(&self) -> bool {
        self.inner.value() >= self.len
    }
}
