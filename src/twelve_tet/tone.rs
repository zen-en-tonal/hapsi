use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::core::{Cycle, Number, Octave};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<Tone> for usize {
    fn from(value: Tone) -> Self {
        (value.tone as i32 + value.accidental as i32) as usize
    }
}

static TWELVE_MAP: Lazy<HashMap<usize, Tone>> = Lazy::new(|| {
    use AccidentalSymbol::*;
    use ToneSymbol::*;
    let mut map = HashMap::<usize, Tone>::new();
    map.insert(0, Tone::new(C, Natural));
    map.insert(1, Tone::new(C, Sharp));
    map.insert(2, Tone::new(D, Natural));
    map.insert(3, Tone::new(D, Sharp));
    map.insert(4, Tone::new(E, Natural));
    map.insert(5, Tone::new(F, Natural));
    map.insert(6, Tone::new(F, Sharp));
    map.insert(7, Tone::new(G, Natural));
    map.insert(8, Tone::new(G, Sharp));
    map.insert(9, Tone::new(A, Natural));
    map.insert(10, Tone::new(A, Sharp));
    map.insert(11, Tone::new(B, Natural));
    map
});

#[derive(Debug)]
pub struct Twelve;

impl Octave for Twelve {
    type PitchClass = Tone;

    fn get_class(&self, number: &Cycle<impl Number>) -> &Self::PitchClass {
        TWELVE_MAP.get_class(number)
    }

    fn get_number(&self, class: &Self::PitchClass) -> Option<usize> {
        TWELVE_MAP.get_number(class)
    }

    fn len(&self) -> usize {
        12
    }
}
