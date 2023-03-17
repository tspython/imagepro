use image::{
    Pixel, 
    Rgba, 
    GenericImageView, 
    ImageFormat,
    ImageBuffer,
    imageops::{resize,FilterType, overlay},
    open,
    DynamicImage,
    RgbaImage, GenericImage,
};
use imageproc::{
    geometric_transformations::{rotate_about_center, Interpolation},
    definitions::Image,
    //drawing::draw
    rect::Rect,
};
use std::sync::{Arc, Mutex};
use std::path::Path;
use crossbeam::thread;
use crate::errors::ImageProcessingError;
use std::thread::spawn;
//use std::sync::Mutex;

pub fn rotate_image<P>(image: &Image<P>, theta: f32, interpolation: Interpolation, default: P) -> Image<P>
where
    P: Pixel<Subpixel = u8> + Send + Sync + 'static,
{
    return rotate_about_center(image, theta, interpolation, default);
}

pub fn crop_image<P>(image: &ImageBuffer<P, Vec<P::Subpixel>>, x: u32, y: u32, width: u32, height: u32) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    P: Pixel + 'static,
{
    let cropped_image = image.view(x, y, width, height).to_image();
    return cropped_image;
}

pub fn resize_image<P>(
    image: &ImageBuffer<P, Vec<P::Subpixel>>,
    width: u32,
    height: u32,
    filter: image::imageops::FilterType,
) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    P: Pixel + 'static,
{
    let resized_image = resize(image, width, height, filter);
    return resized_image;
}

pub fn concat_images_vertically(image_paths: &[&str]) -> DynamicImage {
    // Load all images and get their heights
    let mut images: Vec<DynamicImage> = Vec::new();
    let mut heights: Vec<u32> = Vec::new();
    for path in image_paths {
        let img = image::open(&Path::new(path)).unwrap();
        let height = img.height();
        images.push(img);
        heights.push(height);
    }
    
    // Compute the height of the final image
    let total_height: u32 = heights.iter().sum();
    
    // Create a new image with the same width as the first image and the combined height
    let first_image = &images[0];
    let width = first_image.width();
    let mut result = DynamicImage::new_rgb8(width, total_height);
    
    // Copy each image into the final image, stacked vertically
    let mut y_offset = 0;
    for (img, height) in images.iter().zip(heights.iter()) {
        result.copy_from(img, 0, y_offset).unwrap();
        y_offset += height;
    }
    
    // Return the final image
    result
}

