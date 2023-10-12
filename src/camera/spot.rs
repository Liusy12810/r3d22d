//! hello
//!
//!

use std::{fmt::Display, ops::Mul};

use image::Rgba;

use crate::linear::{HomVec2d, HomVec3d};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

/// # Color
/// [position: x y z] [color(0.0~1.0): r g b] [rad]
#[derive(Debug, Clone)]
pub struct Spot {
    pub pos: HomVec3d,
    pub col: Color,
    pub rad: f64,
    pub bri: f64,
}

#[derive(Debug, Clone)]
pub struct Spot2d {
    pub pos: HomVec2d,
    pub col: Color,
    pub rad: f64,
    pub bri: f64,
}

impl Color {
    fn new(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self { r, g, b, a }
    }

    pub fn times(&self, val: f64) -> Self {
        assert!(/* val <= 1.0 && */ val >= 0.0);
        let Self { r, g, b, .. } = self;
        Self {
            r: (*r as f64 * val).min(255.0) as u16,
            g: (*g as f64 * val).min(255.0) as u16,
            b: (*b as f64 * val).min(255.0) as u16,
            ..*self
        }
    }
}

impl Spot {
    pub fn new(val: &str) -> Self {
        let mut iter = val.split(" ");
        let x: f64 = iter
            .next()
            .expect("no x coordinate found")
            .parse()
            .expect("invalid value x: can't parse");
        let y: f64 = iter
            .next()
            .expect("no y coordinate found")
            .parse()
            .expect("invalid value y: can't parse");
        let z: f64 = iter
            .next()
            .expect("no z coordinate found")
            .parse()
            .expect("invalid value z: can't parse");
        let pos = HomVec3d::point(x, y, z);

        let r: u16 = iter
            .next()
            .expect("no r coordinate found")
            .parse()
            .expect("invalid value r: can't parse");
        let g: u16 = iter
            .next()
            .expect("no g coordinate found")
            .parse()
            .expect("invalid value g: can't parse");
        let b: u16 = iter
            .next()
            .expect("no b coordinate found")
            .parse()
            .expect("invalid value b: can't parse");
        let col = Color::new(r, g, b, 255);

        let rad: f64 = iter
            .next()
            .expect("no rad coordinate found")
            .parse()
            .expect("invalid value rad: can't parse");
        let bri: f64 = iter
            .next()
            .unwrap_or("0")
            .parse()
            .expect("can't parse brightness");
        Spot { pos, col, rad, bri }
    }
}

impl From<&Color> for Rgba<u8> {
    fn from(value: &Color) -> Self {
        Rgba([value.r as u8, value.g as u8, value.b as u8, value.a as u8])
    }
}

impl From<Color> for Rgba<u8> {
    fn from(value: Color) -> Self {
        Rgba([value.r as u8, value.g as u8, value.b as u8, value.a as u8])
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            a: (self.a as f64 * rhs).min(255.0) as u16,
            ..self
        }
    }
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            a: (self.a as f64 * rhs).min(255.0) as u16,
            ..*self
        }
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color {
            a: (rhs.a as f64 * self).min(255.0) as u16,
            ..*rhs
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            a: (rhs.a as f64 * self).min(255.0) as u16,
            ..rhs
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let r = ((self.r * rhs.r) as f64 / 255.0) as u16;
        let g = ((self.g * rhs.g) as f64 / 255.0) as u16;
        let b = ((self.b * rhs.b) as f64 / 255.0) as u16;
        let a = ((self.a * rhs.a) as f64 / 255.0) as u16;
        Self { r, g, b, a }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Eq for Color {}

impl Display for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for Spot2d {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.col == other.col && self.rad == other.rad
    }
}

impl PartialOrd for Spot2d {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.pos.partial_cmp(&other.pos) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.rad.partial_cmp(&other.rad)
    }
}
