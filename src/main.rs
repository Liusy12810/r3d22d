//! test binary

// use std::error::Error;

use r3d22d::{
    camera::Camera,
    linear::{EnrichMtrx, HOM_I3, HomMtrx3x3},
};

fn main() {
    #[cfg(debug_assertions)]
    {
        let mtrx = HomMtrx3x3::from([
            [2.0, 2.0, 4.0, 2.0],
            [8.0, 3.0, 5.0, 3.0],
            [0.0, 0.0, 1.0, 0.0],
            [7.0, 0.0, 2.0, 1.0],
        ]);
        let mut enmtrx = EnrichMtrx::new(&mtrx, &HOM_I3);
        enmtrx.g_eliminate();
        println!("{}{}{}", &mtrx, &enmtrx.right, &mtrx * &enmtrx.right);
    }
    let camera = Camera::from_file("file.txt", "img-3.png");

    println!("{}", camera.run().unwrap());
    
}
