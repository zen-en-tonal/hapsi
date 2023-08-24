use super::tone::{Chroma, Tone};

/// 基準からの距離
pub trait Interval {
    type Interval;
    fn distance(&self, to: &Tone) -> Option<Self::Interval>;
}

pub trait Scale {
    /// 主音
    fn key(&self) -> Tone;

    /// 取りうる主音からの距離
    fn distances(&self) -> Vec<i32>;

    fn chroma(&self) -> Chroma;

    fn tones(&self) -> Vec<Tone> {
        self.chroma()
            .tones()
            .into_iter()
            .filter(|t| self.distance(t).is_some())
            .collect()
    }
}

impl<T: Scale + ?Sized> Interval for T {
    type Interval = Degree;

    fn distance(&self, to: &Tone) -> Option<Self::Interval> {
        let distance = self.chroma().distance(&self.key(), to) as i32;
        match self.distances().contains(&distance) {
            true => Some(Degree::new(&self.key(), to)),
            false => None,
        }
    }
}

pub struct Degree {
    from: Tone,
    to: Tone,
}

impl Degree {
    pub fn new(from: &Tone, to: &Tone) -> Self {
        Self {
            from: from.clone(),
            to: to.clone(),
        }
    }
}
