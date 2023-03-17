use clap:: {
    arg,
    Args,
    Parser,
    Subcommand, 
    Command,
    Arg,
    value_parser,
};

use std:: {
    any::TypeId,
    ffi::OsString,
    path::PathBuf,
};

pub fn cli() -> Command {
    Command::new("imagepro")
        .about("Image Processing CLI Tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("rotate")
                .about("Rotate image by certain amount of degrees")
                .arg(
                    Arg::new("degrees")
                        .help("Amount of degrees to rotate the image")
                        .required(true)
                        .value_parser(value_parser!(f32))
                )
                .arg(
                    Arg::new("source_path")
                        .help("image to be rotated")
                        .required(true)
                        .value_parser(value_parser!(String))
                )
                .arg(
                    Arg::new("dest_path")
                        .help("output image")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .last(true)
                )
        )
        .subcommand(
            Command::new("crop")
                .about("Crop an image")
                .arg(
                    Arg::new("x")
                        .help("x coordinate")
                        .required(true)
                        .value_parser(value_parser!(u32))
                )
                .arg(
                    Arg::new("y")
                        .help("y coordinate")
                        .required(true)
                        .value_parser(value_parser!(u32))
                )
                .arg(
                    Arg::new("width")
                        .help("new width after crop")
                        .required(true)
                        .value_parser(value_parser!(u32))
                )
                .arg(
                    Arg::new("height")
                        .help("new height after crop")
                        .required(true)
                        .value_parser(value_parser!(u32))
                )
                .arg(
                    Arg::new("source_path")
                        .help("input path")
                        .required(true)
                        .value_parser(value_parser!(String))
                )
                .arg(
                    Arg::new("dest_path")
                        .help("output path")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .last(true)
                )
        )
        .subcommand(
            Command::new("resize")
                .about("Resize")
                .arg(
                    Arg::new("width")
                        .help("new width after resize")
                        .required(true)
                        .value_parser(value_parser!(u32))
                )
                .arg(
                    Arg::new("height")
                        .help("new height after resize")
                        .required(true)
                        .value_parser(value_parser!(u32))
                )                
                .arg(
                    Arg::new("source_path")
                        .help("input path")
                        .required(true)
                        .value_parser(value_parser!(String))
                )
                .arg(
                    Arg::new("dest_path")
                        .help("output path")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .last(true)
                )
        )
            
        .subcommand(
            Command::new("collage")
                .about("Combine multiple images into a collage")
                .arg(
                    Arg::new("width")
                        .help("Width of the collage")
                        .required(true)
                        .value_parser(value_parser!(u32))
                )
                .arg(
                    Arg::new("height")
                        .help("Height of the collage")
                        .required(true)
                        .value_parser(value_parser!(u32))
                )
                .arg(
                    Arg::new("image_paths")
                        .help("Paths to the images to include in the collage")
                        .required(true)
                        //.multiple(true)
                        //.min_values(1)
                        .value_delimiter(',')
                        .value_parser(value_parser!(String))
                )
                .arg(
                    Arg::new("dest_path")
                        .help("Output path for the collage")
                        .required(true)
                        .value_parser(value_parser!(String))
                        .last(true)
                )
        )
}
