use std::mem::swap;

use crate::rtweekend::interval;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Point3;

#[derive(Clone, Debug)]
pub struct Aabb {
    pub b: [Interval; 3],
}
pub const EMPTY: Aabb = Aabb {
    b: [interval::EMPTY, interval::EMPTY, interval::EMPTY],
};
pub const _UNIVERSE: Aabb = Aabb {
    b: [interval::UNIVERSE, interval::UNIVERSE, interval::UNIVERSE],
};
impl Aabb {
    pub fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.b[0].size() < delta {
            self.b[0] = self.b[0].expand(delta);
        }
        if self.b[1].size() < delta {
            self.b[1] = self.b[1].expand(delta);
        }
        if self.b[2].size() < delta {
            self.b[2] = self.b[2].expand(delta);
        }
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
pub fn point_to_aabb(a: &Point3, b: &Point3) -> Aabb {
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
    let mut aabb = Aabb { b: [x, y, z] };
    aabb.pad_to_minimums();
    aabb
}
pub fn merge(a: &Aabb, b: &Aabb) -> Aabb {
    let mut new_b = [interval::EMPTY; 3];
    for (i, item) in new_b.iter_mut().enumerate() {
        item.min = if a.b[i].min < b.b[i].min {
            a.b[i].min
        } else {
            b.b[i].min
        };
        item.max = if a.b[i].max > b.b[i].max {
            a.b[i].max
        } else {
            b.b[i].max
        };
    }
    Aabb { b: new_b }
}
