#[derive(Debug, Clone, Default)]
pub(super) struct MinorScaleWalker {
    index: usize,
}

impl Iterator for MinorScaleWalker {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 6 {
            return None;
        }
        let interval = if [1, 4].contains(&self.index) { 1 } else { 2 };
        self.index += 1;
        Some(interval)
    }
}

#[cfg(test)]
mod tests {
    use super::MinorScaleWalker;

    #[test]
    fn minor_scale_walker_works_correctly() {
        let mut walker = MinorScaleWalker::default();
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 1);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 1);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next(), None);
    }
}
