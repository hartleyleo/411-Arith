use csc411_image;
use csc411_rpegio;
// use csc411_image::Write;
use csc411_image::{Read, RgbImage};
// use bitpack::bitpack::{newu, news};
// use csc411_rpegio::{output_rpeg_data, read_in_rpeg_data};
use crate::lib::{prepare_ppm};

pub fn compress(filename: Option<&str>) {
    
    let image = RgbImage::read(Some(filename)).unwrap();

    let rgb_float_image = prepare_ppm(&image);

    for i in 0..rgb_float_image.len() {
        print!("{}", rgb_float_image[i]);
    }

}

pub fn decompress(filename: Option<&str>) {
    todo!();
}
