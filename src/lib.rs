// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! todo

pub mod coords;

use crate::coords::*;
use num::{Float, FromPrimitive};
use num_complex::Complex;
use num_traits::float::FloatConst;
// use std::f64::consts::{FRAC_1_PI, SQRT_2};

pub fn sh00<T: Float + FloatConst + FromPrimitive>(_p: &impl Coordinates<T>) -> T {
    T::from_f64(0.5).unwrap() * T::FRAC_1_PI().sqrt()
}

pub fn sh1n1<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    (T::from_f64(3.0 / 4.0).unwrap() * T::FRAC_1_PI()).sqrt() * p.y() / p.r()
}

pub fn sh10<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    (T::from_f64(3.0 / 4.0).unwrap() * T::FRAC_1_PI()).sqrt() * p.z() / p.r()
}

pub fn sh1p1<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    (T::from_f64(3.0 / 4.0).unwrap() * T::FRAC_1_PI()).sqrt() * p.x() / p.r()
}

pub fn sh2n2<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.5).unwrap()
        * (T::from_f64(15.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.x() * p.y())
        / p.r().powi(2)
}

pub fn sh2n1<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.5).unwrap()
        * (T::from_f64(15.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.y() * p.z())
        / p.r().powi(2)
}

pub fn sh20<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(15.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (-p.x().powi(2) - p.y().powi(2) * T::from_f64(2.0).unwrap() * p.z().powi(2))
        / p.r().powi(2)
}

pub fn sh2p1<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.5).unwrap()
        * (T::from_f64(15.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.z() * p.x())
        / p.r().powi(2)
}

pub fn sh2p2<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(15.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.x().powi(2) * p.y().powi(2))
        / p.r().powi(2)
}

pub fn sh3n3<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(35.0 / 2.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (T::from_f64(3.0).unwrap() * p.x().powi(2) - p.y().powi(2))
        * p.y()
        / p.r().powi(3)
}

pub fn sh3n2<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.5).unwrap()
        * (T::from_f64(105.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * (p.x() * p.y() * p.z())
        / p.r().powi(3)
}

pub fn sh3n1<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(21.0 / 2.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * p.y()
        * (T::from_f64(4.0).unwrap() * p.z().powi(2) - p.x().powi(2) - p.y().powi(2))
        / p.r().powi(3)
}

pub fn sh30<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(7.0).unwrap() * T::FRAC_1_PI()).sqrt()
        * p.z()
        * (T::from_f64(2.0).unwrap() * p.z().powi(2)
            - T::from_f64(3.0).unwrap() * p.x().powi(2)
            - T::from_f64(3.0).unwrap() * p.y().powi(2))
        / p.r().powi(3)
}

pub fn sh3p1<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(21.0 / 2.0).unwrap() * T::FRAC_1_PI())
        * p.x()
        * (T::from_f64(4.0).unwrap() * p.z().powi(2) - p.x().powi(2) - p.y().powi(2))
        / p.r().powi(3)
}

pub fn sh3p2<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(105.0).unwrap() * T::FRAC_1_PI())
        * (p.x().powi(2) - p.y().powi(2))
        * p.z()
        / p.r().powi(3)
}

pub fn sh3p3<T: Float + FloatConst + FromPrimitive>(p: &impl Coordinates<T>) -> T {
    T::from_f64(0.25).unwrap()
        * (T::from_f64(35.0 / 2.0).unwrap() * T::FRAC_1_PI())
        * (p.x().powi(2) - T::from_f64(3.0).unwrap() * p.y().powi(2))
        * p.x().powi(2)
        / p.r().powi(3)
}

#[inline]
fn factorial(n: i64) -> i64 {
    (1..=n).fold(1, |acc, x| acc * x)
}

#[allow(non_snake_case)]
#[inline]
fn K<T: Float + FloatConst + FromPrimitive>(l: i64, m: i64) -> T {
    let m = m.abs();
    (T::FRAC_1_PI() * T::from_i64(2 * l + 1).unwrap() / T::from_f64(4.0).unwrap()
        * T::from_i64(factorial(l - m)).unwrap()
        / T::from_i64(factorial(l + m)).unwrap())
    .sqrt()
}

#[allow(non_snake_case)]
#[inline]
fn P<T: Float + FloatConst + FromPrimitive>(l: i64, m: i64, x: T) -> T {
    let mut pmm = T::one();

    if m > 0 {
        let somx2 = ((T::one() - x) * (T::one() + x)).sqrt();
        let mut fact = T::one();
        for _ in 1..=m {
            pmm = pmm * -fact * somx2;
            fact = fact + T::from_f64(2.0).unwrap();
        }
    }

    if l == m {
        return pmm;
    }

    let mut pmmp1 = x * T::from_i64(2 * m + 1).unwrap() * pmm;

    if l == m + 1 {
        return pmmp1;
    }

    let mut pll = T::zero();
    for ll in (m + 2 + 1)..=l {
        pll = (T::from_i64(2 * ll - 1).unwrap() * x * pmmp1
            - (T::from_i64(ll + m - 1)).unwrap() * pmm)
            / T::from_i64(ll - m).unwrap();
        pmm = pmmp1;
        pmmp1 = pll;
    }
    pll
}

#[allow(non_snake_case)]
#[inline]
pub fn SH<T: Float + FloatConst + FromPrimitive>(
    l: i64,
    m: i64,
    p: &impl Coordinates<T>,
) -> Complex<T> {
    let v: T = if m == 0 {
        K::<T>(l, 0) * P(l, m, p.theta().cos())
    } else if m > 0 {
        K::<T>(l, m) * P(l, m, p.theta().cos())
    } else {
        K::<T>(l, -m) * P(l, -m, p.theta().cos())
    };
    Complex::new(
        v * (T::from_i64(m).unwrap() * p.phi()).sin(),
        v * (T::from_i64(m).unwrap() * p.phi()).cos(),
    )
}

#[allow(non_snake_case)]
#[inline]
pub fn RealSH<T: Float + FloatConst + FromPrimitive>(l: i64, m: i64, p: &impl Coordinates<T>) -> T {
    if m == 0 {
        K::<T>(l, 0) * P(l, m, p.theta().cos())
    } else if m > 0 {
        T::SQRT_2()
            * K::<T>(l, m)
            * (T::from_i64(m).unwrap() * p.phi()).cos()
            * P(l, m, p.theta().cos())
    } else {
        T::SQRT_2()
            * K::<T>(l, -m)
            * (T::from_i64(-m).unwrap() * p.phi()).sin()
            * P(l, -m, p.theta().cos())
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn RegularSolidSH<T: Float + FloatConst + FromPrimitive>(
    l: i64,
    m: i64,
    p: &impl Coordinates<T>,
) -> Complex<T> {
    let scaling = ((T::from_f64(4.0).unwrap() * T::PI()) / T::from_i64(2 * l + 1).unwrap()).sqrt()
        * p.r().powi(l as i32);
    let sh = SH(l, m, p);
    Complex::new(sh.re * scaling, sh.im * scaling)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn it_works() {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        let v = sh10(&p);
        // println!("p: {:?} | v: {}", p, v);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn comp() {
        let p = GenCoordinates::spherical(1.0, PI / 2.0, 0.0);
        // let p = GenCoordinates::cartesian(1.0, 1.0, 0.3);
        assert!((RealSH(2, 1, &p) - sh2p1(&p)) < std::f64::EPSILON);
        assert!((RealSH(3, -2, &p) - sh3n2(&p)) < std::f64::EPSILON);
    }
}