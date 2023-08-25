use crate::chord::Chord;
use crate::core::*;
use crate::prelude::Chroma;
use crate::prelude::Tone;

pub fn detect_chord(
    scale: &impl Scale<ToneLike = Tone, ChromaLike = Chroma>,
    tones: &[Tone],
) -> Chord {
    let deg = tones
        .iter()
        .map(|t| scale.distance(t))
        .filter(|option| option.is_some())
        .map(|some| some.unwrap())
        .collect();
    Chord::new(deg)
}
