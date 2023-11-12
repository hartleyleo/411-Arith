use csc411_image;
use csc411_rpegio;
use csc411_image::{Read, RgbImage};
use crate::compress_decompress::{prepare_ppm, convert_rgb_to_rgb_float, convert_rgb_float_to_component_video, pack_as_32_bit, convert_rgb_float_to_rgb, convert_component_video_to_rgb_float, unpack_to_pixel_values};
use crate::transform::{discrete_cosine_transfer, inverse_discrete_cosine_transfer};
use crate::compress_decompress::Ypbpr;
use crate::compress_decompress::PixelBlockValues;
use csc411_rpegio::{output_rpeg_data, input_rpeg_data};
use csc411_image::Write;

pub fn compress(filename: Option<&str>) {

    // Load in image
    let image = RgbImage::read(Some(filename).expect("REASON").as_deref()).unwrap();

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

    // print!("Compression-----------------------------");

    // for i in 0..rgb_image.len() {
    //     print!("rgb_image: index: {}, r: {}, g: {}, b: {}\n", i, rgb_image[i].red, rgb_image[i].green, rgb_image[i].blue);
    // }

    // Translate the rgb vec into a vec with floating points for the rgb values
    let rgb_float_image = convert_rgb_to_rgb_float(&rgb_image, image.denominator);

    // for i in 0..rgb_float_image.len() {
    //     print!("rgb_float_image: index: {}, r: {}, g: {}, b: {}\n", i, rgb_float_image[i].r, rgb_float_image[i].g, rgb_float_image[i].b);
    // }

    // Translate the rgb float image into a vec of custom struct: Ypbpr
    let component_video_image = convert_rgb_float_to_component_video(&rgb_float_image);

    // for i in 0..component_video_image.len() {
    //     print!("component_video_image: index: {}, y: {}, pb: {}, pr: {}\n", i, component_video_image[i].y, component_video_image[i].pb, component_video_image[i].pr);
    // }

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

    // for i in 0..averaged_pixels.len() {
    //     print!("averaged_pixels: index: {} - a: {}, b: {}, c: {}, d: {}, avg_pb: {}, avg_pr: {}\n", i, averaged_pixels[i].a, averaged_pixels[i].b, averaged_pixels[i].c, averaged_pixels[i].d, averaged_pixels[i].avg_pb, averaged_pixels[i].avg_pr);
    // }
    
    // Bitpack
    let final_image = pack_as_32_bit(&averaged_pixels);
    
    // Write to output
    csc411_rpegio::output_rpeg_data(&final_image, width as usize, height as usize).unwrap();
}

pub fn decompress(filename: Option<&str>) {
    
    // print!("Decompression------------------------------");
    // Load in compressed image
    let (word_vec, _width, _height) = csc411_rpegio::input_rpeg_data(filename).unwrap();

    // Unpack compressed image to PixelBlockValues type vec
    let unpacked_pixel_vec = unpack_to_pixel_values(word_vec);

    // Create a vector that is four times the size of the unpacked compressed image
    // Run them through the inverse discrete cosine transfer, and then place them back
    // into the block formatting they were originally placed in. 
    // for instance: 
    // two blocks next to each other will have to be translated into a vector as such
    // ( 0 1 ) ( 2 3 )
    // ( 4 5 ) ( 6 7 )
    let mut decompressed_pixels = Vec::new();
    for i in 0..unpacked_pixel_vec.len() {
        decompressed_pixels.push(inverse_discrete_cosine_transfer(&unpacked_pixel_vec[i]));
    }

    let mut component_video_image = vec![Ypbpr {y: 0.0, pb: 0.0, pr: 0.0}; _width * _height];

    for i in (0.._height).step_by(2) {
        for j in (0.._width).step_by(2) {
            component_video_image[((_width * i) + j) as usize] = decompressed_pixels[i][0].clone();
            component_video_image[((_width * i) + (j+1)) as usize] = decompressed_pixels[i][1].clone();
            component_video_image[((_width * (i+1)) + j) as usize] = decompressed_pixels[i][2].clone();
            component_video_image[((_width * (i+1)) + (j+1)) as usize] = decompressed_pixels[i][3].clone();
        }
    }

    // for i in 0..component_video_image.len() {
    //     print!("component_video_image: index: {}, y: {}, pb: {}, pr: {}\n", i, component_video_image[i].y, component_video_image[i].pb, component_video_image[i].pr);
    // }

    // Translate these component video pixels into an rgb float vector
    let rgb_float_image = convert_component_video_to_rgb_float(&component_video_image);

    // for i in 0..rgb_float_image.len() {
    //     print!("rgb_float_image: index: {}, r: {}, g: {}, b: {}\n", i, rgb_float_image[i].r, rgb_float_image[i].g, rgb_float_image[i].b);
    // }

    // // Translate the rgb float vector into rgb values
    let rgb_image = convert_rgb_float_to_rgb(&rgb_float_image);

    // for i in 0..rgb_image.len() {
    //     print!("rgb_image: index: {}, r: {}, g: {}, b: {}\n", i, rgb_image[i].red, rgb_image[i].green, rgb_image[i].blue);
    // }

    // Create a PPM image from these rgb values
    let image = RgbImage {
        pixels: rgb_image,
        width: _width as u32,
        height: _height as u32,
        denominator: 255 as u16,
    };

    image.write(None).unwrap();

}