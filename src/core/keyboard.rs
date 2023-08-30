use std::collections::HashMap;

use super::{distance::Degree, pitch::Pitch};

pub use super::number::*;

pub trait Octave: Sized {
    type PitchClass: PartialEq;

    fn get_class(&self, number: &Cycle<impl Number>) -> &Self::PitchClass;

    fn get_number(&self, class: &Self::PitchClass) -> Option<usize>;

    fn len(&self) -> usize;

    fn iter(&self) -> ClassIter<'_, Self> {
        ClassIter {
            inner: self,
            cycle: Cycle::new(0, self.len()),
        }
    }
}

pub struct ClassIter<'a, Oct> {
    inner: &'a Oct,
    cycle: Cycle<usize>,
}

impl<'a, Oct: Octave> Iterator for ClassIter<'a, Oct> {
    type Item = &'a Oct::PitchClass;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycle.has_cycled() {
            return None;
        }
        let class = self.inner.get_class(&self.cycle);
        self.cycle.increment(1);
        return Some(class);
    }
}

impl<V: PartialEq> Octave for HashMap<usize, V> {
    type PitchClass = V;

    fn get_class(&self, number: &Cycle<impl Number>) -> &Self::PitchClass {
        self.get(&number.value()).unwrap()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get_number(&self, class: &Self::PitchClass) -> Option<usize> {
        self.iter().find(|x| x.1 == class).map(|x| *x.0)
    }
}

impl<V: PartialEq> Octave for Vec<V> {
    type PitchClass = V;

    fn get_class(&self, number: &Cycle<impl Number>) -> &Self::PitchClass {
        self.get(number.value()).unwrap()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get_number(&self, class: &Self::PitchClass) -> Option<usize> {
        self.iter().position(|x| x == class)
    }
}

pub struct Keyboard<T>(T);

impl<T> Keyboard<T> {
    pub fn new(oct: T) -> Self {
        Self(oct)
    }
}

impl<Oct: Octave> Keyboard<Oct> {
    pub fn get_class(&self, number: &impl Number) -> &Oct::PitchClass {
        let cycle = Cycle::new(number.to_owned(), self.0.len());
        self.0.get_class(&cycle)
    }

    pub fn get_pitch(&self, number: &impl Number) -> Pitch<&Oct::PitchClass> {
        let class = self.get_class(number);
        let oct = number.value() / self.0.len();
        Pitch::new(class, oct)
    }

    pub fn as_number(&self, pitch: &Pitch<Oct::PitchClass>) -> Option<usize> {
        self.0
            .get_number(pitch.class())
            .map(|x| x + self.0.len() * pitch.oct())
    }

    pub fn class_iter(&self) -> ClassIter<'_, Oct> {
        self.0.iter()
    }
}

pub trait Scale {
    fn convert(&self, number: impl Number) -> usize;

    fn len(&self) -> usize;
}

impl Scale for Vec<usize> {
    fn convert(&self, number: impl Number) -> usize {
        self.get(number.value()).unwrap().clone()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

pub struct Scaled<S, O> {
    oct: O,
    scaler: S,
}

impl<S, O> Scaled<S, O> {
    pub fn new(scale: S, oct: O) -> Self {
        Self { oct, scaler: scale }
    }
}

impl<S: Scale, O: Octave> Octave for Scaled<S, O> {
    type PitchClass = O::PitchClass;

    fn get_class(&self, number: &Cycle<impl Number>) -> &Self::PitchClass {
        let cycle = Cycle::new(self.scaler.convert(number.value()), self.oct.len());
        self.oct.get_class(&cycle)
    }

    fn get_number(&self, class: &Self::PitchClass) -> Option<usize> {
        if self.oct.get_number(class).is_none() {
            return None;
        }
        let mut c = Cycle::new(0_usize, self.len());
        loop {
            let current_class = self.get_class(&c);
            if current_class == class {
                return Some(c.value());
            }
            c.increment(1);
            if c.has_cycled() {
                return None;
            }
        }
    }

    fn len(&self) -> usize {
        self.scaler.len()
    }
}

pub trait Distance<T> {
    type Distance;

    fn measure(&self, from: T, to: T) -> Option<Self::Distance>;
}

impl<S: Scale, Oct: Octave> Distance<&Pitch<Oct::PitchClass>> for Keyboard<Scaled<S, Oct>> {
    type Distance = Degree;

    fn measure(
        &self,
        from: &Pitch<Oct::PitchClass>,
        to: &Pitch<Oct::PitchClass>,
    ) -> Option<Self::Distance> {
        match (self.as_number(from), self.as_number(to)) {
            (Some(from), Some(to)) => Some(Degree::new(to - from + 1).unwrap()),
            (_, _) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_class() {
        let oct = vec![0, 1, 2, 3, 4, 5];
        let key = Keyboard::new(oct);
        assert_eq!(key.get_class(&0_usize), &0);
        assert_eq!(key.get_class(&6_usize), &0);
    }

    #[test]
    fn get_pitch() {
        let oct = vec![0, 1, 2, 3, 4, 5];
        let key = Keyboard::new(oct);
        assert_eq!(key.get_pitch(&0_usize), Pitch::new(&0_usize, 0));
        assert_eq!(key.get_pitch(&6_usize), Pitch::new(&0_usize, 1));
    }

    #[test]
    fn scaler() {
        let oct = vec![0, 1, 2, 3, 4, 5];
        let scaler = vec![0, 2, 4];
        let key = Keyboard::new(Scaled::new(scaler, oct));
        assert_eq!(key.get_class(&0_usize), &0);
        assert_eq!(key.get_class(&1_usize), &2);
        assert_eq!(key.get_class(&2_usize), &4);
        assert_eq!(key.get_class(&3_usize), &0);
    }

    #[test]
    fn scaler_get_number() {
        let oct = vec![0, 1, 2, 3, 4, 5];
        let scaler = vec![0, 2, 4];
        let scaled = Scaled::new(scaler, oct);
        assert_eq!(scaled.get_number(&4), Some(2));
        assert_eq!(scaled.get_number(&5), None);
    }

    #[test]
    fn scaler_iter() {
        let oct = vec![0, 1, 2, 3, 4, 5];
        let scaler = vec![0, 2, 4];
        let binding = Scaled::new(scaler, oct);
        let mut scaled = binding.iter();
        assert_eq!(scaled.next(), Some(&0));
        assert_eq!(scaled.next(), Some(&2));
        assert_eq!(scaled.next(), Some(&4));
        assert_eq!(scaled.next(), None);
    }

    #[test]
    fn measure_degree() {
        let oct = vec![0, 1, 2, 3, 4, 5];
        let scaler = vec![0, 2, 4];
        let key = Keyboard::new(Scaled::new(scaler, oct));
        assert_eq!(
            key.measure(&Pitch::new(0, 0), &Pitch::new(0, 0)),
            Some(Degree::new(1).unwrap())
        );
        assert_eq!(
            key.measure(&Pitch::new(0, 0), &Pitch::new(2, 0)),
            Some(Degree::new(2).unwrap())
        );
        assert_eq!(
            key.measure(&Pitch::new(0, 0), &Pitch::new(0, 1)),
            Some(Degree::new(4).unwrap())
        );
        assert_eq!(key.measure(&Pitch::new(0, 0), &Pitch::new(1, 1)), None);
    }
}
