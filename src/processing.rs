use image::{
    Pixel, 
    Rgba, 
    GenericImageView, 
    ImageBuffer, 
    ImageFormat,
    imageops::resize,
};
use imageproc::{
    geometric_transformations::{rotate_about_center, Interpolation},
    definitions::Image,
};

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

