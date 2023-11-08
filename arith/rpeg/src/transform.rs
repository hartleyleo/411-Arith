use csc411_image::{RgbImage, Rgb};
use array2::Array2;
use csc411_arith::index_of_chroma;
use csc411_arith::chroma_of_index;
use crate::compress_decompress::Ypbpr;
use crate::compress_decompress::PixelBlockValues;

pub fn discrete_cosine_transfer(pixels: Vec<Ypbpr>) -> PixelBlockValues {
    
    // Use this for simplification of division for averages
    let pixel_total: f32 = pixels.len() as f32;

    // Calculate a, b, c, d
    // -----------------------------------------------------
    // Based on formula provided in assignment description:
    // a = (Y4 + Y3 + Y2 + Y1)/4.0
    // b = (Y4 + Y3 − Y2 − Y1)/4.0
    // c = (Y4 − Y3 + Y2 − Y1)/4.0
    // d = (Y4 − Y3 − Y2 + Y1)/4.0
    // If we imagine these Y's as the pixel in the 2x2 vec's y values, 
    //       ( Y1 Y2 )   as    ( 0.y 1.y )
    //       ( Y3 Y4 )   ->    ( 2.y 3.y )
    // then we can calculate as follows:
    let mut a: f32 = (pixels[0].y + pixels[1].y + pixels[2].y + pixels[3].y) / pixel_total;
    let mut b: f32 = (-pixels[0].y - pixels[1].y + pixels[2].y + pixels[3].y) / pixel_total;
    let mut c: f32 = (-pixels[0].y + pixels[1].y - pixels[2].y + pixels[3].y) / pixel_total;
    let mut d: f32 = (pixels[0].y - pixels[1].y - pixels[2].y + pixels[3].y) / pixel_total;

    // For b, c, d, we clamp it to be between the floating point range of -0.3 and 0.3
    a = (a * (511 as f32)).round();
    b = (b.clamp(-0.3,0.3) * 50.0).round();
    c = (c.clamp(-0.3,0.3) * 50.0).round();
    d = (d.clamp(-0.3,0.3) * 50.0).round();

    // Calculate average pb
    let avg_pb = (pixels[0].pb + pixels[1].pb + pixels[2].pb + pixels[3].pb) / pixel_total;
    let avg_pb = index_of_chroma(avg_pb as f32);

    // Calculate average pr
    let avg_pr = (pixels[0].pr + pixels[1].pr + pixels[2].pr + pixels[3].pr) / pixel_total;
    let avg_pr = index_of_chroma(avg_pr as f32);

    return PixelBlockValues {a, b, c, d, avg_pb, avg_pr};

}

// pub fn inverse_discrete_cosine_transfer() -> todo!() {

// }