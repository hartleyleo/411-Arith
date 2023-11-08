
use csc411_image::{RgbImage, Rgb};
use array2::Array2;

// Documenatation:
// RgbImage: https://docs.rs/csc411_image/latest/csc411_image/struct.RgbImage.html

#[derive(Clone, Debug)]
pub struct RGBFloat {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Debug)]
pub struct Ypbpr {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

// Function that reads in an RgbImage, trims off either the last row and/or column
// to make the image evenly dimensioned, then loads the information into an Array2
// of rgb values 
pub fn prepare_ppm(image: &RgbImage) -> Vec<csc411_image::Rgb> {

    let mut width = image.width;
    let mut height = image.height;

    // Trim edges if width or height are uneven
    if image.width % 2 != 0 {
        width -= 1;
    }

    if image.height % 2 != 0 {
        height -= 1;
    }

    let mut pixel_data: Vec<Rgb> = vec![Rgb{red: 0, green: 0, blue: 0}; (width * height) as usize];

    // Looks through each pixel (rgb value) in the image, and pushes each pixel into 
    // a new vec for later manipulation
    for i in 0..height {
        for j in 0..width {
            pixel_data.push(image.pixels[(image.width as usize * i as usize) + j as usize].clone());
        }
    }

    return pixel_data;
}

pub fn convert_to_rgb_float(rbg_vec: &Vec<csc411_image::Rgb>, denominator: u16) -> Vec<RGBFloat> {

    let mut rgb_float_image: Vec<RGBFloat> = vec![RGBFloat { r: 0.0, g: 0.0, b: 0.0 }; rbg_vec.len()];

    // For each pixel in the vec, push a copy of it to this new float image.
    // To transform to a floating-point representation of RGB in respect to the 
    // ppm format and its denominator, we divide each pixel's rgb values by
    // said denominator and create a new floating point 'rgb' struct value
    rgb_float_image = rbg_vec.iter()
        .map(|el| RGBFloat {
            r: el.red as f32 / denominator as f32,
            g: el.green as f32 / denominator as f32,
            b: el.blue as f32 / denominator as f32,
        })
        .collect();

    // working for loop implementation:
    // for el in rbg_vec.iter() {
    //     rgb_float_image.push(RGBFloat { r: el.red as f32 / denominator as f32, g: el.green as f32 / denominator as f32, b: el.blue as f32 / denominator as f32});
    // }

    return rgb_float_image;
}

pub fn convert_to_component_video(rbg_float_vec: &Vec<RGBFloat>) -> Vec<Ypbpr> {

    let mut component_video_per_pixel: Vec<Ypbpr> = vec![Ypbpr { y: 0.0, pb: 0.0, pr: 0.0 }; rbg_float_vec.len()];

    // Based off of the formulas provided in the assignment description:
    // y = 0.299 * r + 0.587 * g + 0.114 * b;
    // pb = -0.168736 * r - 0.331264 * g + 0.5 * b;
    // pr = 0.5 * r - 0.418688 * g - 0.081312 * b;
    // ---------------------------------------------------------------------
    // This block below takes each element in the rgb_float_vec vector and 
    // maps its values to the Ypbpr data struct after running it through the
    // appropriate conversion calculations
    component_video_per_pixel = rbg_float_vec.iter()
        .map(|el| Ypbpr {
            y: (0.299 * el.r) + (0.587 * el.g) + (0.114 * el.b) as f32, 
            pb: (-0.168736 * el.r) - (0.331264 * el.g) + (0.5 * el.b) as f32,
            pr: (0.5 * el.r) - (0.418688 * el.g) - (0.081312 * el.b) as f32,
        })
        .collect();

    return component_video_per_pixel;
}

// pub fn write_to_standard_output() -> todo!() {

// }

// pub fn convert_to_four_bit() -> todo!() {

// }

// pub fn discrete_cosine_transfer() -> todo!() {

// }

// pub fn inverse_discrete_cosine_transfer() -> todo!() {

// }