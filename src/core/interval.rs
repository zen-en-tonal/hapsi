use super::tone::Tone;

/// 基準からの距離
pub trait Interval {
    type Interval;
    fn distance(&self, to: &Tone) -> Option<Self::Interval>;
}
