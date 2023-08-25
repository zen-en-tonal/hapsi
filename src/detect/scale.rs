use crate::prelude::*;

pub fn detect_scale(tones: &[Tone]) -> Option<Diatonic> {
    let mut scales = Chroma
        .tones_with_start(&Tone::new(C, Natural))
        .into_iter()
        .map(|t| Diatonic::major(&t.try_into().unwrap()));
    scales.find(|s| s.is_satisfy(tones))
}

pub trait Detect {
    fn is_satisfy(&self, tone: &[Tone]) -> bool;
}

impl<T: Scale<ToneLike = Tone, ChromaLike = Chroma>> Detect for T {
    fn is_satisfy(&self, tones: &[Tone]) -> bool {
        tones.into_iter().all(|tone| self.tones().contains(tone))
    }
}

#[cfg(test)]
mod tests {
    use super::detect_scale;
    use crate::prelude::*;

    #[test]
    fn detect() {
        let tones = vec![Tone::new(C, Natural), Tone::new(A, Natural)];
        let scale = detect_scale(tones.as_slice());
        assert_eq!(scale, Some(Diatonic::major(&Tone::new(C, Natural))))
    }
}
