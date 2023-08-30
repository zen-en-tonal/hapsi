use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    core::Chord as CoreChord,
    core::*,
    prelude::{Tone, Twelve},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chord<T> {
    root: T,
    quality: Quality,
}

impl<T> Chord<T> {
    pub fn new(root: T, quality: Quality) -> Self {
        Self { root, quality }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quality {
    Major,
    Minor,
    Dim,
    Aug,
    Major7,
    Minor7,
}

static ENUM_QUALITY: Lazy<[Quality; 6]> = Lazy::new(|| {
    [
        Quality::Major,
        Quality::Minor,
        Quality::Dim,
        Quality::Aug,
        Quality::Major7,
        Quality::Minor7,
    ]
});

impl Quality {
    pub fn enumerate() -> std::slice::Iter<'static, Quality> {
        ENUM_QUALITY.iter()
    }
}

static QUALITY_TO_INTERVAL: Lazy<HashMap<Quality, Vec<Interval>>> = Lazy::new(|| {
    let mut hash = HashMap::<Quality, Vec<Interval>>::default();
    hash.insert(Quality::Major, vec![4.into(), 7.into()]);
    hash.insert(Quality::Minor, vec![3.into(), 7.into()]);
    hash.insert(Quality::Dim, vec![3.into(), 6.into()]);
    hash.insert(Quality::Aug, vec![4.into(), 8.into()]);
    hash.insert(Quality::Major7, vec![4.into(), 7.into(), 11.into()]);
    hash.insert(Quality::Minor7, vec![3.into(), 7.into(), 11.into()]);
    hash
});

impl Chord<Tone> {
    pub fn into_class(self) -> CoreChord<Tone> {
        let keyboard = Keyboard::new(Twelve);
        let intervals = QUALITY_TO_INTERVAL.get(&self.quality).unwrap();
        let root_value: usize = self.root.into();
        let others = intervals
            .iter()
            .map(|i| keyboard.get_class(&(root_value + i.value())).clone())
            .collect();
        CoreChord::new(self.root, others)
    }
}

impl Chord<Pitch<Tone>> {
    pub fn into_pitch(self) -> CoreChord<Pitch<Tone>> {
        let keyboard = Keyboard::new(Twelve);
        let intervals = QUALITY_TO_INTERVAL.get(&self.quality).unwrap();
        let root_value: usize = keyboard.as_number(&self.root).unwrap();
        let others = intervals
            .iter()
            .map(|i| keyboard.get_pitch(&(root_value + i.value())).deref())
            .collect();
        CoreChord::new(self.root, others)
    }
}

#[cfg(test)]
mod tests {

    use super::Chord;

    #[test]
    fn into_class() {
        let mut chord = Chord::new("A".parse().unwrap(), super::Quality::Major7)
            .into_class()
            .into_vec()
            .into_iter();
        assert_eq!(chord.next(), Some("A".parse().unwrap()));
        assert_eq!(chord.next(), Some("Cs".parse().unwrap()));
        assert_eq!(chord.next(), Some("E".parse().unwrap()));
        assert_eq!(chord.next(), Some("Gs".parse().unwrap()));
    }
}
