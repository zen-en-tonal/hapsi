use crate::prelude::*;

#[derive(Debug)]
pub struct Diatonic {
    key: Tone,
    quality: Quality,
    chroma: Chroma,
    distances: [Interval; 7],
}

impl Diatonic {
    pub fn major(key: &Tone) -> Self {
        Self {
            key: *key,
            quality: Quality::Major,
            chroma: Chroma::new(),
            distances: [0, 2, 4, 5, 7, 9, 11].map(Into::<Interval>::into),
        }
    }

    pub fn minor(key: &Tone) -> Self {
        Self {
            key: *key,
            quality: Quality::Minor,
            chroma: Chroma::new(),
            distances: [0, 2, 3, 5, 7, 8, 10].map(Into::<Interval>::into),
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

impl ScaleLike for Diatonic {
    type ToneLike = Tone;
    type ChromaLike = Chroma;

    fn key(&self) -> &Tone {
        &self.key
    }

    fn chroma(&self) -> &Self::ChromaLike {
        &self.chroma
    }

    fn intervals(&self) -> &[Interval] {
        &self.distances
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn major() {
        let scale = super::Diatonic::major(&Tone::new(C, Natural));
        let mut tones = scale.tones();
        assert_eq!(tones.next(), Some(&Tone::new(C, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(D, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(E, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(F, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(G, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(A, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(B, Natural)));
        assert_eq!(tones.next(), None);
    }

    #[test]
    fn minor() {
        let scale = super::Diatonic::minor(&Tone::new(A, Natural));
        let mut tones = scale.tones();
        assert_eq!(tones.next(), Some(&Tone::new(A, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(B, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(C, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(D, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(E, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(F, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(G, Natural)));
        assert_eq!(tones.next(), None);
    }

    #[test]
    fn degree() {
        let scale = super::Diatonic::minor(&Tone::new(A, Natural));
        assert_eq!(
            scale.get_distance_from_key(&Tone::new(A, Natural)),
            Some(Degree::new(1).unwrap())
        );
        assert_eq!(
            scale.get_distance_from_key(&Tone::new(B, Natural)),
            Some(Degree::new(2).unwrap())
        );
        assert_eq!(
            scale.get_distance_from_key(&Tone::new(C, Natural)),
            Some(Degree::new(3).unwrap())
        );
        assert_eq!(scale.get_distance_from_key(&Tone::new(C, Sharp)), None);
    }

    #[test]
    fn get_degree() {
        let scale = super::Diatonic::minor(&Tone::new(A, Natural));
        assert_eq!(
            scale.get_distance(&Tone::new(A, Natural), &Tone::new(A, Natural)),
            Some(Degree::new(1).unwrap())
        );
        assert_eq!(
            scale.get_distance(&Tone::new(C, Natural), &Tone::new(A, Natural)),
            Some(Degree::new(6).unwrap())
        );
        assert_eq!(
            scale.get_distance(&Tone::new(A, Natural), &Tone::new(C, Natural)),
            Some(Degree::new(3).unwrap())
        );
        assert_eq!(
            scale.get_distance(&Tone::new(A, Natural), &Tone::new(A, Sharp)),
            None
        );
    }

    #[test]
    fn get_tone_by_degree() {
        let scale = super::Diatonic::minor(&Tone::new(A, Natural));
        assert_eq!(
            scale.get_tone_by_degree_from_key(&Degree::new(1).unwrap()),
            &Tone::new(A, Natural)
        );
        assert_eq!(
            scale.get_tone_by_degree_from_key(&Degree::new(13).unwrap()),
            &Tone::new(F, Natural)
        );
    }
}
