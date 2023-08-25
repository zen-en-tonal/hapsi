use super::tone::{ChromaLike, ToneLike};

pub trait Scale {
    type ToneLike: ToneLike;
    type ChromaLike: ChromaLike<Tone = Self::ToneLike>;

    /// 主音
    fn key(&self) -> Self::ToneLike;

    /// 取りうる主音からの距離
    fn distances(&self) -> Vec<i32>;

    fn chroma(&self) -> Self::ChromaLike;

    fn tones(&self) -> Vec<Self::ToneLike> {
        self.chroma()
            .tones_with_start(&self.key())
            .into_iter()
            .filter(|t| self.distance(t).is_some())
            .collect()
    }

    fn is_on_scale(&self, tone: &Self::ToneLike) -> bool {
        self.tones().contains(tone)
    }

    fn get_by_degree(&self, degree: &Degree) -> Self::ToneLike {
        let interval = self.get_interval_by_degree(degree);
        self.chroma().tone(self.key().step() as i32 + interval)
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

    fn distance(&self, to: &Self::ToneLike) -> Option<Degree> {
        let distance = self.chroma().distance(&self.key(), to) as i32;
        match self.distances().iter().position(|d| d == &distance) {
            Some(index) => Some(Degree(index + 1)),
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Degree(usize);

impl Degree {
    pub fn new(value: usize) -> Self {
        Degree(value)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}
