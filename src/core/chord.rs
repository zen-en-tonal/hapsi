use std::fmt::Debug;

use super::{Degree, Distance, Keyboard, Octave, Pitch, Scale, Scaled};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord<T> {
    root: T,
    other: Vec<T>,
}

impl<T> Chord<T> {
    pub fn new(root: T, other: Vec<T>) -> Self {
        Self { root, other }
    }

    pub fn into_vec(self) -> Vec<T> {
        let mut vec = self.other;
        vec.insert(0, self.root);
        vec
    }
}

impl Chord<Degree> {
    pub fn to_pitch<'a, O: Octave>(&'a self, key: &'a Keyboard<O>) -> Chord<Pitch<&O::PitchClass>> {
        let root = key.get_pitch(&self.root);
        let other = self
            .other
            .iter()
            .map(|&x| key.get_pitch(&(self.root.value() + x.value() - 2)))
            .collect();
        Chord { root, other }
    }
}

impl<T: Clone> Chord<Pitch<T>> {
    pub fn to_degree<'a, O: Octave<PitchClass = T>>(
        &'a self,
        keyboard: &'a Keyboard<Scaled<impl Scale, O>>,
    ) -> Option<Chord<Degree>> {
        let key = keyboard.get_pitch(&0_usize).deref();
        let Some(root_deg) = keyboard.measure(&key, &self.root) else {
            return None;
        };
        let others = self.other.iter().map(|p| keyboard.measure(&self.root, p));
        if others.clone().any(|o| o.is_none()) {
            return None;
        }
        Some(Chord {
            root: root_deg,
            other: others.map(|o| o.unwrap()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::{Degree, Keyboard, Pitch, Scaled, Tone, Twelve},
        scale::Diatonic,
    };

    use super::Chord;

    #[test]
    fn to_pitch() {
        let scale = Diatonic::minor(&"A".parse().unwrap());
        let scaled = Scaled::new(scale, Twelve);
        let keyboard = Keyboard::new(scaled);
        let chord = Chord::<Degree> {
            root: Degree::new(1).unwrap(),
            other: vec![Degree::new(3).unwrap(), Degree::new(5).unwrap()],
        };
        let c = chord.to_pitch(&keyboard);
        assert_eq!(c.root, Pitch::new(&"A".parse().unwrap(), 0));
        assert_eq!(c.other.get(0), Some(&Pitch::new(&"C".parse().unwrap(), 0)));
        assert_eq!(c.other.get(1), Some(&Pitch::new(&"E".parse().unwrap(), 0)));
    }

    #[test]
    fn to_degree() {
        let scale = Diatonic::minor(&"A".parse().unwrap());
        let scaled = Scaled::new(scale, Twelve);
        let keyboard = Keyboard::new(scaled);
        let chord = Chord::<Pitch<Tone>> {
            root: Pitch::new("A".parse().unwrap(), 0),
            other: vec![
                Pitch::new("C".parse().unwrap(), 1),
                Pitch::new("E".parse().unwrap(), 1),
            ],
        };
        let c = chord.to_degree(&keyboard).unwrap();
        assert_eq!(c.root, Degree::new(1).unwrap());
        assert_eq!(c.other.get(0), Some(&Degree::new(3).unwrap()));
        assert_eq!(c.other.get(1), Some(&Degree::new(5).unwrap()));
    }
}
