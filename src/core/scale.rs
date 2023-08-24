use super::{
    interval::Interval,
    tone::{Chroma, Tone},
};

pub trait Scale {
    /// 主音
    fn key(&self) -> Tone;

    /// 取りうる主音からの距離
    fn distances(&self) -> Vec<i32>;

    fn chroma(&self) -> Chroma;

    fn tones(&self) -> Vec<Tone> {
        self.chroma()
            .tones_with_start(&self.key())
            .into_iter()
            .filter(|t| self.distance(t).is_some())
            .collect()
    }
}

impl<T: ?Sized + Scale> Interval for T {
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
    fn new(from: &Tone, to: &Tone) -> Self {
        Self {
            from: from.clone(),
            to: to.clone(),
        }
    }

    pub fn interval_in_step(&self) -> i32 {
        self.to.step() as i32 - self.from.step() as i32
    }
}
