//! homvec.rs --
//!
// #![allow(unused)]

use std::{
    fmt::Display,
    ops::{Add, Mul},
};

pub const HomI3: HomMtrx3x3 = HomMtrx3x3 {
    c1: HomVec3d { x: 1.0, y: 0.0, z: 0.0, w: 0.0 },
    c2: HomVec3d { x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
    c3: HomVec3d { x: 0.0, y: 0.0, z: 1.0, w: 0.0 },
    cw: HomVec3d { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
};

pub const HomI2: HomMtrx2x2 = HomMtrx2x2 {
    c1: HomVec2d { x: 1.0, y: 0.0, w: 0.0 },
    c2: HomVec2d { x: 0.0, y: 1.0, w: 0.0 },
    cw: HomVec2d { x: 0.0, y: 0.0, w: 1.0 },
};

#[derive(Debug, Default, Clone, Copy)]

pub struct HomVec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct HomVec2d {
    pub x: f64,
    pub y: f64,
    pub w: f64,
}

impl HomVec3d {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        HomVec3d { x, y, z, w }
    }
    #[inline]
    pub fn vec(x: f64, y: f64, z: f64) -> Self {
        HomVec3d { x, y, z, w: 0.0 }
    }
    #[inline]
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        HomVec3d { x, y, z, w: 1.0 }
    }
}

impl HomVec2d {
    #[inline]
    pub fn new(x: f64, y: f64, w: f64) -> Self {
        HomVec2d { x, y, w }
    }

    #[inline]
    pub fn vec(x: f64, y: f64) -> Self {
        HomVec2d { x, y, w: 0.0 }
    }

    #[inline]
    pub fn point(x: f64, y: f64) -> Self {
        HomVec2d { x, y, w: 1.0 }
    }
}

impl From<&[f64]> for HomVec3d {
    #[inline]
    fn from(a: &[f64]) -> Self {
        assert_eq!(a.len(), 4);
        HomVec3d::new(a[0], a[1], a[2], a[3])
    }
}

impl From<[f64; 4]> for HomVec3d {
    #[inline]
    fn from(a: [f64; 4]) -> Self {
        HomVec3d::new(a[0], a[1], a[2], a[3])
    }
}

impl From<(f64, f64, f64, f64)> for HomVec3d {
    #[inline]
    fn from(a: (f64, f64, f64, f64)) -> Self {
        HomVec3d::new(a.0, a.1, a.2, a.3)
    }
}

impl Add for &HomVec3d {
    type Output = HomVec3d;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        HomVec3d::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Add for HomVec3d {
    type Output = HomVec3d;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Mul<HomVec3d> for f64 {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: HomVec3d) -> Self::Output {
        HomVec3d::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl Mul<&HomVec3d> for f64 {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: &HomVec3d) -> Self::Output {
        HomVec3d::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl Mul<f64> for HomVec3d {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(rhs * self.x, rhs * self.y, rhs * self.z, rhs * self.w)
    }
}

impl Mul<f64> for &HomVec3d {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        HomVec3d::new(rhs * self.x, rhs * self.y, rhs * self.z, rhs * self.w)
    }
}

impl PartialEq for HomVec3d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl Eq for HomVec3d {}

impl Display for HomVec3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:^5.1}, {:^5.1}, {:^5.1}, {:^5.1})",
            self.x, self.y, self.z, self.w
        )
    }
}

impl From<&[f64]> for HomVec2d {
    #[inline]
    fn from(a: &[f64]) -> Self {
        assert_eq!(a.len(), 3);
        HomVec2d::new(a[0], a[1], a[2])
    }
}

impl From<[f64; 3]> for HomVec2d {
    #[inline]
    fn from(a: [f64; 3]) -> Self {
        HomVec2d::new(a[0], a[1], a[2])
    }
}

impl From<(f64, f64, f64)> for HomVec2d {
    #[inline]
    fn from(a: (f64, f64, f64)) -> Self {
        HomVec2d::new(a.0, a.1, a.2)
    }
}

