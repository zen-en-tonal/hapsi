#[derive(Debug, Clone, Default)]
pub(super) struct MinorPentatonicScaleWalker {
    index: usize,
}

impl Iterator for MinorPentatonicScaleWalker {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 4 {
            return None;
        }
        let interval = if [0, 3].contains(&self.index) { 3 } else { 2 };
        self.index += 1;
        Some(interval)
    }
}

#[cfg(test)]
mod tests {
    use super::MinorPentatonicScaleWalker;

    #[test]
    fn minor_pentatonic_scale_walker_walks_correctly() {
        let mut walker = MinorPentatonicScaleWalker::default();
        assert_eq!(walker.next().unwrap(), 3);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 3);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next(), None);
    }
}
