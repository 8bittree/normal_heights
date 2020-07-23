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
    img
}