impl Add for HomVec2d {
    type Output = HomVec2d;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        HomVec2d::new(self.x + rhs.x, self.y + rhs.y, self.w + rhs.w)
    }
}

impl Add for &HomVec2d {
    type Output = HomVec2d;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        HomVec2d::new(self.x + rhs.x, self.y + rhs.y, self.w + rhs.w)
    }
}

impl Mul<HomVec2d> for f64 {
    type Output = HomVec2d;

    #[inline]
    fn mul(self, rhs: HomVec2d) -> Self::Output {
        HomVec2d::new(self * rhs.x, self * rhs.y, self * rhs.w)
    }
}

impl Mul<&HomVec2d> for f64 {
    type Output = HomVec2d;

    #[inline]
    fn mul(self, rhs: &HomVec2d) -> Self::Output {
        HomVec2d::new(self * rhs.x, self * rhs.y, self * rhs.w)
    }
}

impl Mul<f64> for HomVec2d {
    type Output = HomVec2d;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(rhs * self.x, rhs * self.y, rhs * self.w)
    }
}

impl Mul<f64> for &HomVec2d {
    type Output = HomVec2d;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        HomVec2d::new(rhs * self.x, rhs * self.y, rhs * self.w)
    }
}

impl PartialEq for HomVec2d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.w == other.w
    }
}

impl Eq for HomVec2d {}

impl Display for HomVec2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:^5.1}, {:^5.1}, {:^5.1})", self.x, self.y, self.w)
    }
}

/// This is the part where we define Homogeneous Matrix
#[derive(Debug, Default, Clone)]
pub struct HomMtrx3x3 {
    pub c1: HomVec3d,
    pub c2: HomVec3d,
    pub c3: HomVec3d,
    pub cw: HomVec3d,
}

#[derive(Debug, Default, Clone)]

pub struct HomMtrx2x3 {
    pub c1: HomVec3d,
    pub c2: HomVec3d,
    pub cw: HomVec3d,
}

/// # 2x3 matrix
/// matrices of 2 row and 3 column.
#[derive(Debug, Default, Clone)]

pub struct HomMtrx3x2 {
    pub c1: HomVec2d,
    pub c2: HomVec2d,
    pub c3: HomVec2d,
    pub cw: HomVec2d,
}

#[derive(Debug, Default, Clone)]

pub struct HomMtrx2x2 {
    pub c1: HomVec2d,
    pub c2: HomVec2d,
    pub cw: HomVec2d,
}

impl HomMtrx3x3 {
    pub fn new(c1: HomVec3d, c2: HomVec3d, c3: HomVec3d, cw: HomVec3d) -> Self {
        HomMtrx3x3 { c1, c2, c3, cw }
    }

    pub fn det(&self) -> f64 {
        let Self {
            c1: HomVec3d { x: x1, y: y1, z: z1, .. },
            c2: HomVec3d { x: x2, y: y2, z: z2, .. },
            c3: HomVec3d { x: x3, y: y3, z: z3, .. },
            ..
        } = self;
        x1 * (y2 * z3 - y3 * z2) - x2 * (y1 * z3 - y3 * z1) + x3 * (y1 * z2 - y2 * z1)
    }

    pub fn det_full(&self) -> f64 {
        let Self {
            c1: HomVec3d { x: x1, y: y1, z: z1, w: w1 },
            c2: HomVec3d { x: x2, y: y2, z: z2, w: w2 },
            c3: HomVec3d { x: x3, y: y3, z: z3, w: w3 },
            cw: HomVec3d { x: x4, y: y4, z: z4, w: w4 },
        } = self;
        let detz3w4 = z3 * w4 - z4 * w3;
        let detz2w4 = z2 * w4 - z4 * w2;
        let detz1w3 = z1 * w3 - z3 * w1;
        let detz2w3 = z2 * w3 - z3 * w2;
        let detz1w4 = z1 * w4 - z4 * w1;
        let detz1w2 = z1 * w2 - z2 * w1;
        0.0 + x1 * (y2 * detz3w4 - y3 * detz2w4 + y4 * detz2w3)
            - x2 * (y1 * detz3w4 - y3 * detz1w4 + y4 * detz1w3)
            + x3 * (y1 * detz2w4 - y2 * detz1w4 + y4 * detz1w2)
            - x4 * (y1 * detz2w3 - y2 * detz1w3 + y3 * detz1w2)
    }
}

