//! test binary

// use std::error::Error;

use r3d22d::{
    camera::Camera,
    linear::{EnrichMtrx, HomMtrx3x3, HomVec3d, HomI3},
};

fn main() {
    let mtrx = HomMtrx3x3::from([
        [2.0, 2.0, 4.0, 2.0],
        [8.0, 3.0, 5.0, 3.0],
        [0.0, 0.0, 1.0, 0.0],
        [7.0, 0.0, 2.0, 1.0],
    ]);
    // let vect = HomVec3d::point(4.0, 6.0, 8.0);
    let mut enmtrx = EnrichMtrx::new(&mtrx, &HomI3);
    enmtrx.g_eliminate();
    println!("{}{}{}", &mtrx, &enmtrx.right, &mtrx * &enmtrx.right);
}

// fn main() -> () {
//     let camera = Camera::from_file("file.txt");

//     match camera.run() {
//         Ok(_) => (),
//         Err(err) => println!("got this error: {}", err),
//     }
// }
