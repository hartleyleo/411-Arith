use csc411_image;
use csc411_rpegio;
use csc411_image::{Read, RgbImage};
use crate::compress_decompress::{prepare_ppm};
// use bitpack::bitpack::{newu, news};
// use csc411_rpegio::{output_rpeg_data, read_in_rpeg_data};
// use csc411_image::Write;

pub fn compress(filename: Option<&str>) {

    let image = RgbImage::read(Some(filename.unwrap())).unwrap();

    let rgb_float_image = prepare_ppm(&image);

    for i in 0..rgb_float_image.len() {
        print!("{}", i);
    }

    

}

pub fn decompress(filename: Option<&str>) {
    todo!();
}