impl HomMtrx2x3 {
    #[inline]
    fn new(c1: HomVec3d, c2: HomVec3d, cw: HomVec3d) -> Self {
        HomMtrx2x3 { c1, c2, cw }
    }
}

impl HomMtrx3x2 {
    pub fn new(c1: HomVec2d, c2: HomVec2d, c3: HomVec2d, cw: HomVec2d) -> Self {
        HomMtrx3x2 { c1, c2, c3, cw }
    }
}

impl HomMtrx2x2 {
    fn new(c1: HomVec2d, c2: HomVec2d, cw: HomVec2d) -> Self {
        HomMtrx2x2 { c1, c2, cw }
    }

    pub fn det(&self) -> f64 {
        let Self {
            c1: HomVec2d { x: x1, y: y1, .. },
            c2: HomVec2d { x: x2, y: y2, .. },
            ..
        } = self;
        x1 * y2 - x2 * y1
    }

    pub fn det_full(&self) -> f64 {
        let Self {
            c1: HomVec2d { x: x1, y: y1, w: w1 },
            c2: HomVec2d { x: x2, y: y2, w: w2 },
            cw: HomVec2d { x: x3, y: y3, w: w3 },
        } = self;
        x1 * (y2 * w3 - y3 * w2) - x2 * (y1 * w3 - y3 * w1) + x3 * (y1 * w2 - y2 * w1)
    }
}

impl From<[HomVec3d; 4]> for HomMtrx3x3 {
    #[inline]
    fn from(a: [HomVec3d; 4]) -> Self {
        HomMtrx3x3::new(a[0], a[1], a[2], a[3])
    }
}

impl From<[[f64; 4]; 4]> for HomMtrx3x3 {
    #[inline]
    fn from(a: [[f64; 4]; 4]) -> Self {
        HomMtrx3x3::new(
            HomVec3d::new(a[0][0], a[1][0], a[2][0], a[3][0]),
            HomVec3d::new(a[0][1], a[1][1], a[2][1], a[3][1]),
            HomVec3d::new(a[0][2], a[1][2], a[2][2], a[3][2]),
            HomVec3d::new(a[0][3], a[1][3], a[2][3], a[3][3]),
        )
    }
}

impl Mul<HomVec3d> for HomMtrx3x3 {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: HomVec3d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.z * self.c3 + rhs.w * self.cw
    }
}

impl Mul<&HomVec3d> for HomMtrx3x3 {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: &HomVec3d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.z * self.c3 + rhs.w * self.cw
    }
}

impl Mul<HomVec3d> for &HomMtrx3x3 {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: HomVec3d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.z * self.c3 + rhs.w * self.cw
    }
}

impl Mul<&HomVec3d> for &HomMtrx3x3 {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: &HomVec3d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.z * self.c3 + rhs.w * self.cw
    }
}

impl Mul<&HomMtrx3x3> for HomMtrx3x3 {
    type Output = HomMtrx3x3;

    #[inline]
    fn mul(self, rhs: &HomMtrx3x3) -> Self::Output {
        HomMtrx3x3::new(
            &self * &rhs.c1,
            &self * &rhs.c2,
            &self * &rhs.c3,
            &self * &rhs.cw,
        )
    }
}

