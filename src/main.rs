use clap::Parser;
use image::{imageops::FilterType, open};
use std::path::{Path, PathBuf};

mod cli;
mod processing;
mod errors;

use crate::errors::ImageProcessingError;
//mod image_handling;
fn main() {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("rotate", sub_matches)) => {
            let degrees = sub_matches.get_one::<f32>("degrees").unwrap(); 
            let source_path = sub_matches.get_one::<String>("source_path").unwrap();
            let dest_path = sub_matches.get_one::<String>("dest_path").unwrap();
            let degrees = &degrees_to_radians(*degrees);
            let image = match open(Path::new(&source_path)) {
                Ok(img) => img.to_rgba8(),
                Err(err) => {
                    eprintln!("Error opening source image: {}", ImageProcessingError::from(err));
                    return;
                }
            };
            let rotated_image = processing::rotate_image(
                &image,
                *degrees,
                imageproc::geometric_transformations::Interpolation::Bilinear,
                image::Rgba([0, 0, 0, 0]),
            );
            if let Err(err) = rotated_image.save(dest_path) {
                eprintln!("Error saving destination image: {}", ImageProcessingError::from(err));
            }
        }

        Some(("crop", sub_matches)) => {
            let x = sub_matches.get_one::<u32>("x").unwrap();
            let y = sub_matches.get_one::<u32>("y").unwrap();
            let width = sub_matches.get_one::<u32>("width").unwrap();
            let height = sub_matches.get_one::<u32>("height").unwrap();
            let source_path = sub_matches.get_one::<String>("source_path").unwrap();
            let dest_path = sub_matches.get_one::<String>("dest_path").unwrap();

            let image = open(Path::new(&source_path)).unwrap().to_rgba8();
            let cropped_image = processing::crop_image(&image, *x, *y, *width, *height);
            cropped_image.save(dest_path).unwrap();
        }

        Some(("resize", sub_matches)) => {
            let width = sub_matches.get_one::<u32>("width").unwrap();
            let height = sub_matches.get_one::<u32>("height").unwrap();
            let source_path = sub_matches.get_one::<String>("source_path").unwrap();
            let dest_path = sub_matches.get_one::<String>("dest_path").unwrap();

            let image = open(Path::new(&source_path)).unwrap().to_rgba8();
            let resized_image =
                processing::resize_image(&image, *width, *height, FilterType::Lanczos3);
            resized_image.save(dest_path).unwrap();
        }
        Some(("collage", sub_matches)) => {
            let collage_width = sub_matches.get_one::<u32>("width").unwrap();
            let collage_height = sub_matches.get_one::<u32>("height").unwrap();
            let image_paths_vec = sub_matches
                .get_many::<String>("image_paths")
                .unwrap()
                .map(|value_ref| value_ref.as_str())
                .collect::<Vec<_>>();
            let dest_path = sub_matches.get_one::<String>("dest_path").unwrap();
            let collage = processing::concat_images_vertically(&image_paths_vec);
            collage.save(dest_path);
        }

        _ => unreachable!(),
    }
}

fn degrees_to_radians(degrees: f32) -> f32 {
    const PI: f32 = 3.14159265358979323846;
    degrees * (PI / 180.0)
}

