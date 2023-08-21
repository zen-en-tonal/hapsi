#[derive(Debug, Clone, Default)]
pub(super) struct MajorScaleWalker {
    index: usize,
}

impl Iterator for MajorScaleWalker {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 6 {
            return None;
        }
        let interval = if [2, 6].contains(&self.index) { 1 } else { 2 };
        self.index += 1;
        Some(interval)
    }
}

#[cfg(test)]
mod tests {
    use crate::tonality::scale::major_scale_walker::MajorScaleWalker;

    #[test]
    fn major_scale_walker_walks_correctly() {
        let mut walker = MajorScaleWalker::default();
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 1);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 1);
        assert_eq!(walker.next(), None);
    }
}
