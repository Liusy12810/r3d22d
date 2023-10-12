//! 你不知道我为了搞初等行变换做了多少努力，你不知道！！！

#![allow(unused)]

use std::ops::{DivAssign, Mul, Sub, SubAssign};

use super::*;

struct Line<T, const C: usize>([T; C]);

impl<T, const C: usize> From<[T; C]> for Line<T, C> {
    fn from(value: [T; C]) -> Self {
        Self(value)
    }
}

impl<T, const C: usize> From<[&T; C]> for Line<T, C>
where
    T: Copy + Default,
{
    fn from(value: [&T; C]) -> Self {
        let mut arr: [T; C] = [T::default(); C];
        for i in 0..value.len() {
            arr[i] = *value[i];
        }
        Self(arr)
    }
}

impl<T, const C: usize> From<[&mut T; C]> for Line<T, C>
where
    T: Copy + Default,
{
    fn from(value: [&mut T; C]) -> Self {
        let mut arr: [T; C] = [T::default(); C];
        for i in 0..value.len() {
            arr[i] = *value[i];
        }
        Self(arr)
    }
}

impl<T, U, const C: usize> Mul<U> for &Line<T, C>
where
    T: Copy + Default + Mul<U, Output = T>,
    U: Copy,
{
    type Output = Line<T, C>;

    fn mul(self, rhs: U) -> Self::Output {
        let mut arr: [T; C] = [T::default(); C];
        for i in 0..self.0.len() {
            arr[i] = self.0[i] * rhs;
        }
        Line(arr)
    }
}

impl<T, U, const C: usize> Mul<U> for Line<T, C>
where
    T: Copy + Default + Mul<U, Output = T>,
    U: Copy,
{
    type Output = Self;

    fn mul(self, rhs: U) -> Self::Output {
        let mut arr: [T; C] = [T::default(); C];
        for i in 0..self.0.len() {
            arr[i] = self.0[i] * rhs;
        }
        Self(arr)
    }
}

impl<T, const C: usize> SubAssign<Line<T, C>> for [T; C]
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: Line<T, C>) {
        for i in 0..self.len() {
            self[i] -= rhs.0[i];
        }
    }
}

impl<T, const C: usize> SubAssign<&Line<T, C>> for [T; C]
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: &Line<T, C>) {
        for i in 0..self.len() {
            self[i] -= rhs.0[i];
        }
    }
}

impl<T, const C: usize> SubAssign<Line<T, C>> for [&mut T; C]
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: Line<T, C>) {
        for i in 0..self.len() {
            *self[i] -= rhs.0[i];
        }
    }
}

impl<T, const C: usize> SubAssign<&Line<T, C>> for [&mut T; C]
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: &Line<T, C>) {
        for i in 0..self.len() {
            *self[i] -= rhs.0[i];
        }
    }
}

impl<T, const C: usize> Sub<Line<T, C>> for [T; C]
where
    T: Copy + Default + Sub<T, Output = T>,
{
    type Output = [T; C];

    fn sub(self, rhs: Line<T, C>) -> Self::Output {
        let mut arr = [T::default(); C];
        for i in 0..self.len() {
            arr[i] = self[i] - rhs.0[i];
        }
        arr
    }
}

impl<T, const C: usize> Sub<&Line<T, C>> for [T; C]
where
    T: Copy + Default + Sub<T, Output = T>,
{
    type Output = [T; C];

    fn sub(self, rhs: &Line<T, C>) -> Self::Output {
        let mut arr = [T::default(); C];
        for i in 0..self.len() {
            arr[i] = self[i] - rhs.0[i];
        }
        arr
    }
}

impl<T, const C: usize> Sub<Line<T, C>> for [&mut T; C]
where
    T: Copy + Default + Sub<T, Output = T>,
{
    type Output = [T; C];

    fn sub(self, rhs: Line<T, C>) -> Self::Output {
        let mut arr = [T::default(); C];
        for i in 0..self.len() {
            arr[i] = *self[i] - rhs.0[i];
        }
        arr
    }
}

impl<T, const C: usize> Sub<&Line<T, C>> for [&mut T; C]
where
    T: Copy + Default + Sub<T, Output = T>,
{
    type Output = [T; C];

    fn sub(self, rhs: &Line<T, C>) -> Self::Output {
        let mut arr = [T::default(); C];
        for i in 0..self.len() {
            arr[i] = *self[i] - rhs.0[i];
        }
        arr
    }
}

pub struct EnrichMtrx<T, U, const R: usize, const C1: usize, const C2: usize>
where
    T: Matrix<f64, R, C1>,
    U: Matrix<f64, R, C2>,
{
    pub left: T,
    pub right: U,
}
// Assert<{ T::DIM.0 == U::DIM.0 }>: IsTrue, {}

