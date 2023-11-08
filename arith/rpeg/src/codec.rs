use csc411_image;
use csc411_rpegio;
use csc411_image::{Read, RgbImage};
use crate::compress_decompress::{prepare_ppm, convert_to_rgb_float, convert_to_component_video};
// use bitpack::bitpack::{newu, news};
// use csc411_rpegio::{output_rpeg_data, read_in_rpeg_data};
// use csc411_image::Write;

pub fn compress(filename: Option<&str>) {

    // Load in image
    let image = RgbImage::read(Some(filename.unwrap())).unwrap();

    // Load image into a Vec<Rgb>
    let rgb_image = prepare_ppm(&image);

    // Translate the rgb vec into a vec with floating points for the rgb values
    let rgb_float_image = convert_to_rgb_float(&rgb_image, image.denominator);

    // Translate the rgb float image into a vec of custom struct: Ypbpr
    let component_video_image = convert_to_component_video(&rgb_float_image);

}

pub fn decompress(filename: Option<&str>) {
    todo!();
}