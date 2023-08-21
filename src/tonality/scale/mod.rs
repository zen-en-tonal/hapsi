mod major_pentatonic_scale_walker;
mod major_scale_walker;
mod minor_pentatonic_scale_walker;
mod minor_scale_walker;

use self::{
    major_pentatonic_scale_walker::MajorPentatonicScaleWalker,
    major_scale_walker::MajorScaleWalker,
    minor_pentatonic_scale_walker::MinorPentatonicScaleWalker,
    minor_scale_walker::MinorScaleWalker,
};

use super::Tone;
use crate::tonality::Pitch;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Scale {
    Major(Tone),
    Minor(Tone),
    MajorPentatonic(Tone),
    MinorPentatonic(Tone),
}

impl Scale {
    fn get_walker(&self) -> Walker {
        match self {
            Scale::Major(tonic) => Walker {
                walker: Box::new(MajorScaleWalker::default()),
                current: tonic.clone(),
            },
            Scale::Minor(tonic) => Walker {
                walker: Box::new(MinorScaleWalker::default()),
                current: tonic.clone(),
            },
            Scale::MajorPentatonic(tonic) => Walker {
                walker: Box::new(MajorPentatonicScaleWalker::default()),
                current: tonic.clone(),
            },
            Scale::MinorPentatonic(tonic) => Walker {
                walker: Box::new(MinorPentatonicScaleWalker::default()),
                current: tonic.clone(),
            },
        }
    }

    pub fn into_relative(self) -> Option<Scale> {
        match self {
            Scale::Major(tonic) => Some(Self::Minor(tonic.transpose(-3))),
            Scale::Minor(tonic) => Some(Self::Major(tonic.transpose(3))),
            Scale::MajorPentatonic(tonic) => Some(Self::MinorPentatonic(tonic.transpose(-3))),
            Scale::MinorPentatonic(tonic) => Some(Self::MajorPentatonic(tonic.transpose(3))),
        }
    }

    pub fn transpose(self, transpose: i32) -> Self {
        match self {
            Scale::Major(tonic) => Self::Major(tonic.transpose(transpose)),
            Scale::Minor(tonic) => Self::Minor(tonic.transpose(transpose)),
            Scale::MajorPentatonic(tonic) => Scale::MajorPentatonic(tonic.transpose(transpose)),
            Scale::MinorPentatonic(tonic) => Scale::MinorPentatonic(tonic.transpose(transpose)),
        }
    }

    pub fn composed_notes(&self) -> Vec<Tone> {
        let waler = self.get_walker();
        waler.into_iter().collect()
    }

    pub fn tonic(&self) -> &Tone {
        match self {
            Scale::Major(tonic) => tonic,
            Scale::Minor(tonic) => tonic,
            Scale::MajorPentatonic(tonic) => tonic,
            Scale::MinorPentatonic(tonic) => tonic,
        }
    }

    pub fn degree(&self, from: &Tone, to: &Tone) -> Result<Degree, ScaleError> {
        let from_index = self.composed_notes().iter().position(|t| t == from);
        let to_index = self.composed_notes().iter().position(|t| t == to);
        match (from_index, to_index) {
            (Some(from), Some(to)) => Ok(Degree(((to as i32 - from as i32).abs() + 1) as usize)),
            _ => Err(ScaleError::NotFound),
        }
    }

    pub fn degree_from_tonic(&self, other: &Tone) -> Result<Degree, ScaleError> {
        self.degree(self.tonic(), other)
    }

    pub fn dominant(&self) -> Tone {
        let dominant = Degree::new(5);
        self.composed_notes()
            .iter()
            .find(|t| self.degree_from_tonic(t).unwrap() == dominant)
            .unwrap()
            .clone()
    }
}

struct Walker {
    walker: Box<dyn Iterator<Item = i32>>,
    current: Tone,
}

impl Iterator for Walker {
    type Item = Tone;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(interval) = self.walker.next() else { return None };
        let c = self.current.clone();
        let mut transposed = self.current.clone().transpose(interval);
        if c.tone() == transposed.tone() {
            transposed = transposed.flip_accidential();
        }
        self.current = transposed;
        Some(c)
    }
}

#[cfg(test)]
mod test {
    use crate::tonality::{AccidentalSymbol, Degree, Scale, ScaleError, Tone, ToneSymbol};

    #[test]
    fn major_scale() {
        use AccidentalSymbol::*;
        use ToneSymbol::*;

        let a_major = Scale::Major(Tone::new(A, Natural));
        assert_eq!(
            a_major.composed_notes(),
            vec![
                Tone::new(A, Natural),
                Tone::new(B, Natural),
                Tone::new(C, Sharp),
                Tone::new(D, Natural),
                Tone::new(E, Natural),
                Tone::new(F, Sharp),
                Tone::new(G, Sharp)
            ]
        );

        let e_flat_major = Scale::Major(Tone::new(E, Flat));
        assert_eq!(
            e_flat_major.composed_notes(),
            vec![
                Tone::new(E, Flat),
                Tone::new(F, Natural),
                Tone::new(G, Natural),
                Tone::new(A, Flat),
                Tone::new(B, Flat),
                Tone::new(C, Natural),
                Tone::new(D, Natural),
            ]
        );
    }

    #[test]
    fn minor_scale() {
        use AccidentalSymbol::*;
        use ToneSymbol::*;
        let scale = Scale::Minor(Tone::new(A, Natural));
        assert_eq!(
            scale.composed_notes(),
            vec![
                Tone::new(A, Natural),
                Tone::new(B, Natural),
                Tone::new(C, Natural),
                Tone::new(D, Natural),
                Tone::new(E, Natural),
                Tone::new(F, Natural),
                Tone::new(G, Natural)
            ]
        )
    }

    #[test]
    fn degree() {
        use AccidentalSymbol::*;
        use ToneSymbol::*;
        assert_eq!(
            Scale::Major(Tone::new(A, Natural))
                .degree(&Tone::new(A, Natural), &Tone::new(C, Sharp))
                .unwrap(),
            Degree::new(3)
        );
        assert_eq!(
            Scale::Major(Tone::new(C, Natural))
                .degree(&Tone::new(C, Natural), &Tone::new(C, Natural))
                .unwrap(),
            Degree::new(1)
        );
        assert_eq!(
            Scale::Major(Tone::new(C, Natural))
                .degree(&Tone::new(E, Natural), &Tone::new(C, Natural))
                .unwrap(),
            Degree::new(3)
        );
        assert_eq!(
            Scale::Major(Tone::new(C, Natural))
                .degree(&Tone::new(C, Sharp), &Tone::new(C, Natural))
                .unwrap_err(),
            ScaleError::NotFound
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Degree(usize);

impl Degree {
    pub fn new(degree: usize) -> Self {
        Self(degree)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ScaleError {
    NotFound,
}

impl Display for ScaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ScaleError")
    }
}

impl Error for ScaleError {}
