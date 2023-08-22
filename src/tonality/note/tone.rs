use crate::tonality::Pitch;

use super::{
    symbol::{AccidentalSymbol, ToneSymbol},
    Divided,
};

#[derive(Debug, Clone, Hash)]
pub struct Tone {
    tone: ToneSymbol,
    accidental: AccidentalSymbol,
}

impl Tone {
    ///
    /// Toneインスタンスを生成する。
    ///
    /// ## Note
    ///
    /// 生成されたToneインスタンスは正規化される。
    ///
    /// ```rust
    /// # use hapsi::tonality::{AccidentalSymbol::*, ToneSymbol::*, Tone};
    /// assert_eq!(Tone::new(C, Flat), Tone::new(B, Natural))
    /// ```
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

    pub fn flip_accidential(self) -> Self {
        match self.accidental() {
            AccidentalSymbol::Flat => self.into_accidental_sharp(),
            AccidentalSymbol::Natural => self,
            AccidentalSymbol::Sharp => self.into_accidental_flat(),
        }
    }

    ///
    /// アクシデンタルを同位なフラットに変換する。
    ///
    /// ## Example
    ///
    /// ```rust
    /// use hapsi::tonality::{AccidentalSymbol::*, ToneSymbol::*, Tone};
    /// assert_eq!(
    ///     Tone::new(C, Sharp).into_accidental_flat(),
    ///     Tone::new(D, Flat)
    /// );
    /// ```
    ///
    /// ## Note
    ///
    /// ナチュラルの場合は変換しない。
    /// ```rust
    /// # use hapsi::tonality::{AccidentalSymbol::*, ToneSymbol::*, Tone};
    /// assert_eq!(
    ///     Tone::new(C, Natural).into_accidental_flat(),
    ///     Tone::new(C, Natural)
    /// );
    /// ```
    ///
    /// 既にフラットの場合は変換しない。
    /// ```rust
    /// # use hapsi::tonality::{AccidentalSymbol::*, ToneSymbol::*, Tone};
    /// assert_eq!(
    ///    Tone::new(C, Flat).into_accidental_flat(),
    ///    Tone::new(C, Flat)
    /// );
    /// ```
    pub fn into_accidental_flat(self) -> Self {
        use AccidentalSymbol::*;
        match self.accidental() {
            Flat => self,
            Natural => self,
            Sharp => Self::new(self.tone.transpose(1), Flat),
        }
    }

    ///
    /// アクシデンタルを同位なシャープに変換する。
    ///
    /// ## Example
    ///
    /// ```rust
    /// use hapsi::tonality::{AccidentalSymbol::*, ToneSymbol::*, Tone};
    /// assert_eq!(
    ///     Tone::new(D, Flat).into_accidental_sharp(),
    ///     Tone::new(C, Sharp)
    /// );
    /// ```
    ///
    /// ## Note
    ///
    /// ナチュラルの場合は変換しない。
    /// ```rust
    /// # use hapsi::tonality::{AccidentalSymbol::*, ToneSymbol::*, Tone};
    /// assert_eq!(
    ///     Tone::new(C, Natural).into_accidental_sharp(),
    ///     Tone::new(C, Natural)
    /// );
    /// ```
    ///
    /// 既にシャープの場合は変換しない。
    /// ```rust
    /// # use hapsi::tonality::{AccidentalSymbol::*, ToneSymbol::*, Tone};
    /// assert_eq!(
    ///    Tone::new(C, Sharp).into_accidental_sharp(),
    ///    Tone::new(C, Sharp)
    /// );
    /// ```
    pub fn into_accidental_sharp(self) -> Self {
        use AccidentalSymbol::*;
        match self.accidental() {
            Sharp => self,
            Natural => self,
            Flat => Self::new(self.tone.transpose(-1), Sharp),
        }
    }
}

impl Divided for Tone {
    fn num_devided() -> usize {
        12
    }

