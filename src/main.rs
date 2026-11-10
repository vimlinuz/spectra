#![allow(unused)]
mod cli;

use clap::Parser;
use cli::Args;

use image::{ImageReader, RgbaImage};

struct ImageChannels {
    red: ImageMatrix,
    green: ImageMatrix,
    blue: ImageMatrix,
    alpha: ImageMatrix,
}

impl ImageChannels {
    fn from_rgba(img: RgbaImage) -> ImageChannels {
        let (height, width) = img.dimensions();

        let mut channels = Self {
            red: ImageMatrix::new(width as usize, height as usize),
            green: ImageMatrix::new(width as usize, height as usize),
            blue: ImageMatrix::new(width as usize, height as usize),
            alpha: ImageMatrix::new(width as usize, height as usize),
        };

        for (x, y, pixel) in img.enumerate_pixels() {
            let x = x as usize;
            let y = y as usize;

            channels.red.set(y, x, pixel[0] as f64);
            channels.green.set(y, x, pixel[1] as f64);
            channels.blue.set(y, x, pixel[2] as f64);
            channels.alpha.set(y, x, pixel[3] as f64);
        }

        channels
    }
}

struct ImageMatrix {
    width: usize,
    height: usize,
    data: Vec<f64>,
}

impl ImageMatrix {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
        }
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.width + col] = value;
    }
}

fn main() {
    let args = Args::parse();
    let input_path = args.input;

    let img = ImageReader::open(input_path)
        .expect("Image not found")
        .decode()
        .expect("cound't decode the image")
        .to_rgba8();
}
