use clap::{Arg, App};
use image::DynamicImage;
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

    let input = matches.value_of("INPUT").expect("No input file provided.");
    let output = matches.value_of("OUTPUT").expect("No output file specified.");

    let img = image::open(input)?;

    let normal_map = map_normals(&img);
    normal_map.save(output).unwrap();

    Ok(())
}

fn map_normals(img: &DynamicImage) -> DynamicImage {
    img.clone()
}