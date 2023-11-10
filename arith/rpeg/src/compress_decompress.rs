
use csc411_image::{RgbImage, Rgb};
use bitpack::bitpack::{newu, news, getu, gets};

// Documenatation:
// Rgb: https://docs.rs/csc411_image/latest/csc411_image/imgtype/struct.Rgb.html
// RgbImage: https://docs.rs/csc411_image/latest/csc411_image/struct.RgbImage.html
// Vecs: https://doc.rust-lang.org/std/vec/struct.Vec.html

// ------------------------------------------
//           Custom Data Structs
// ------------------------------------------

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

pub fn pack_as_32_bit(compression_vec: &Vec<PixelBlockValues>) -> Vec<0_u64>{
    
    let mut final_image = Vec::new();
    for i in 0..compression_vec.len() {
        let mut word = 0_u64;
        word = newu(word, 9, 23, compression_vec[i].a as u64 ).unwrap();
        word = news(word, 5, 18, compression_vec[i].b as i64 ).unwrap();
        word = news(word, 5, 13, compression_vec[i].c as i64 ).unwrap();
        word = news(word, 5, 8, compression_vec[i].d as i64 ).unwrap();
        word = newu(word, 4, 4, compression_vec[i].avg_pb as u64 ).unwrap();
        word = newu(word, 4, 0, compression_vec[i].avg_pr as u64 ).unwrap();
        final_image.push((word as u32).to_be_bytes());
    }

    return final_image;
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

pub fn unpack_to_pixel_values(decompression_vec: &Vec<0_u64>) -> Vec<PixelBlockValues>{
    
    let unpacked_pixel_vec = Vec::new();
    for el in 0..decompression_vec.len() {
        let word = u32::from_be_bytes(el);
        let decompressed_a = getu(word as u64, 9, 23);
        let decompressed_b = gets(word as u64, 5, 18);
        let decompressed_c = gets(word as u64, 5, 13);
        let decompressed_d = gets(word as u64, 5, 8);
        let decompressed_avg_pb = getu(word as u64, 4, 4);
        let decompressed_avg_pr = getu(word as u64, 4, 0);

        unpacked_pixel_vec.push(PixelBlockValues {a: decompressed_a as f32, b: decompressed_b as f32, c: decompressed_c as f32, d: decompressed_d as f32, avg_pb: decompressed_avg_pb as usize, avg_pr: decompressed_avg_pr as usize});
    }

    return unpacked_pixel_vec;
}