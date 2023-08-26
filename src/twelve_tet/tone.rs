use std::hash::Hash;

use crate::core::{ChromaLike, ToneLike};

#[derive(Debug, Clone, Copy, Eq, Default)]
pub struct Tone {
    tone: ToneSymbol,
    accidental: AccidentalSymbol,
}

impl Tone {
    pub fn new(tone: ToneSymbol, accidental: AccidentalSymbol) -> Self {
        Tone { tone, accidental }.normalize()
    }

    pub fn tone(&self) -> &ToneSymbol {
        &self.tone
    }

    pub fn accidental(&self) -> &AccidentalSymbol {
        &self.accidental
    }

    fn normalize(self) -> Self {
        use AccidentalSymbol::*;
        use ToneSymbol::*;
        match (&self.tone, &self.accidental) {
            (&C, &Flat) => Self::new(B, Natural),
            (&E, &Sharp) => Self::new(F, Natural),
            (&F, &Flat) => Self::new(E, Natural),
            (&B, &Sharp) => Self::new(C, Natural),
            (_, _) => self,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq)]
#[repr(i32)]
pub enum ToneSymbol {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl Default for ToneSymbol {
    fn default() -> Self {
        Self::C
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq)]
#[repr(i32)]
pub enum AccidentalSymbol {
    Flat = -1,
    Natural = 0,
    Sharp = 1,
}

impl Default for AccidentalSymbol {
    fn default() -> Self {
        Self::Natural
    }
}

impl ToneLike for Tone {
    fn step(&self) -> usize {
        (self.tone as i32 + self.accidental as i32) as usize
    }

    fn chroma_size(&self) -> usize {
        12
    }
}

impl PartialEq for Tone {
    fn eq(&self, other: &Self) -> bool {
        self.step() == other.step()
    }
}

impl PartialOrd for Tone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.step().partial_cmp(&other.step())
    }
}

impl Hash for Tone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.chroma_size().hash(state);
        self.step().hash(state);
    }
}

#[derive(Debug)]
pub struct Chroma([Tone; 12]);

impl Chroma {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Chroma {
    fn default() -> Self {
        let mut array: [Tone; 12] = Default::default();
        for (i, _) in array.into_iter().enumerate() {
            array[i] = convert_usize_to_tone(i).unwrap()
        }
        Self(array)
    }
}

impl ChromaLike for Chroma {
    type Tone = Tone;

    fn get_exactly(&self, index: usize) -> Option<&Self::Tone> {
        self.0.get(index)
    }

    fn size(&self) -> usize {
        self.0.len()
    }
}

fn convert_usize_to_tone(index: usize) -> Option<Tone> {
    use AccidentalSymbol::*;
    use ToneSymbol::*;

    if index > 11 {
        return None;
    }

    Some(match index {
        0 => Tone::new(C, Natural),
        1 => Tone::new(C, Sharp),
        2 => Tone::new(D, Natural),
        3 => Tone::new(D, Sharp),
        4 => Tone::new(E, Natural),
        5 => Tone::new(F, Natural),
        6 => Tone::new(F, Sharp),
        7 => Tone::new(G, Natural),
        8 => Tone::new(G, Sharp),
        9 => Tone::new(A, Natural),
        10 => Tone::new(A, Sharp),
        11 => Tone::new(B, Natural),
        _ => unreachable!(),
    })
}
