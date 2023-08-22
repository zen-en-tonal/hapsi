use crate::tonality::{Divided, Tone};

use super::Scale;

#[derive(Debug, Clone)]
pub struct MinorScale(Tone);

impl MinorScale {
    pub fn new(key: &Tone) -> Self {
        Self(key.clone())
    }
}

impl Scale for MinorScale {
    fn tonic(&self) -> &Tone {
        &self.0
    }

    fn is_on_scale(&self, tone: &Tone) -> bool {
        let distance = (self.tonic().index() as i32 - tone.index() as i32).abs();
        ![1, 3, 6, 8, 10].contains(&distance)
    }
}

#[cfg(test)]
mod tests {
    use super::MinorScale;
    use crate::tonality::*;
    use AccidentalSymbol::*;
    use ToneSymbol::*;

    #[test]
    fn minor_scale_works_correctly() {
        let mut scale = MinorScale::new(&Tone::new(A, Natural)).into_scaler();
        assert_eq!(scale.next(), Some(Tone::new(A, Natural)));
        assert_eq!(scale.next(), Some(Tone::new(B, Natural)));
        assert_eq!(scale.next(), Some(Tone::new(C, Natural)));
        assert_eq!(scale.next(), Some(Tone::new(D, Natural)));
        assert_eq!(scale.next(), Some(Tone::new(E, Natural)));
        assert_eq!(scale.next(), Some(Tone::new(F, Natural)));
        assert_eq!(scale.next(), Some(Tone::new(G, Natural)));
        assert_eq!(scale.next(), None);
    }
}
