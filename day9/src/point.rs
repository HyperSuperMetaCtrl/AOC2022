use std::ops::{Add, AddAssign, Div, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
impl Div<isize> for Point {
    type Output = Self;

    fn div(self, rhs: isize) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl From<(isize, isize)> for Point {
    fn from(point: (isize, isize)) -> Self {
        Point {
            x: point.0,
            y: point.1,
        }
    }
}
impl Point {
    pub fn dist(self, rhs: Self) -> Self {
        rhs - self
    }
    pub fn abs_dist(self, rhs: Self) -> isize {
        num::integer::sqrt((rhs.x - self.x).pow(2) + (rhs.y - self.y).pow(2))
    }
    pub fn sign(self) -> Point {
        Point {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}
