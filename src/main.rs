use clap::{Arg, App, crate_version};
use std::error::Error;

use normal_heights::map_normals;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Normal Heights")
        .version(crate_version!())
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
