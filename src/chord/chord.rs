use crate::core::*;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Chord {
    root: Degree,
    quantity: Quantity,
    quality: Quality,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Quantity {
    Triad,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Quality {
    Major,
    Minor,
}

impl Chord {
    pub fn from_degree(degree: &Degree, quality: &Quality, quantitiy: &Quantity) -> Self {
        Self {
            root: degree.clone(),
            quantity: quantitiy.clone(),
            quality: quality.clone(),
        }
    }

    pub fn degrees(&self) -> Vec<Degree> {
        vec![
            self.root.clone(),
            Degree::new(self.root.value() + 2),
            Degree::new(self.root.value() + 4),
        ]
    }
}

pub trait ChordScale {
    type ToneLike: ToneLike;
    fn avoids(&self, chord: &Chord) -> Vec<Self::ToneLike>;
    fn chord_tones(&self, chord: &Chord) -> Vec<Self::ToneLike>;
}

impl<T: Scale> ChordScale for T {
    type ToneLike = T::ToneLike;

    fn avoids(&self, chord: &Chord) -> Vec<Self::ToneLike> {
        let mut vec = self
            .chord_tones(chord)
            .into_iter()
            .flat_map(|t| {
                vec![
                    self.chroma().tone(t.step() as i32 - 1),
                    self.chroma().tone(t.step() as i32 + 1),
                ]
            })
            .filter(|t| self.is_on_scale(t))
            .collect::<Vec<Self::ToneLike>>();
        vec.dedup();
        vec
    }

    fn chord_tones(&self, chord: &Chord) -> Vec<Self::ToneLike> {
        chord
            .degrees()
            .iter()
            .map(|d| self.get_by_degree(d))
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        chord::chord::{Chord, ChordScale, Quality, Quantity},
        prelude::*,
    };

    #[test]
    fn chord_tones() {
        let scale = Diatonic::major(&Tone::new(C, Natural));
        let chord = Chord::from_degree(&Degree::new(1), &Quality::Major, &Quantity::Triad);
        let mut tones = scale.chord_tones(&chord).into_iter();
        assert_eq!(tones.next(), Some(Tone::new(C, Natural)));
        assert_eq!(tones.next(), Some(Tone::new(E, Natural)));
        assert_eq!(tones.next(), Some(Tone::new(G, Natural)));
        assert_eq!(tones.next(), None);
    }

    #[test]
    fn avoids() {
        let scale = Diatonic::major(&Tone::new(ToneSymbol::C, AccidentalSymbol::Natural));
        let chord = Chord::from_degree(&Degree::new(1), &Quality::Major, &Quantity::Triad);
        let mut tones = scale.avoids(&chord).into_iter();
        assert_eq!(tones.next(), Some(Tone::new(B, Natural)));
        assert_eq!(tones.next(), Some(Tone::new(F, Natural)));
        assert_eq!(tones.next(), None);
    }
}
