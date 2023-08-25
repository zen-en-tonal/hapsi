use crate::core::*;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Chord(Vec<Degree>);

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Quality {
    Major,
    Minor,
}

impl Chord {
    pub fn degrees(&self) -> Vec<Degree> {
        self.0.clone()
    }

    pub fn new(vec: Vec<Degree>) -> Self {
        Chord(vec)
    }
}

pub trait ChordScale {
    type ToneLike: ToneLike;

    fn avoids(&self, chord: &Chord) -> Vec<Self::ToneLike>;

    fn chord_tones(&self, chord: &Chord) -> Vec<Self::ToneLike>;
}

impl<T: Scale> ChordScale for T {
    type ToneLike = T::ToneLike;

    fn avoids(&self, chord: &Chord) -> Vec<Self::ToneLike> {
        let mut vec = self
            .chord_tones(chord)
            .into_iter()
            .flat_map(|t| {
                vec![
                    self.chroma().tone(t.step() as i32 - 1),
                    self.chroma().tone(t.step() as i32 + 1),
                ]
            })
            .filter(|t| self.is_on_scale(t))
            .collect::<Vec<Self::ToneLike>>();
        vec.dedup();
        vec
    }

    fn chord_tones(&self, chord: &Chord) -> Vec<Self::ToneLike> {
        chord
            .degrees()
            .iter()
            .map(|d| self.get_by_degree(d))
            .collect()
    }
}
