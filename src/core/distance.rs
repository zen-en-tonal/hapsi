#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Interval(usize);

impl Interval {
    pub fn new<T: Into<usize>>(a: T, b: T) -> Self {
        let a: usize = a.into();
        let b: usize = b.into();
        if a > b {
            Self(a - b)
        } else {
            Self(b - a)
        }
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

impl From<usize> for Interval {
    fn from(value: usize) -> Self {
        Interval(value)
    }
}

impl From<Interval> for usize {
    fn from(value: Interval) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Degree(usize);

impl Degree {
    pub fn new(value: usize) -> Self {
        Degree(value)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}