impl Mul for &HomMtrx3x3 {
    type Output = HomMtrx3x3;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        HomMtrx3x3::new(
            self * &rhs.c1,
            self * &rhs.c2,
            self * &rhs.c3,
            self * &rhs.cw,
        )
    }
}

impl Display for HomMtrx3x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        writeln!(
            f,
            "┌{:^5.1} {:^5.1} {:^5.1} {:^5.1}┐",
            self.c1.x, self.c2.x, self.c3.x, self.cw.x
        )?;
        writeln!(
            f,
            "│{:^5.1} {:^5.1} {:^5.1} {:^5.1}│",
            self.c1.y, self.c2.y, self.c3.y, self.cw.y
        )?;
        writeln!(
            f,
            "│{:^5.1} {:^5.1} {:^5.1} {:^5.1}│",
            self.c1.z, self.c2.z, self.c3.z, self.cw.z
        )?;
        writeln!(
            f,
            "└{:^5.1} {:^5.1} {:^5.1} {:^5.1}┘",
            self.c1.w, self.c2.w, self.c3.w, self.cw.w
        )
    }
}

impl From<[HomVec3d; 3]> for HomMtrx2x3 {
    #[inline]
    fn from(a: [HomVec3d; 3]) -> Self {
        HomMtrx2x3::new(a[0], a[1], a[2])
    }
}

impl From<[[f64; 3]; 4]> for HomMtrx2x3 {
    #[inline]
    fn from(a: [[f64; 3]; 4]) -> Self {
        HomMtrx2x3::new(
            HomVec3d::new(a[0][0], a[1][0], a[2][0], a[3][0]),
            HomVec3d::new(a[0][1], a[1][1], a[2][1], a[3][1]),
            HomVec3d::new(a[0][2], a[1][2], a[2][2], a[3][2]),
        )
    }
}

impl Mul<HomVec2d> for HomMtrx2x3 {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: HomVec2d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.w * self.cw
    }
}

impl Mul<&HomVec2d> for HomMtrx2x3 {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: &HomVec2d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.w * self.cw
    }
}

impl Mul<HomVec2d> for &HomMtrx2x3 {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: HomVec2d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.w * self.cw
    }
}

impl Mul<&HomVec2d> for &HomMtrx2x3 {
    type Output = HomVec3d;

    #[inline]
    fn mul(self, rhs: &HomVec2d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.w * self.cw
    }
}

impl Mul<&HomMtrx2x3> for HomMtrx3x3 {
    type Output = HomMtrx2x3;

    #[inline]
    fn mul(self, rhs: &HomMtrx2x3) -> Self::Output {
        HomMtrx2x3::new(&self * &rhs.c1, &self * &rhs.c2, &self * &rhs.cw)
    }
}

impl Mul<&HomMtrx2x3> for &HomMtrx3x3 {
    type Output = HomMtrx2x3;

    #[inline]
    fn mul(self, rhs: &HomMtrx2x3) -> Self::Output {
        HomMtrx2x3::new(self * &rhs.c1, self * &rhs.c2, self * &rhs.cw)
    }
}

impl Display for HomMtrx2x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        writeln!(
            f,
            "┌{:^5.1} {:^5.1} {:^5.1}┐",
            self.c1.x, self.c2.x, self.cw.x
        )?;
        writeln!(
            f,
            "│{:^5.1} {:^5.1} {:^5.1}│",
            self.c1.y, self.c2.y, self.cw.y
        )?;
        writeln!(
            f,
            "│{:^5.1} {:^5.1} {:^5.1}│",
            self.c1.z, self.c2.z, self.cw.z
        )?;
        writeln!(
            f,
            "└{:^5.1} {:^5.1} {:^5.1}┘",
            self.c1.w, self.c2.w, self.cw.w
        )
    }
}

impl From<[HomVec2d; 4]> for HomMtrx3x2 {
    #[inline]
    fn from(a: [HomVec2d; 4]) -> Self {
        HomMtrx3x2::new(a[0], a[1], a[2], a[3])
    }
}

