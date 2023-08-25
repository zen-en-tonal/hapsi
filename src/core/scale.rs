use super::{
    interval::Interval,
    tone::{Chroma, Tone},
    transpose::Transpose,
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

    fn get_by_degree(&self, degree: &Degree) -> Tone {
        let interval = self.get_interval_by_degree(degree);
        self.key().transpose(interval)
    }

    // スケールの本質は、Degree <=> Intervalに変換する関数。
    fn get_interval_by_degree(&self, degree: &Degree) -> i32 {
        let binding = self.distances();
        *binding
            .get((degree.0 as usize - 1) % binding.len())
            .unwrap()
    }

    fn get_freq_rate_by_degree(&self, degree: &Degree) -> f32 {
        2.0_f32.powf(self.get_interval_by_degree(degree) as f32 / self.key().chroma_size() as f32)
    }
}

impl<T: ?Sized + Scale> Interval for T {
    type Interval = Degree;

    fn distance(&self, to: &Tone) -> Option<Self::Interval> {
        let distance = self.chroma().distance(&self.key(), to) as i32;
        match self.distances().iter().position(|d| d == &distance) {
            Some(index) => Some(Degree(index + 1)),
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Degree(usize);

impl Degree {
    pub fn new(value: usize) -> Self {
        Degree(value)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}
