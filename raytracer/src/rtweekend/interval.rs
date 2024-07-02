use crate::rtweekend::INF;
pub struct Interval {
    pub min: f64,
    pub max: f64,
}
impl Interval {
    pub fn _size(&self) -> f64 {
        self.max - self.min
    }
    pub fn _contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}
pub const _EMPTY: Interval = Interval {
    min: INF,
    max: -INF,
};
pub const _UNIVERSE: Interval = Interval {
    min: -INF,
    max: INF,
};