impl From<[[f64; 4]; 3]> for HomMtrx3x2 {
    #[inline]
    fn from(a: [[f64; 4]; 3]) -> Self {
        HomMtrx3x2::new(
            HomVec2d::new(a[0][0], a[1][0], a[2][0]),
            HomVec2d::new(a[0][1], a[1][1], a[2][1]),
            HomVec2d::new(a[0][2], a[1][2], a[2][2]),
            HomVec2d::new(a[0][3], a[1][3], a[2][3]),
        )
    }
}

impl Mul<HomVec3d> for HomMtrx3x2 {
    type Output = HomVec2d;

    fn mul(self, rhs: HomVec3d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.z * self.c3 + rhs.w * self.cw
    }
}

impl Mul<&HomVec3d> for HomMtrx3x2 {
    type Output = HomVec2d;

    fn mul(self, rhs: &HomVec3d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.z * self.c3 + rhs.w * self.cw
    }
}

impl Mul<HomVec3d> for &HomMtrx3x2 {
    type Output = HomVec2d;

    fn mul(self, rhs: HomVec3d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.z * self.c3 + rhs.w * self.cw
    }
}

impl Mul<&HomVec3d> for &HomMtrx3x2 {
    type Output = HomVec2d;

    fn mul(self, rhs: &HomVec3d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.z * self.c3 + rhs.w * self.cw
    }
}

impl Mul<&HomMtrx2x3> for HomMtrx3x2 {
    type Output = HomMtrx2x2;

    fn mul(self, rhs: &HomMtrx2x3) -> Self::Output {
        HomMtrx2x2::new(&self * &rhs.c1, &self * &rhs.c2, &self * &rhs.cw)
    }
}

impl Mul<&HomMtrx2x3> for &HomMtrx3x2 {
    type Output = HomMtrx2x2;

    fn mul(self, rhs: &HomMtrx2x3) -> Self::Output {
        HomMtrx2x2::new(self * &rhs.c1, self * &rhs.c2, self * &rhs.cw)
    }
}

impl Mul<&HomMtrx3x2> for HomMtrx2x3 {
    type Output = HomMtrx3x3;

    fn mul(self, rhs: &HomMtrx3x2) -> Self::Output {
        HomMtrx3x3::new(
            &self * &rhs.c1,
            &self * &rhs.c2,
            &self * &rhs.c3,
            &self * &rhs.cw,
        )
    }
}

impl Mul<&HomMtrx3x2> for &HomMtrx2x3 {
    type Output = HomMtrx3x3;

    fn mul(self, rhs: &HomMtrx3x2) -> Self::Output {
        HomMtrx3x3::new(
            self * &rhs.c1,
            self * &rhs.c2,
            self * &rhs.c3,
            self * &rhs.cw,
        )
    }
}

impl Mul<&HomMtrx3x3> for HomMtrx3x2 {
    type Output = HomMtrx3x2;

    fn mul(self, rhs: &HomMtrx3x3) -> Self::Output {
        HomMtrx3x2::new(
            &self * &rhs.c1,
            &self * &rhs.c2,
            &self * &rhs.c3,
            &self * &rhs.cw,
        )
    }
}

impl Mul<&HomMtrx3x3> for &HomMtrx3x2 {
    type Output = HomMtrx3x2;

    fn mul(self, rhs: &HomMtrx3x3) -> Self::Output {
        HomMtrx3x2::new(
            self * &rhs.c1,
            self * &rhs.c2,
            self * &rhs.c3,
            self * &rhs.cw,
        )
    }
}

