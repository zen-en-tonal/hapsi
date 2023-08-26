use crate::prelude::*;

#[derive(Debug)]
pub struct Pentatonic {
    key: Tone,
    quality: Quality,
    chroma: Chroma,
    distances: [Interval; 5],
}

impl Pentatonic {
    pub fn major(key: &Tone) -> Self {
        Self {
            key: *key,
            quality: Quality::Major,
            chroma: Chroma::new(),
            distances: [0, 2, 4, 7, 9].map(Into::<Interval>::into),
        }
    }

    pub fn minor(key: &Tone) -> Self {
        Self {
            key: *key,
            quality: Quality::Minor,
            chroma: Chroma::new(),
            distances: [0, 3, 5, 7, 10].map(Into::<Interval>::into),
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

impl ScaleLike for Pentatonic {
    type ToneLike = Tone;

    type ChromaLike = Chroma;

    fn key(&self) -> &Self::ToneLike {
        &self.key
    }

    fn intervals(&self) -> &[Interval] {
        &self.distances
    }

    fn chroma(&self) -> &Self::ChromaLike {
        &self.chroma
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn major() {
        let scale = super::Pentatonic::major(&Tone::new(C, Natural));
        let mut tones = scale.tones();
        assert_eq!(tones.next(), Some(&Tone::new(C, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(D, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(E, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(G, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(A, Natural)));
        assert_eq!(tones.next(), None);
    }

    #[test]
    fn minor() {
        let scale = super::Pentatonic::minor(&Tone::new(A, Natural));
        let mut tones = scale.tones();
        assert_eq!(tones.next(), Some(&Tone::new(A, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(C, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(D, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(E, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(G, Natural)));
        assert_eq!(tones.next(), None);
    }
}
