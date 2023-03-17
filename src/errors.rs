#[derive(Debug)]
pub enum ImageProcessingError {
    IoError(std::io::Error),
    ImageError(image::ImageError),
}

impl std::fmt::Display for ImageProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "I/O error: {}", e),
            Self::ImageError(e) => write!(f, "Image error: {}", e),
        }
    }
}

impl std::error::Error for ImageProcessingError {}

impl From<std::io::Error> for ImageProcessingError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<image::ImageError> for ImageProcessingError {
    fn from(err: image::ImageError) -> Self {
        Self::ImageError(err)
    }
}

