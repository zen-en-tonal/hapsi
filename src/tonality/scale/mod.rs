use super::Tone;
use std::sync::Arc;

mod major_scale;
mod minor_scale;

pub use major_scale::*;
pub use minor_scale::*;

pub trait Scale {
    fn tonic(&self) -> &Tone;

    fn is_on_scale(&self, tone: &Tone) -> bool;
}

pub trait IntoScaler<S> {
    fn into_scaler(self) -> Scaler<S>;
}

impl<T: Scale + Sized> IntoScaler<T> for T {
    fn into_scaler(self) -> Scaler<T> {
        let vec: Vec<Tone> = self
            .tonic()
            .clone()
            .into_iter()
            .filter(|t| self.is_on_scale(t))
            .collect();
        let iter = ScaleToneIter {
            inner: vec.as_slice().into(),
            index: 0,
        };
        Scaler::<T> {
            scale: Arc::new(self),
            iter,
        }
    }
}

#[derive(Debug, Clone)]
struct ScaleToneIter {
    inner: Arc<[Tone]>,
    index: usize,
}

impl Iterator for ScaleToneIter {
    type Item = Tone;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.len() > self.index {
            true => {
                let current = self.inner[self.index].clone();
                self.index += 1;
                Some(current)
            }
            false => None,
        }
    }
}

#[derive(Debug)]
pub struct Scaler<S> {
    iter: ScaleToneIter,
    scale: Arc<S>,
}

impl<S: Scale> Clone for Scaler<S> {
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            scale: self.scale.clone(),
        }
    }
}

impl<S> Iterator for Scaler<S> {
    type Item = Tone;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<S: Scale> Scale for Scaler<S> {
    fn tonic(&self) -> &Tone {
        self.scale.tonic()
    }

    fn is_on_scale(&self, tone: &Tone) -> bool {
        self.scale.is_on_scale(tone)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Degree(usize);

impl Degree {
    pub fn new(degree: usize) -> Self {
        Self(degree)
    }
}

impl<S: Scale> Scaler<S> {
    pub fn get_interval(&self, tone: Tone) -> Option<Degree> {
        if !self.is_on_scale(&tone) {
            return None;
        }
        let mut iter = self.clone();
        let index = iter.position(|t| t == tone).unwrap();
        Some(Degree(index + 1))
    }
}

#[cfg(test)]
mod tests {
    use crate::tonality::{AccidentalSymbol, Degree, IntoScaler, Scale, Tone, ToneSymbol};
    use AccidentalSymbol::*;
    use ToneSymbol::*;

    struct RandomScale(Tone);
    impl Scale for RandomScale {
        fn tonic(&self) -> &Tone {
            &self.0
        }

        fn is_on_scale(&self, _: &Tone) -> bool {
            true
        }
    }

    #[test]
    fn get_interval() {
        let c = RandomScale(Tone::new(C, Natural)).into_scaler();
        assert_eq!(c.get_interval(Tone::new(C, Natural)), Some(Degree::new(1)));
        assert_eq!(c.get_interval(Tone::new(B, Natural)), Some(Degree::new(12)));
    }
}
