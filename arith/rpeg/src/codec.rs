use csc411_image;
use csc411_rpegio;
use csc411_image::{Read, RgbImage};
use crate::compress_decompress::{prepare_ppm, convert_rgb_to_rgb_float, convert_rgb_float_to_component_video};
use crate::transform::{discrete_cosine_transfer};
use crate::compress_decompress::Ypbpr;
use crate::compress_decompress::PixelBlockValues;
// use bitpack::bitpack::{newu, news};
// use csc411_rpegio::{output_rpeg_data, read_in_rpeg_data};
// use csc411_image::Write;

pub fn compress(filename: Option<&str>) {

    // Load in image
    let image = RgbImage::read(Some(filename.unwrap())).unwrap();

    // Trim the image here so that we can reference the new sizings later
    let mut width: u32 = image.width;
    let mut height: u32 = image.height;

    if image.width % 2 != 0 {
        width -= 1;
    }

    if image.height % 2 != 0 {
        height -= 1;
    }

    // Load image into a Vec<Rgb>
    let rgb_image = prepare_ppm(&image, width, height);

    // Translate the rgb vec into a vec with floating points for the rgb values
    let rgb_float_image = convert_rgb_to_rgb_float(&rgb_image, image.denominator);

    // Translate the rgb float image into a vec of custom struct: Ypbpr
    let component_video_image = convert_rgb_float_to_component_video(&rgb_float_image);

    // Collect pixels into squares and feed them into the discrete cosine transfer function
    let mut averaged_pixels: Vec<PixelBlockValues> =  Vec::new();
    let mut pixel_square: Vec<Ypbpr> = Vec::new();

    for i in (0..height).step_by(2) {
        for j in (0..width).step_by(2) {
            pixel_square.push(component_video_image[((width * i) + j) as usize].clone());
            pixel_square.push(component_video_image[((width * i) + (j+1)) as usize].clone());
            pixel_square.push(component_video_image[((width * (i+1)) + j) as usize].clone());
            pixel_square.push(component_video_image[((width * (i+1)) + (j+1)) as usize].clone());

            averaged_pixels.push(discrete_cosine_transfer(pixel_square));
            pixel_square = Vec::new();
        }
    }
    
    // Bitpack

}

pub fn decompress(filename: Option<&str>) {
    
    // Load in compressed image

    // Unpack compressed image to PixelBlockValues type vec

    // Create a vector that is four times the size of the unpacked compressed image
    // Run them through the inverse discrete cosine transfer, and then place them back
    // into the block formatting they were originally placed in. 
    // for instance: 
    // two blocks next to each other will have to be translated into a vector as such
    // ( 0 1 ) ( 2 3 )
    // ( 4 5 ) ( 6 7 )

    // Translate these component video pixels into an rgb float vector

    // Translate the rgb float vector into rgb values

    // Create a PPM image from these rgb values

}