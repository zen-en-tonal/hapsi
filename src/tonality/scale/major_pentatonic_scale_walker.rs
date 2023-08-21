#[derive(Debug, Clone, Default)]
pub(super) struct MajorPentatonicScaleWalker {
    index: usize,
}

impl Iterator for MajorPentatonicScaleWalker {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 4 {
            return None;
        }
        let interval = if self.index == 2 { 3 } else { 2 };
        self.index += 1;
        Some(interval)
    }
}

#[cfg(test)]
mod tests {
    use super::MajorPentatonicScaleWalker;

    #[test]
    fn major_pentatonic_scale_walker_walks_correctly() {
        let mut walker = MajorPentatonicScaleWalker::default();
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 3);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next().unwrap(), 2);
        assert_eq!(walker.next(), None);
    }
}
