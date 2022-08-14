use std::{
    fmt,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate<T> {
    pub y: T,
    pub x: T,
}

impl<T> Coordinate<T> {
    pub fn new(y: T, x: T) -> Self {
        Self { y, x }
    }
}

impl<T: Add<Output = T>> Add for Coordinate<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            y: self.y - other.y,
            x: self.x - other.x,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Coordinate<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(y={}, x={})", self.y, self.x)
    }
}