impl Display for HomMtrx3x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        writeln!(
            f,
            "┌{:^5.1} {:^5.1} {:^5.1} {:^5.1}┐",
            self.c1.x, self.c2.x, self.c3.x, self.cw.x
        )?;
        writeln!(
            f,
            "│{:^5.1} {:^5.1} {:^5.1} {:^5.1}│",
            self.c1.y, self.c2.y, self.c3.y, self.cw.y
        )?;
        writeln!(
            f,
            "└{:^5.1} {:^5.1} {:^5.1} {:^5.1}┘",
            self.c1.w, self.c2.w, self.c3.w, self.cw.w
        )
    }
}

impl From<[HomVec2d; 3]> for HomMtrx2x2 {
    #[inline]
    fn from(a: [HomVec2d; 3]) -> Self {
        HomMtrx2x2::new(a[0], a[1], a[2])
    }
}

impl From<[[f64; 3]; 3]> for HomMtrx2x2 {
    #[inline]
    fn from(a: [[f64; 3]; 3]) -> Self {
        HomMtrx2x2::new(
            HomVec2d::new(a[0][0], a[0][0], a[0][0]),
            HomVec2d::new(a[0][1], a[0][1], a[0][1]),
            HomVec2d::new(a[0][2], a[0][2], a[0][2]),
        )
    }
}

impl Add<Self> for HomMtrx2x2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(&self.c1 + &rhs.c1, &self.c2 + &rhs.c2, &self.cw + &rhs.cw)
    }
}

impl Mul<HomVec2d> for HomMtrx2x2 {
    type Output = HomVec2d;

    fn mul(self, rhs: HomVec2d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.w * self.cw
    }
}

impl Mul<&HomVec2d> for HomMtrx2x2 {
    type Output = HomVec2d;

    fn mul(self, rhs: &HomVec2d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.w * self.cw
    }
}

impl Mul<HomVec2d> for &HomMtrx2x2 {
    type Output = HomVec2d;

    fn mul(self, rhs: HomVec2d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.w * self.cw
    }
}

impl Mul<&HomVec2d> for &HomMtrx2x2 {
    type Output = HomVec2d;

    fn mul(self, rhs: &HomVec2d) -> Self::Output {
        rhs.x * self.c1 + rhs.y * self.c2 + rhs.w * self.cw
    }
}

impl Mul<&HomMtrx2x2> for HomMtrx2x2 {
    type Output = Self;

    fn mul(self, rhs: &HomMtrx2x2) -> Self::Output {
        HomMtrx2x2::new(&self * &rhs.c1, &self * &rhs.c2, &self * &rhs.cw)
    }
}

impl Mul<&HomMtrx2x2> for &HomMtrx2x2 {
    type Output = HomMtrx2x2;

    fn mul(self, rhs: &HomMtrx2x2) -> Self::Output {
        HomMtrx2x2::new(self * &rhs.c1, self * &rhs.c2, self * &rhs.cw)
    }
}

impl Mul<&HomMtrx3x2> for HomMtrx2x2 {
    type Output = Self;

    fn mul(self, rhs: &HomMtrx3x2) -> Self::Output {
        HomMtrx2x2::new(&self * &rhs.c1, &self * &rhs.c2, &self * &rhs.cw)
    }
}

impl Mul<&HomMtrx3x2> for &HomMtrx2x2 {
    type Output = HomMtrx2x2;

    fn mul(self, rhs: &HomMtrx3x2) -> Self::Output {
        HomMtrx2x2::new(self * &rhs.c1, self * &rhs.c2, self * &rhs.cw)
    }
}

impl Mul<&HomMtrx2x2> for HomMtrx2x3 {
    type Output = Self;

    fn mul(self, rhs: &HomMtrx2x2) -> Self::Output {
        HomMtrx2x3::new(&self * &rhs.c1, &self * &rhs.c2, &self * &rhs.cw)
    }
}

impl Mul<&HomMtrx2x2> for &HomMtrx2x3 {
    type Output = HomMtrx2x3;

    fn mul(self, rhs: &HomMtrx2x2) -> Self::Output {
        HomMtrx2x3::new(self * &rhs.c1, self * &rhs.c2, self * &rhs.cw)
    }
}
