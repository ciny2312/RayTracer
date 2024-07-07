use std::mem::swap;

use crate::rtweekend::interval;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Point3;

#[derive(Clone, Debug)]
pub struct AABB {
    pub b: [Interval; 3],
}
pub const EMPTY: AABB = AABB {
    b: [interval::EMPTY, interval::EMPTY, interval::EMPTY],
};
pub const _UNIVERSE: AABB = AABB {
    b: [
        interval::_UNIVERSE,
        interval::_UNIVERSE,
        interval::_UNIVERSE,
    ],
};
impl AABB {
    pub fn point_to_aabb(a: &Point3, b: &Point3) -> Self {
        let mut x = Interval {
            min: a.e[0],
            max: b.e[0],
        };
        if x.min > x.max {
            swap(&mut x.min, &mut x.max);
        }
        let mut y = Interval {
            min: a.e[1],
            max: b.e[1],
        };
        if y.min > y.max {
            swap(&mut y.min, &mut y.max);
        }
        let mut z = Interval {
            min: a.e[2],
            max: b.e[2],
        };
        if z.min > z.max {
            swap(&mut z.min, &mut z.max);
        }
        Self { b: [x, y, z] }
    }
    pub fn merge(a: &AABB, b: &AABB) -> Self {
        let mut new_b = [interval::EMPTY; 3];
        for i in 0..3 {
            new_b[i].min = if a.b[i].min < b.b[i].min {
                a.b[i].min
            } else {
                b.b[i].min
            };
            new_b[i].max = if a.b[i].max > b.b[i].max {
                a.b[i].max
            } else {
                b.b[i].max
            };
        }
        Self { b: new_b }
    }
    pub fn longest_axis(&self) -> u32 {
        if self.b[0].size() > self.b[1].size() {
            if self.b[0].size() > self.b[2].size() {
                return 0;
            } else {
                return 2;
            }
        }
        if self.b[1].size() > self.b[2].size() {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let mut interval = Interval {
            min: ray_t.min,
            max: ray_t.max,
        };
        for axis in 0..3 {
            let ax = &self.b[axis];
            let adinv = 1.0 / r.dir.e[axis];

            let t0 = (ax.min - r.ori.e[axis]) * adinv;
            let t1 = (ax.max - r.ori.e[axis]) * adinv;

            if t0 < t1 {
                if t0 > interval.min {
                    interval.min = t0;
                }
                if t1 < interval.max {
                    interval.max = t1;
                }
            } else {
                if t1 > interval.min {
                    interval.min = t1;
                }
                if t0 < interval.max {
                    interval.max = t0;
                }
            }
        }
        interval.min < interval.max
    }
}
