use crate::prelude::*;

#[derive(Debug)]
pub struct Diatonic {
    key: Tone,
    quality: Quality,
    distances: [usize; 7],
}

impl Diatonic {
    pub fn major(key: &Tone) -> Scaled<Diatonic, Twelve> {
        Scaled::new(
            Self {
                key: *key,
                quality: Quality::Major,
                distances: [0, 2, 4, 5, 7, 9, 11],
            },
            Twelve,
        )
    }

    pub fn minor(key: &Tone) -> Scaled<Diatonic, Twelve> {
        Scaled::new(
            Self {
                key: *key,
                quality: Quality::Minor,
                distances: [0, 2, 3, 5, 7, 8, 10],
            },
            Twelve,
        )
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
    fn convert(&self, number: impl Number) -> usize {
        let i: usize = self.key.into();
        i + self.distances.get(number.value()).unwrap().clone()
    }

    fn len(&self) -> usize {
        7
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn major() {
        let scaled = super::Diatonic::major(&Tone::new(C, Natural));
        let keyboard = Keyboard::new(scaled);
        let mut tones = keyboard.class_iter();
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
        let scaled = super::Diatonic::minor(&Tone::new(A, Natural));
        let keyboard = Keyboard::new(scaled);
        let mut tones = keyboard.class_iter();
        assert_eq!(tones.next(), Some(&Tone::new(A, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(B, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(C, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(D, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(E, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(F, Natural)));
        assert_eq!(tones.next(), Some(&Tone::new(G, Natural)));
        assert_eq!(tones.next(), None);
    }
}
