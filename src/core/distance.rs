use std::{error::Error, fmt::Display};

use super::Number;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Interval(i32);

impl Interval {
    pub fn new<T: Into<usize>>(a: T, b: T) -> Self {
        let a: usize = a.into();
        let b: usize = b.into();
        if a > b {
            Self(-1 * (a - b) as i32)
        } else {
            Self((b - a) as i32)
        }
    }

    pub fn value(&self) -> usize {
        self.0.abs() as usize
    }

    pub fn direction(&self) -> i32 {
        self.0.signum()
    }
}

impl From<usize> for Interval {
    fn from(value: usize) -> Self {
        Interval(value as i32)
    }
}

impl From<i32> for Interval {
    fn from(value: i32) -> Self {
        Interval(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Degree(usize);

impl Degree {
    /// Returns a `Degree` instance.
    ///
    /// # Errors
    /// - if `value` is less than 1, returns error.
    pub fn new(value: usize) -> Result<Self, DegreeConstructError> {
        if value < 1 {
            Err(DegreeConstructError)
        } else {
            Ok(Degree(value))
        }
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

impl Number for Degree {
    fn value(&self) -> usize {
        self.0 - 1
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DegreeConstructError;

impl Display for DegreeConstructError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "degree must be greater than 0")
    }
}

impl Error for DegreeConstructError {}
