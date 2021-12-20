use image::image_dimensions;
use image::io::Reader as ImageReader;
use image::ImageError;
use std::path::PathBuf;

const MIN_WIDTH: u32 = 1;
const MAX_WIDTH: u32 = std::u32::MAX;
const MIN_HEIGHT: u32 = 1;
const MAX_HEIGHT: u32 = std::u32::MAX;

pub(crate) fn read_image(path: &PathBuf) -> Result<Vec<u8>, ImageError> {
    let img = ImageReader::open(path)?;
    let (width, height) = image_dimensions(path)?;

    assert!(
        width >= MIN_WIDTH && width <= MAX_WIDTH && height <= MAX_HEIGHT && height >= MIN_HEIGHT,
        "image dimensions must be of size (1, 1) -> (1, u32::MAX)"
    );

    let img = match img.decode() {
        Ok(img) => img,
        Err(_) => panic!("failed to load image {:?}", path),
    };

    let pixels: Vec<u8> = img.as_rgb8().unwrap().as_raw().to_vec();
    Ok(pixels)
}
