//! test binary

// use std::error::Error;

use std::rc::Rc;

use r3d22d::{
    camera::Camera,
    linear::{HomMtrx3x3, SqrMtrx},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let defaults:[Rc<str>;2] =  ["input.txt".into(), "output.png".into()];
    let file = args.get(1).map_or(defaults[0].clone(), |s| (&s[..]).into());
    let output = args.get(2).map_or(defaults[1].clone(), |s| (&s[..]).into());
    let camera = Camera::from_file(&file, &output);

    println!("{}", camera.run().unwrap());
}
