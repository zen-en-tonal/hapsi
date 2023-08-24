use crate::core::scale::Scale;
use crate::core::tone::{Chroma, Tone};
use crate::twelve_tet;

#[derive(Debug, Clone, Copy)]
pub struct Diatonic {
    key: Tone,
    quality: Quality,
}

impl Diatonic {
    pub fn major(key: &twelve_tet::tone::Tone) -> Self {
        Self {
            key: key.clone().into(),
            quality: Quality::Major,
        }
    }

    pub fn minor(key: &twelve_tet::tone::Tone) -> Self {
        Self {
            key: key.clone().into(),
            quality: Quality::Minor,
        }
    }

    pub fn quality(&self) -> Quality {
        self.quality
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Quality {
    Major,
    Minor,
}

impl Scale for Diatonic {
    fn key(&self) -> Tone {
        self.key.into()
    }

    fn distances(&self) -> Vec<i32> {
        match self.quality() {
            Quality::Major => [0, 2, 4, 5, 7, 9, 11].to_vec(),
            Quality::Minor => [0, 2, 3, 5, 7, 8, 10].to_vec(),
        }
    }

    fn chroma(&self) -> Chroma {
        Chroma::new(12)
    }
}
