use std::fmt::Display;

use super::Divided;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
#[repr(u8)]
pub enum ToneSymbol {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl From<usize> for ToneSymbol {
    fn from(value: usize) -> Self {
        use ToneSymbol::*;
        match value % 7 {
            0 => C,
            1 => D,
            2 => E,
            3 => F,
            4 => G,
            5 => A,
            6 => B,
            _ => unreachable!(),
        }
    }
}

impl Divided for ToneSymbol {
    fn num_devided() -> usize {
        7
    }

    fn index(&self) -> usize {
        (self.clone() as u8) as usize
    }
}

impl Display for ToneSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self {
            ToneSymbol::C => "C",
            ToneSymbol::D => "D",
            ToneSymbol::E => "E",
            ToneSymbol::F => "F",
            ToneSymbol::G => "G",
            ToneSymbol::A => "A",
            ToneSymbol::B => "B",
        };
        write!(f, "{letter}")
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum AccidentalSymbol {
    Flat,
    Natural,
    Sharp,
}

impl Display for AccidentalSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match self {
            AccidentalSymbol::Flat => "♭",
            AccidentalSymbol::Natural => "",
            AccidentalSymbol::Sharp => "#",
        };
        write!(f, "{letter}")
    }
}

#[cfg(test)]
mod tests {
    use crate::tonality::note::{symbol::ToneSymbol::*, Pitch};

    #[test]
    fn transpose() {
        assert_eq!(C.transpose(1), D);
        assert_eq!(C.transpose(7), C);
        assert_eq!(C.transpose(-1), B);
        assert_eq!(C.transpose(-7), C);
        assert_eq!(C.transpose(0), C);
    }
}
