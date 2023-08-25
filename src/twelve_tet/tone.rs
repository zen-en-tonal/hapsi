use crate::core::{ChromaLike, ToneLike};

#[derive(Debug, Clone, Copy, Hash, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Eq)]
#[repr(i32)]
pub enum AccidentalSymbol {
    Flat = -1,
    Natural = 0,
    Sharp = 1,
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

pub struct Chroma;

impl ChromaLike for Chroma {
    type Tone = Tone;

    fn tone(&self, step: i32) -> Self::Tone {
        let step = step % self.size() as i32;
        if step < 0 {
            return self.tone(self.size() as i32 + step);
        }
        step.into()
    }

    fn size(&self) -> usize {
        12
    }

    fn distance(&self, from: &Self::Tone, to: &Self::Tone) -> usize {
        match to >= from {
            true => to.step() - from.step(),
            false => to.step() + self.size() - from.step(),
        }
    }
}

impl From<i32> for Tone {
    fn from(value: i32) -> Self {
        use AccidentalSymbol::*;
        use ToneSymbol::*;
        match value % 12 {
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
        }
    }
}
