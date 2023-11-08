
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

#[derive(Clone, Debug)]
pub struct PixelBlockValues {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub avg_pb: usize,
    pub avg_pr: usize,
}

// -----------------------------------------------------------------------------------
//                          COMPRESSION FUNCTIONS
// -----------------------------------------------------------------------------------

// Function that reads in an RgbImage, trims off either the last row and/or column
// to make the image evenly dimensioned, then loads the information into an Array2
// of rgb values 
pub fn prepare_ppm(image: &RgbImage, trimmed_width: u32, trimmed_height: u32) -> Vec<csc411_image::Rgb> {

    let mut pixel_data: Vec<Rgb> = vec![Rgb{red: 0, green: 0, blue: 0}; (trimmed_width * trimmed_height) as usize];

    // Looks through each pixel (rgb value) in the image, and pushes each pixel into 
    // a new vec for later manipulation
    for i in 0..trimmed_height {
        for j in 0..trimmed_width {
            pixel_data[(trimmed_width as usize * i as usize) + j as usize] = image.pixels[(image.width as usize * i as usize) + j as usize].clone();
        }
    }

    return pixel_data;
}

pub fn convert_rgb_to_rgb_float(rbg_vec: &Vec<csc411_image::Rgb>, denominator: u16) -> Vec<RGBFloat> {

    // For each pixel in the vec, push a copy of it to this new float image.
    // To transform to a floating-point representation of RGB in respect to the 
    // ppm format and its denominator, we divide each pixel's rgb values by
    // said denominator and create a new floating point 'rgb' struct value
    let rgb_float_image: Vec<RGBFloat> = rbg_vec.iter()
        .map(|el| RGBFloat {
            r: el.red as f32 / denominator as f32,
            g: el.green as f32 / denominator as f32,
            b: el.blue as f32 / denominator as f32,
        })
        .collect();

    return rgb_float_image;
}

pub fn convert_rgb_float_to_component_video(rbg_float_vec: &Vec<RGBFloat>) -> Vec<Ypbpr> {

    // Based off of the formulas provided in the assignment description:
    // y = 0.299 * r + 0.587 * g + 0.114 * b;
    // pb = -0.168736 * r - 0.331264 * g + 0.5 * b;
    // pr = 0.5 * r - 0.418688 * g - 0.081312 * b;
    // ---------------------------------------------------------------------
    // This block below takes each element in the rgb_float_vec vector and 
    // maps its values to the Ypbpr data struct after running it through the
    // appropriate conversion calculations
    let component_video_per_pixel: Vec<Ypbpr> = rbg_float_vec.iter()
        .map(|el| Ypbpr {
            y: (0.299 * el.r) + (0.587 * el.g) + (0.114 * el.b) as f32, 
            pb: (-0.168736 * el.r) - (0.331264 * el.g) + (0.5 * el.b) as f32,
            pr: (0.5 * el.r) - (0.418688 * el.g) - (0.081312 * el.b) as f32,
        })
        .collect();

    return component_video_per_pixel;
}

// -----------------------------------------------------------------------------------
//                         DECOMPRESSION FUNCTIONS
// -----------------------------------------------------------------------------------

pub fn convert_component_video_to_rgb_float(component_video_pixel_vec: &Vec<Ypbpr>) -> Vec<RGBFloat> {

    // Based off of the formulas provided in the assignment description:
    // r = 1.0 * y + 0.0 * pb + 1.402 * pr;
    // g = 1.0 * y - 0.344136 * pb - 0.714136 * pr;
    // b = 1.0 * y + 1.772 * pb + 0.0 * pr;
    // ---------------------------------------------------------------------
    let rgb_float_vec: Vec<RGBFloat> = component_video_pixel_vec.iter()
        .map(|el| RGBFloat {
            r: 1.0 * el.y + 0.0 * el.pb + 1.402 * el.pr as f32, 
            g: 1.0 * el.y - 0.344136 * el.pb - 0.714136 * el.pr as f32,
            b: 1.0 * el.y + 1.772 * el.pb + 0.0 * el.pr as f32,
        })
        .collect();

    return rgb_float_vec;
}

pub fn convert_rgb_float_to_rgb(rbg_float_vec: &Vec<RGBFloat>) -> Vec<csc411_image::Rgb>{

    let pixel_data: Vec<csc411_image::Rgb> = rbg_float_vec.iter()
        .map(|el| csc411_image::Rgb {
            red: el.r as u16, 
            green: el.g as u16,
            blue: el.b as u16,
        })
        .collect();

    return pixel_data;
}

pub fn convert_rgb_to_rgb_image(rgb_vec: &Vec<Rgb>, discovered_width: u32, discovered_height: u32, discovered_denominator: u16) -> RgbImage {

    return RgbImage {
        pixels: rgb_vec.to_vec(),
        width: discovered_width,
        height: discovered_height,
        denominator: discovered_denominator,
    };

}

// pub fn write_to_standard_output() -> todo!() {

// }

// pub fn convert_to_four_bit() -> todo!() {

// }
