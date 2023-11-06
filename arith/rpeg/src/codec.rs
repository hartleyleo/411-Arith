use csc411_image;
use lib.rs::{prepare_ppm};

pub fn compress(filename: Option<&str>) {
    
    let image = RgbImage::read(Some(filename)).unwrap();

    let rgb_float_image = prepare_ppm(&image);
    
}

pub fn decompress(filename: Option<&str>) {
    todo!();
}
