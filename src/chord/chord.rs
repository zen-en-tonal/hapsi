use crate::core::{
    scale::{Degree, Scale},
    tone::Tone,
    transpose::Transpose,
};

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
    fn avoids(&self, chord: &Chord) -> Vec<Tone>;
    fn chord_tones(&self, chord: &Chord) -> Vec<Tone>;
}

impl<T: Scale> ChordScale for T {
    fn avoids(&self, chord: &Chord) -> Vec<Tone> {
        let mut vec = self
            .chord_tones(chord)
            .into_iter()
            .flat_map(|t| vec![t.transpose(-1), t.transpose(1)])
            .filter(|t| self.is_on_scale(t))
            .collect::<Vec<Tone>>();
        vec.dedup();
        vec
    }

    fn chord_tones(&self, chord: &Chord) -> Vec<Tone> {
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
        core::scale::Degree,
        scale::diatonic::Diatonic,
        twelve_tet::tone::{tone, AccidentalSymbol, Tone, ToneSymbol},
    };

    use super::{Chord, ChordScale, Quality, Quantity};

    #[test]
    fn chord_tones() {
        let scale = Diatonic::major(&Tone::new(ToneSymbol::C, AccidentalSymbol::Natural));
        let chord = Chord::from_degree(&Degree::new(1), &Quality::Major, &Quantity::Triad);
        let mut tones = scale.chord_tones(&chord).into_iter();
        assert_eq!(
            tones.next(),
            Some(tone(ToneSymbol::C, AccidentalSymbol::Natural))
        );
        assert_eq!(
            tones.next(),
            Some(tone(ToneSymbol::E, AccidentalSymbol::Natural))
        );
        assert_eq!(
            tones.next(),
            Some(tone(ToneSymbol::G, AccidentalSymbol::Natural))
        );
        assert_eq!(tones.next(), None);
    }

    #[test]
    fn avoids() {
        let scale = Diatonic::major(&Tone::new(ToneSymbol::C, AccidentalSymbol::Natural));
        let chord = Chord::from_degree(&Degree::new(1), &Quality::Major, &Quantity::Triad);
        let mut tones = scale.avoids(&chord).into_iter();
        assert_eq!(
            tones.next(),
            Some(tone(ToneSymbol::B, AccidentalSymbol::Natural))
        );
        assert_eq!(
            tones.next(),
            Some(tone(ToneSymbol::F, AccidentalSymbol::Natural))
        );
        assert_eq!(tones.next(), None);
    }
}
