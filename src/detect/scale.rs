use crate::{
    core::{scale::Scale, tone::Tone},
    scale::diatonic::{self, Diatonic},
    twelve_tet,
};

pub fn detect_scale(tones: &[Tone]) -> Option<Diatonic> {
    let mut scales = twelve_tet::tone::tones()
        .into_iter()
        .map(|t| diatonic::Diatonic::major(&t.try_into().unwrap()));
    scales.find(|s| s.is_satisfy(tones))
}

pub trait Detect {
    fn is_satisfy(&self, tone: &[Tone]) -> bool;
}

impl<T: Scale> Detect for T {
    fn is_satisfy(&self, tones: &[Tone]) -> bool {
        tones.into_iter().all(|tone| self.tones().contains(tone))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        scale::diatonic::Diatonic,
        twelve_tet::tone::{tone, AccidentalSymbol, Tone, ToneSymbol},
    };

    use super::detect_scale;

    #[test]
    fn detect() {
        let tones = vec![
            tone(ToneSymbol::C, AccidentalSymbol::Natural).into(),
            tone(ToneSymbol::A, AccidentalSymbol::Natural).into(),
        ];
        let scale = detect_scale(tones.as_slice());
        assert_eq!(
            scale,
            Some(Diatonic::major(&Tone::new(
                ToneSymbol::C,
                AccidentalSymbol::Natural
            )))
        )
    }
}
