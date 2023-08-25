use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Pentatonic {
    key: Tone,
    quality: Quality,
}

impl Pentatonic {
    pub fn major(key: &Tone) -> Self {
        Self {
            key: key.clone().into(),
            quality: Quality::Major,
        }
    }

    pub fn minor(key: &Tone) -> Self {
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

impl Scale for Pentatonic {
    fn key(&self) -> Tone {
        self.key.into()
    }

    fn distances(&self) -> Vec<i32> {
        match self.quality() {
            Quality::Major => [0, 2, 4, 7, 9].to_vec(),
            Quality::Minor => [0, 3, 5, 7, 10].to_vec(),
        }
    }

    fn chroma(&self) -> Chroma {
        Chroma
    }

    type ToneLike = Tone;

    type ChromaLike = Chroma;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn major() {
        let scale = super::Pentatonic::major(&Tone::new(C, Natural));
        let mut tones = scale.tones().into_iter();
        assert_eq!(tones.next(), Some(Tone::new(C, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(D, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(E, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(G, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(A, Natural).into()));
        assert_eq!(tones.next(), None);
    }

    #[test]
    fn minor() {
        let scale = super::Pentatonic::minor(&Tone::new(A, Natural));
        let mut tones = scale.tones().into_iter();
        assert_eq!(tones.next(), Some(Tone::new(A, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(C, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(D, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(E, Natural).into()));
        assert_eq!(tones.next(), Some(Tone::new(G, Natural).into()));
        assert_eq!(tones.next(), None);
    }
}
