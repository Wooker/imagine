use std::{env, ffi::OsString, path::PathBuf};

use ansi_term::Colour;
use crossterm::terminal;
use image::{io::Reader as ImageReader, imageops::FilterType};

const BLOCK: &str = "\u{2588}";

#[derive(Debug)]
enum ImagineError {
    ImageError(String),
    IoError(String),
}

impl From<std::io::Error> for ImagineError {
    fn from(e: std::io::Error) -> Self {
        ImagineError::IoError(e.to_string())
    }
}

impl From<image::ImageError> for ImagineError {
    fn from(e: image::ImageError) -> Self {
        ImagineError::ImageError(e.to_string())
    }
}

type ImagineResult<T> = Result<T, ImagineError>;

fn main() -> ImagineResult<()> {
    let args = env::args_os().collect::<Vec<OsString>>();
    let path = PathBuf::from(args.get(1).expect("No image path provided"));

    let (cols, rows) = terminal::size()?;
    let img = ImageReader::open(path.clone())?.decode()?;

    let resized = img.resize(cols.into(), rows.into(), FilterType::Triangle);

    let pixels = resized.as_rgb8().expect("Could not convert to rgb8").enumerate_pixels();
    for pixel in pixels {
        let rgb = pixel.2;
        print!("{}", Colour::RGB(rgb[0], rgb[1], rgb[2]).paint(BLOCK));
        if pixel.0 == resized.width() - 1 {
            println!();
        }
    }

    Ok(())
}
