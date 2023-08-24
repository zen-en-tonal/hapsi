use crate::core::scale::Scale;
use crate::core::tone::{Chroma, Tone};
use crate::twelve_tet;

#[derive(Debug, Clone, Copy)]
pub struct Diatonic {
    key: Tone,
    quality: Quality,
}

impl Diatonic {
    pub fn major(key: &twelve_tet::tone::Tone) -> Self {
        Self {
            key: key.clone().into(),
            quality: Quality::Major,
        }
    }

    pub fn minor(key: &twelve_tet::tone::Tone) -> Self {
        Self {
            key: key.clone().into(),
            quality: Quality::Minor,
        }
    }

    pub fn quality(&self) -> Quality {
        self.quality
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Quality {
    Major,
    Minor,
}

impl Scale for Diatonic {
    fn key(&self) -> Tone {
        self.key.into()
    }

    fn distances(&self) -> Vec<i32> {
        match self.quality() {
            Quality::Major => [0, 2, 4, 5, 7, 9, 11].to_vec(),
            Quality::Minor => [0, 2, 3, 5, 7, 8, 10].to_vec(),
        }
    }

    fn chroma(&self) -> Chroma {
        Chroma::new(12)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::scale::Scale;
    use crate::twelve_tet::tone::AccidentalSymbol::*;
    use crate::twelve_tet::tone::Tone;
    use crate::twelve_tet::tone::ToneSymbol::*;

    #[test]
    fn major() {
        let scale = super::Diatonic::major(&Tone::new(C, Natural));
        let mut tones = scale.tones().into_iter();
        assert_eq!(tones.next(), Some(Tone::new(C, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(D, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(E, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(F, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(G, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(A, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(B, Natural).into()));
        assert_eq!(tones.next(), None);
    }

    #[test]
    fn minor() {
        let scale = super::Diatonic::minor(&Tone::new(A, Natural));
        let mut tones = scale.tones().into_iter();
        assert_eq!(tones.next(), Some(Tone::new(A, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(B, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(C, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(D, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(E, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(F, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(G, Natural).into()));
        assert_eq!(tones.next(), None);
    }
}