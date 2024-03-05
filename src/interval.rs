use crate::INFINITY;

pub struct Interval {
    min: f64,
    max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self::UNIVERSE
    }
}

impl Interval {
    pub const EMPTY: Self = Self::new(INFINITY, -INFINITY);
    pub const UNIVERSE: Self = Self::new(-INFINITY, INFINITY);

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub const fn min(&self) -> f64 {
        self.min
    }

    pub const fn max(&self) -> f64 {
        self.max
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min() <= x && x <= self.max()
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min() < x && x < self.max()
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
