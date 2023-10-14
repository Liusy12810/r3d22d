//! oh
//!

use std::{
    io::{BufReader, Read},
    path::Path,
};

mod spot;

use image::{Pixel, Rgba, RgbaImage};
use spot::Spot;

use crate::linear::HomMtrx3x2;

use self::spot::Spot2d;

struct Tmp(f64, [i32; 2], f64);

#[derive(Debug, Clone)]
pub struct Camera {
    points: Vec<spot::Spot>,
    output: Box<str>,
    /// the first position for width
    /// the second position for height
    screen: [i32; 2],
    height: f64,
    dist: f64,
    // bg: [u8; 4];
}

impl Camera {
    pub fn from_file(file: &str, output: & str) -> Camera {
        println!("{}, {}", file, output);
        let fp = std::fs::File::open(file).expect("ERROR: file not exist!");
        let mut reader = BufReader::new(fp);
        let mut buf = String::new();
        reader
            .read_to_string(&mut buf)
            .expect("ERROR: can't read to string ");
        let mut iter = buf.lines();
        let Tmp(height, screen, dist) = iter.next().expect("ERROR: Can't read line camera").into();
        let points: Vec<Spot> = iter.map(|s| Spot::new(s)).collect();
        let output = output.into();
        Camera {
            points,
            screen,
            height,
            dist,
            output,
        }
    }

    pub fn render(&self) -> Result<RgbaImage, Box<str>> {
        let [width, height] = self.screen;
        let transform = HomMtrx3x2::from([
            [1.0 / self.height, 0.0, 0.0, 0.0],
            [0.0, 1.0 / self.height, 0.0, 0.0],
            [0.0, 0.0, 1.0 / self.dist, 0.0],
        ]);
        let mut points_2d: Vec<Spot2d> = self
            .points
            .iter()
            .map(|s| {
                let pos = &transform * s.pos;
                let col = s.col;
                let rad = s.rad;
                let bri = s.bri;
                // println!("{} => {}", s.pos, &transform * s.pos);
                Spot2d { pos, col, rad, bri }
            })
            .collect();

        points_2d.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let mut image: RgbaImage =
            image::ImageBuffer::from_fn(width as u32, height as u32, |_, _| Rgba([4, 1, 20, 255]));

        for point in points_2d.iter() {
            let Spot2d { pos, rad, col, bri } = point;
            if pos.w < self.dist {
                continue;
            }
            let x = (height as f64 * pos.x / pos.w) as i32 + width / 2;
            let y = (height as f64 * pos.y / pos.w) as i32 + height / 2;
            let r = (height as f64 * rad / pos.w / self.height) as i32;

            for i in (x - r - 1).max(0)..(x + r + 1).min(width) {
                for j in (y - r - 1).max(0)..(y + r + 1).min(height) {
                    let sqr = ((x - i) * (x - i) + (y - j) * (y - j)) as f64;
                    if (x - i) * (x - i) + (y - j) * (y - j) > r * r {
                        continue;
                    };
                    let rat = (r as f64 - sqr.sqrt()) / r as f64;
                    let mut col = col.times(1.0 + bri * rat) * rat;
                    col = col * (1.0 - pos.w * 0.002).max(0.3);
                    image.get_pixel_mut(i as u32, j as u32).blend(&col.into());
                }
            }
        };
        Ok(image.into())
    }

    pub fn run(&self) -> Option<&'static str> {
        let image = match self.render() {
            Ok(image) => image,
            Err(err) => panic!("got this error: {}", err),
        };

        let out_dir = Path::new("images");
        if !out_dir.exists() {
            match std::fs::create_dir(out_dir) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Oops! {}", e);
                    return None;
                }
            }
        }

        match image.save_with_format(out_dir.join(&*self.output), image::ImageFormat::Png) {
            Ok(_) => Some("Yeah!, The rendering finished!"),
            Err(e) => {
                eprintln!("Oops! {}", e);
                None
            }
        }
    }
}

impl From<&str> for Tmp {
    fn from(value: &str) -> Self {
        let mut iter = value.split(" ");
        let height: f64 = iter
            .next()
            .expect("no width specificed")
            .parse()
            .expect("can not parse width");
        let h_pixels: i32 = iter
            .next()
            .expect("no resolution specificed")
            .parse()
            .expect("can not parse resolution");
        let v_pixels: i32 = iter
            .next()
            .expect("no resolution specificed")
            .parse()
            .expect("can not parse resolution");
        let dist: f64 = iter
            .next()
            .expect("no screen distance specificed")
            .parse()
            .expect("can not parse screen distance");
        Tmp(height, [h_pixels, v_pixels], dist)
    }
}
