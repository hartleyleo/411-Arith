pub mod codec;

use csc411_image::{RgbImage, Rgb};
use array2::Array2;

// Documenatation:
// RgbImage: https://docs.rs/csc411_image/latest/csc411_image/struct.RgbImage.html

// Function that reads in an RgbImage, trims off either the last row and/or column
// to make the iamge evenly dimensioned, then loads the information into an Array2
// of rgb values 
pub fn prepare_ppm(image: &RgbImage) -> Array2<Vec<csc411_image::Rgb>> {

    let mut width = image.width;
    let mut height = image.height;

    // Trim edges if width or height are uneven
    if image.width % 2 != 0 {
        width -= 1;
    }

    if image.height % 2 != 0 {
        height -= 1;
    }

    // Store into Array2
    let mut pixel_data: Vec<Rgb> = vec![Rgb{red: 0, green: 0, blue: 0}; (width * height) as usize];

    for i in 0..width {
        for j in 0..height {
            pixel_data[(image.width as usize * i as usize) + j as usize] = image.pixels[(image.width as usize * i as usize) + j as usize].clone();
        }
    }

    // Return Array2
    return Array2::new(pixel_data, width as usize, height as usize);

}

// pub fn convert_to_rgb() -> todo!() {

// }

// pub fn convert_to_component_video() -> todo!() {

// }

// pub fn write_to_standard_output() -> todo!() {

// }

// pub fn convert_to_four_bit() -> todo!() {

// }

// pub fn discrete_cosine_transfer() -> todo!() {

// }

// pub fn inverse_discrete_cosine_transfer() -> todo!() {

// }