    fn index(&self) -> usize {
        let flat = self.clone().into_accidental_flat();
        match (flat.tone(), flat.accidental()) {
            (ToneSymbol::C, AccidentalSymbol::Natural) => 0,
            (ToneSymbol::D, AccidentalSymbol::Flat) => 1,
            (ToneSymbol::D, AccidentalSymbol::Natural) => 2,
            (ToneSymbol::E, AccidentalSymbol::Flat) => 3,
            (ToneSymbol::E, AccidentalSymbol::Natural) => 4,
            (ToneSymbol::F, AccidentalSymbol::Natural) => 5,
            (ToneSymbol::G, AccidentalSymbol::Flat) => 6,
            (ToneSymbol::G, AccidentalSymbol::Natural) => 7,
            (ToneSymbol::A, AccidentalSymbol::Flat) => 8,
            (ToneSymbol::A, AccidentalSymbol::Natural) => 9,
            (ToneSymbol::B, AccidentalSymbol::Flat) => 10,
            (ToneSymbol::B, AccidentalSymbol::Natural) => 11,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Tone {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index()
    }
}

impl PartialOrd for Tone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index().partial_cmp(&other.index())
    }
}

impl IntoIterator for Tone {
    type Item = Self;

    type IntoIter = ToneIter;

    fn into_iter(self) -> Self::IntoIter {
        ToneIter {
            start: self.index(),
            index: 0,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ToneIter {
    start: usize,
    index: usize,
}

impl Iterator for ToneIter {
    type Item = Tone;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index < 12 {
            true => {
                let current = Some((self.index + self.start).into());
                self.index += 1;
                current
            }
            false => None,
        }
    }
}

impl From<usize> for Tone {
    fn from(value: usize) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::tonality::note::{
        symbol::{AccidentalSymbol::*, ToneSymbol::*},
        tone::Tone,
        Pitch,
    };

    #[test]
    fn order() {
        assert!(Tone::new(C, Natural) < Tone::new(D, Natural));
        assert!(Tone::new(D, Flat) < Tone::new(D, Natural));
        assert!(Tone::new(C, Sharp) < Tone::new(D, Natural));
    }

    #[test]
    fn eq() {
        assert!(Tone::new(C, Natural) == Tone::new(C, Natural));
        assert!(Tone::new(C, Natural) != Tone::new(D, Natural));
        assert!(Tone::new(D, Flat) == Tone::new(C, Sharp));
    }

    #[test]
    fn transpose() {
        assert_eq!(Tone::new(C, Natural).transpose(1), Tone::new(C, Sharp));
        assert_eq!(Tone::new(C, Natural).transpose(12), Tone::new(C, Natural));
        assert_eq!(Tone::new(C, Natural).transpose(-1), Tone::new(B, Natural));
        assert_eq!(Tone::new(C, Natural).transpose(-12), Tone::new(C, Natural));
        assert_eq!(Tone::new(C, Natural).transpose(0), Tone::new(C, Natural));
    }

    #[test]
    fn into_iter() {
        let mut tone = Tone::new(A, Natural).into_iter();
        assert_eq!(tone.next(), Some(Tone::new(A, Natural)));
        assert_eq!(tone.next(), Some(Tone::new(A, Sharp)));
        assert_eq!(tone.next(), Some(Tone::new(B, Natural)));
        assert_eq!(tone.next(), Some(Tone::new(C, Natural)));
        assert_eq!(tone.next(), Some(Tone::new(C, Sharp)));
        assert_eq!(tone.next(), Some(Tone::new(D, Natural)));
        assert_eq!(tone.next(), Some(Tone::new(D, Sharp)));
        assert_eq!(tone.next(), Some(Tone::new(E, Natural)));
        assert_eq!(tone.next(), Some(Tone::new(F, Natural)));
        assert_eq!(tone.next(), Some(Tone::new(F, Sharp)));
        assert_eq!(tone.next(), Some(Tone::new(G, Natural)));
        assert_eq!(tone.next(), Some(Tone::new(G, Sharp)));
        assert_eq!(tone.next(), None);
    }
}
