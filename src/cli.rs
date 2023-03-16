use clap:: {
    arg,
    Args,
    Parser,
    Subcommand, 
    Command,
    Arg,
    value_parser,
};
use std::any::TypeId;
use std::ffi::OsString;
use std::path::PathBuf; 

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
                .arg(arg!(x: [F32]))
                .arg(arg!(y: [F32]))
                .arg(arg!(width: [F32]))
                .arg(arg!(height: [F32]))
                .arg(arg!(source_path: [PATH]))
                .arg(arg!(dest_Path: [PATH]).last(true))

        )
        .subcommand(
            Command::new("resize")
                .about("Resize")
                .arg(arg!(width: [F32]))
                .arg(arg!(height: [F32]))
                .arg(arg!(source_path: [PATH]))
                .arg(arg!(dest_Path: [PATH]).last(true))
        )
} 
