use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Diatonic {
    key: Tone,
    quality: Quality,
}

impl Diatonic {
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

    type ToneLike = Tone;

    type ChromaLike = Chroma;

    fn chroma(&self) -> Self::ChromaLike {
        Chroma
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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

    #[test]
    fn degree() {
        let scale = super::Diatonic::minor(&Tone::new(A, Natural));
        assert_eq!(scale.distance(&Tone::new(A, Natural)), Some(Degree::new(1)));
        assert_eq!(
            scale.distance(&Tone::new(B, Natural).into()),
            Some(Degree::new(2))
        );
        assert_eq!(
            scale.distance(&Tone::new(C, Natural).into()),
            Some(Degree::new(3))
        );
        assert_eq!(scale.distance(&Tone::new(C, Sharp).into()), None);
    }

    #[test]
    fn get_by_degree() {
        let scale = super::Diatonic::minor(&Tone::new(A, Natural));
        assert_eq!(scale.get_by_degree(&Degree::new(1)), Tone::new(A, Natural));
        assert_eq!(scale.get_by_degree(&Degree::new(13)), Tone::new(F, Natural));
    }
}
