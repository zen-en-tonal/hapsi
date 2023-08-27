use std::{error::Error, fmt::Display, str::FromStr};

use crate::prelude::{AccidentalSymbol, Tone, ToneSymbol};

impl FromStr for Tone {
    type Err = ParseToneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tone = match s.get(0..1) {
            Some(tone) => tone.parse(),
            None => Err(ParseToneError::InvalidTone),
        };
        let acci = match s.get(1..) {
            Some(acci) => acci.parse(),
            None => Ok(AccidentalSymbol::Natural),
        };
        match (tone, acci) {
            (Ok(tone), Ok(acci)) => Ok(Tone::new(tone, acci)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}

impl FromStr for ToneSymbol {
    type Err = ParseToneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" => Ok(ToneSymbol::A),
            "b" => Ok(ToneSymbol::B),
            "c" => Ok(ToneSymbol::C),
            "d" => Ok(ToneSymbol::D),
            "e" => Ok(ToneSymbol::E),
            "f" => Ok(ToneSymbol::F),
            "g" => Ok(ToneSymbol::G),
            _ => Err(ParseToneError::InvalidTone),
        }
    }
}

impl FromStr for AccidentalSymbol {
    type Err = ParseToneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "♯" => Ok(AccidentalSymbol::Sharp),
            "s" => Ok(AccidentalSymbol::Sharp),
            "♭" => Ok(AccidentalSymbol::Flat),
            "f" => Ok(AccidentalSymbol::Flat),
            "♮" => Ok(AccidentalSymbol::Natural),
            "" => Ok(AccidentalSymbol::Natural),
            _ => Err(ParseToneError::InvalidAccidential),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseToneError {
    InvalidTone,
    InvalidAccidential,
}

impl Display for ParseToneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ParseToneError::InvalidTone => {
                "Invalid tone symbol: tone symbol {'a', 'b', 'c', 'd', 'e', 'f', 'g'} is accepted"
            }
            ParseToneError::InvalidAccidential => "Invalid accidental symbol",
        };
        write!(f, "{message}")
    }
}

impl Error for ParseToneError {}

#[cfg(test)]
mod tests {
    use crate::{
        parse::tone::ParseToneError,
        prelude::{AccidentalSymbol, Tone, ToneSymbol},
    };

    #[test]
    fn parse_tone_symbol() {
        assert_eq!("a".parse(), Ok(ToneSymbol::A));
        assert_eq!("ab".parse::<ToneSymbol>(), Err(ParseToneError::InvalidTone));
        assert_eq!("".parse::<ToneSymbol>(), Err(ParseToneError::InvalidTone));
    }

    #[test]
    fn parse_accidential() {
        assert_eq!("♯".parse(), Ok(AccidentalSymbol::Sharp));
        assert_eq!("s".parse(), Ok(AccidentalSymbol::Sharp));
        assert_eq!("♭".parse(), Ok(AccidentalSymbol::Flat));
        assert_eq!("f".parse(), Ok(AccidentalSymbol::Flat));
        assert_eq!("♮".parse(), Ok(AccidentalSymbol::Natural));
        assert_eq!("".parse(), Ok(AccidentalSymbol::Natural));
        assert_eq!(
            "ab".parse::<AccidentalSymbol>(),
            Err(ParseToneError::InvalidAccidential)
        );
        assert_eq!(
            "a".parse::<AccidentalSymbol>(),
            Err(ParseToneError::InvalidAccidential)
        );
    }

    #[test]
    fn parse_tone() {
        assert_eq!(
            "A".parse::<Tone>(),
            Ok(Tone::new(ToneSymbol::A, AccidentalSymbol::Natural))
        );
        assert_eq!(
            "A♯".parse::<Tone>(),
            Ok(Tone::new(ToneSymbol::A, AccidentalSymbol::Sharp))
        );
    }
}
