use clap::{Arg, App};
use image::{DynamicImage, RgbImage};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Normal Heights")
        .version("0.1")
        .author("Jon O.")
        .about("Makes normal maps from height maps.")
        .arg(Arg::with_name("INPUT")
            .help("Input height map image file.")
            .required(true))
        .arg(Arg::with_name("OUTPUT")
            .help("Filename to write the normal map to.")
            .required(true))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();

    let img = image::open(input)?;

    let normal_map = map_normals(&img);
    normal_map.save(output).unwrap();

    Ok(())
}

fn map_normals(img: &DynamicImage) -> RgbImage {
    let img = img.clone().into_rgb();
    let mut normal_map = RgbImage::new(img.width(), img.height());
    for (x, y, p) in normal_map.enumerate_pixels_mut() {
        println!("x: {}, y: {}, pixel: {:?}", x, y, p);
        let mut new_p = [0.0, 0.0, 0.0];
        let adj_pixels = get_adjacent_pixels(x,y,&img);
    }
    normal_map
}

/// pixel layout:
/// 6 7 8
/// 3 4 5
/// 0 1 2
/// edge pixels are duplicated when necessary
fn get_adjacent_pixels(x: u32, y: u32, img: &RgbImage) -> Vec<u8> {
    let north = if y <= 0 {
        0
    } else {
        y-1
    };
    let south = if y >= (img.height()-1) {
        img.height()-1
    } else {
        y+1
    };
    let west = if x <= 0 {
        0
    } else {
        x-1
    };
    let east = if x >= (img.height()-1) {
        img.height()-1
    } else {
        x+1
    };

    let mut pixels = Vec::with_capacity(9);

    pixels.push(img.get_pixel(west,south)[0]);
    pixels.push(img.get_pixel(   x,south)[0]);
    pixels.push(img.get_pixel(east,south)[0]);
    pixels.push(img.get_pixel(west,    y)[0]);
    pixels.push(img.get_pixel(   x,    y)[0]);
    pixels.push(img.get_pixel(east,    y)[0]);
    pixels.push(img.get_pixel(west,north)[0]);
    pixels.push(img.get_pixel(   x,north)[0]);
    pixels.push(img.get_pixel(east,north)[0]);

    pixels
}
