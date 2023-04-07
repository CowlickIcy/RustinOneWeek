pub use core::*;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = f64::consts::PI;
pub const GAMMA: f64 = 2.2;

pub fn degress_to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn ffmin(lhs: f64, rhs: f64) -> f64 {
    if lhs <= rhs {
        lhs
    } else {
        rhs
    }
}

pub fn ffmax(lhs: f64, rhs: f64) -> f64 {
    if lhs >= rhs {
        lhs
    } else {
        rhs
    }
}

