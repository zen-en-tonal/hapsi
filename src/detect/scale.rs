use crate::prelude::*;

pub fn detect_scale(tones: &[Tone]) -> Diatonic {
    Chroma
        .tones_with_start(&Tone::new(C, Natural))
        .into_iter()
        .map(|t| Diatonic::major(&t.try_into().unwrap()))
        .map(|scale| (scale.score(tones), scale))
        .max_by_key(|t| t.0)
        .unwrap()
        .1
}

pub trait Detect {
    fn score(&self, tones: &[Tone]) -> i32;
}

impl<T: Scale<ToneLike = Tone, ChromaLike = Chroma>> Detect for T {
    fn score(&self, tones: &[Tone]) -> i32 {
        tones
            .iter()
            .map(|t| if self.is_on_scale(t) { 1 } else { 0 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::detect_scale;
    use crate::prelude::*;

    #[test]
    fn detect() {
        let tones = vec![
            Tone::new(C, Natural),
            Tone::new(D, Natural),
            Tone::new(E, Natural),
            Tone::new(F, Natural),
            Tone::new(G, Natural),
            Tone::new(A, Natural),
            Tone::new(B, Natural),
        ];
        let scale = detect_scale(tones.as_slice());
        assert_eq!(scale, Diatonic::major(&Tone::new(C, Natural)))
    }
}