impl<T, U, const R: usize, const C: usize> EnrichMtrx<T, U, R, R, C>
where
    T: Matrix<f64, R, R> + Clone,
    U: Matrix<f64, R, C> + Clone,
{
    const R: usize = R;
    const C: usize = C;
    pub fn new(left: &T, right: &U) -> Self {
        Self {
            left: left.clone(),
            right: right.clone(),
        }
    }

    pub fn g_eliminate(&mut self) {
        for i in 0..(R - 1) {
            let idx = *self.left.row(i)[i];
            self.left
                .row_mut(i)
                .iter_mut()
                .for_each(|v| v.div_assign(idx));
            self.right
                .row_mut(i)
                .iter_mut()
                .for_each(|v| v.div_assign(idx));
            let rowl: Line<f64, R> = Line::from(self.left.row(i));
            let rowr: Line<f64, C> = Line::from(self.right.row(i));
            for j in i + 1..R {
                let ratio = *self.left.row(j)[i];
                let mut lhs = self.left.row_mut(j);
                lhs -= &rowl * ratio;
                let mut lhs: [&mut f64; C] = self.right.row_mut(j);
                lhs -= &rowr * ratio;
            }
        }
        for i in (1..R).rev() {
            let rowl: Line<f64, R> = Line::from(self.left.row(i));
            let rowr: Line<f64, C> = Line::from(self.right.row(i));
            for j in 0..i {
                let ratio = *self.left.row(j)[i];
                let mut lhs = self.left.row_mut(j);
                lhs -= &rowl * ratio;
                let mut lhs: [&mut f64; C] = self.right.row_mut(j);
                lhs -= &rowr * ratio;
            }
        }
    }
}

pub trait Matrix<T, const R: usize, const C: usize> {
    const C: usize = C;
    const R: usize = R;
    fn row(&self, r: usize) -> [&T; C];
    fn col(&self, c: usize) -> [&T; R];
    fn row_mut(&mut self, r: usize) -> [&mut T; C];
    fn col_mut(&mut self, c: usize) -> [&mut T; R];
}

impl Matrix<f64, 4, 1> for HomVec3d {
    fn row(&self, r: usize) -> [&f64; 1] {
        assert!(r < Self::R);
        match r {
            0 => [&self.x],
            1 => [&self.y],
            2 => [&self.z],
            3 => [&self.w],
            r => panic!("This is impossible!!! {r}"),
        }
    }

    fn col(&self, c: usize) -> [&f64; 4] {
        assert_eq!(c, 1);
        [&self.x, &self.y, &self.z, &self.w]
    }

    fn row_mut(&mut self, r: usize) -> [&mut f64; 1] {
        assert!(r < Self::R);
        match r {
            0 => [&mut self.x],
            1 => [&mut self.y],
            2 => [&mut self.z],
            3 => [&mut self.w],
            r => panic!("This is impossible!!! {r}"),
        }
    }

    fn col_mut(&mut self, c: usize) -> [&mut f64; 4] {
        assert_eq!(c, 1);
        [&mut self.x, &mut self.y, &mut self.z, &mut self.w]
    }
}

impl Matrix<f64, 4, 4> for HomMtrx3x3 {
    fn row(&self, r: usize) -> [&f64; 4] {
        assert!(r < Self::R);
        match r {
            0 => [&self.c1.x, &self.c2.x, &self.c3.x, &self.cw.x],
            1 => [&self.c1.y, &self.c2.y, &self.c3.y, &self.cw.y],
            2 => [&self.c1.z, &self.c2.z, &self.c3.z, &self.cw.z],
            3 => [&self.c1.w, &self.c2.w, &self.c3.w, &self.cw.w],
            r => panic!("impossible!!! {r}"),
        }
    }

    fn col(&self, c: usize) -> [&f64; 4] {
        assert!(c < Self::C);
        match c {
            0 => [&self.c1.x, &self.c1.y, &self.c1.z, &self.c1.w],
            1 => [&self.c2.x, &self.c2.y, &self.c2.z, &self.c2.w],
            2 => [&self.c3.x, &self.c3.y, &self.c3.z, &self.c3.w],
            3 => [&self.cw.x, &self.cw.y, &self.cw.z, &self.cw.w],
            c => panic!("impossible!!! {c}"),
        }
    }

    fn row_mut(&mut self, r: usize) -> [&mut f64; 4] {
        assert!(r < Self::R);
        match r {
            0 => [
                &mut self.c1.x,
                &mut self.c2.x,
                &mut self.c3.x,
                &mut self.cw.x,
            ],
            1 => [
                &mut self.c1.y,
                &mut self.c2.y,
                &mut self.c3.y,
                &mut self.cw.y,
            ],
            2 => [
                &mut self.c1.z,
                &mut self.c2.z,
                &mut self.c3.z,
                &mut self.cw.z,
            ],
            3 => [
                &mut self.c1.w,
                &mut self.c2.w,
                &mut self.c3.w,
                &mut self.cw.w,
            ],
            r => panic!("impossible!!! {r}"),
        }
    }

    fn col_mut(&mut self, c: usize) -> [&mut f64; 4] {
        assert!(c < Self::C);
        match c {
            0 => [
                &mut self.c1.x,
                &mut self.c1.y,
                &mut self.c1.z,
                &mut self.c1.w,
            ],
            1 => [
                &mut self.c2.x,
                &mut self.c2.y,
                &mut self.c2.z,
                &mut self.c2.w,
            ],
            2 => [
                &mut self.c3.x,
                &mut self.c3.y,
                &mut self.c3.z,
                &mut self.c3.w,
            ],
            3 => [
                &mut self.cw.x,
                &mut self.cw.y,
                &mut self.cw.z,
                &mut self.cw.w,
            ],
            c => panic!("impossible!!! {c}"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::linear::HomMtrx3x3;

    use super::{EnrichMtrx, Line};

    #[test]
    fn construct() {
        let mtrx = HomMtrx3x3::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        EnrichMtrx::new(&mtrx, &mtrx);
    }

    #[test]
    fn line_sub_mul() {
        let line = Line([1, 2, 3, 4]);
        let l2 = &line * 2;
        let res = [1, 1, 1, 1] - &line;
        // assert_eq!(res, [-1, -3, -5, -7])
    }
}
