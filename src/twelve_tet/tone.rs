use std::{error::Error, fmt::Display};

use crate::core::{
    tone::{self, Chroma},
    transpose::Transpose,
};

#[derive(Debug, Clone, Copy, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
#[repr(i32)]
pub enum AccidentalSymbol {
    Flat = -1,
    Natural = 0,
    Sharp = 1,
}

impl From<Tone> for tone::Tone {
    fn from(value: Tone) -> Self {
        let chroma = Chroma::new(12);
        chroma.tone(value.into())
    }
}

impl TryFrom<tone::Tone> for Tone {
    type Error = NonTwelveTetError;

    fn try_from(value: tone::Tone) -> Result<Self, Self::Error> {
        use AccidentalSymbol::*;
        use ToneSymbol::*;

        if value.chroma_size() != 12 {
            return Err(NonTwelveTetError);
        }

        Ok(match value.step() {
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
}

#[derive(Debug)]
pub struct NonTwelveTetError;

impl Display for NonTwelveTetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for NonTwelveTetError {}

impl From<Tone> for i32 {
    fn from(value: Tone) -> Self {
        value.tone as i32 + value.accidental as i32
    }
}

impl PartialEq for Tone {
    fn eq(&self, other: &Self) -> bool {
        let s: i32 = self.clone().into();
        let o: i32 = other.clone().into();
        s == o
    }
}

impl PartialOrd for Tone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let s: i32 = self.clone().into();
        let o: i32 = other.clone().into();
        s.partial_cmp(&o)
    }
}

impl Transpose for Tone {
    fn transpose(self, transpose: i32) -> Self {
        let tone: tone::Tone = self.into();
        tone.transpose(transpose).try_into().unwrap()
    }
}
