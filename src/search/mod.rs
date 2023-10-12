//! now let's do something related to search
//!
//!
#![allow(unused)]

use crate::linear::HomVec2d;

pub enum Node {
    LR {
        p: usize,
        sep: f64,
        l: usize,
        r: usize,
    },
    TD {
        p: usize,
        sep: f64,
        t: usize,
        d: usize,
    },
    Val {
        p: usize,
        rt: (f64, f64),
        lb: (f64, f64),
    },
}

pub struct BinSepTree {
    pub buf: Vec<Node>,
}

impl BinSepTree {
    fn new(arr: Vec<HomVec2d>) -> Self {
        let mut buf: Vec<Node> = Vec::with_capacity(2 * arr.len() + 2);
        todo!("建树总是令人头痛")
    }
}
