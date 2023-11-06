use csc411_image::{RgbImage, Rgb};

pub fn prepare_ppm(image: &RgbImage) -> todo!() {

    // Trim edges if need be
    if image.width % 2 != 0 {
        let width = image.width -= 1;
    }

    if image.height % 2 != 0 {
        let height = image.height -= 1;
    }

    // Store into Array2

    // Create new Array2
    let mut temp_vec = Vec<Rgb>;
    // Loop through each index
    // Set each new index to the value of the original images index

    for i in 0..width {
        for j in 0..height {
            temp_vec.push(image.pixels[]);
        }
    }

    // Return Array2
    let prepared_image = Array2::new(val: T, width, height);
    return prepared_image;

}

pub fn convert_to_rgb() -> todo!() {

}

pub fn convert_to_component_video() -> todo!() {

}

pub fn write_to_standard_output() -> todo!() {

}

pub fn convert_to_four_bit() -> todo!() {

}

pub fn discrete_cosine_transfer() -> todo!() {

}

pub fn inverse_discrete_cosine_transfer() -> todo!() {

}