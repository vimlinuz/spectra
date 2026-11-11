mod cli;

use clap::Parser;
use cli::Args;

use image::{ImageReader, RgbaImage};

use faer::{Mat, linalg::solvers::Svd};

struct ImageChannels {
    red: ImageChannelMatrix,
    green: ImageChannelMatrix,
    blue: ImageChannelMatrix,
    alpha: ImageChannelMatrix,
}

impl ImageChannels {
    fn from_rgba(img: RgbaImage) -> ImageChannels {
        let (height, width) = img.dimensions();

        let mut channels = Self {
            red: ImageChannelMatrix::new(width as usize, height as usize),
            green: ImageChannelMatrix::new(width as usize, height as usize),
            blue: ImageChannelMatrix::new(width as usize, height as usize),
            alpha: ImageChannelMatrix::new(width as usize, height as usize),
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

    fn to_rgba(self) -> RgbaImage {
        let width = self.red.width as u32;
        let height = self.red.height as u32;

        RgbaImage::from_fn(width, height, |x, y| {
            let row = y as usize;
            let col = x as usize;

            let red = self.red.get(row, col).round().clamp(0.0, 255.0) as u8;
            let green = self.green.get(row, col).round().clamp(0.0, 255.0) as u8;
            let blue = self.blue.get(row, col).round().clamp(0.0, 255.0) as u8;
            let alpha = self.alpha.get(row, col).round().clamp(0.0, 255.0) as u8;

            image::Rgba([red, green, blue, alpha])
        })
    }
}

struct ImageChannelMatrix {
    width: usize,
    height: usize,
    data: Vec<f64>,
}

impl ImageChannelMatrix {
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

    fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.width + col]
    }

    fn from_mat(mat: &Mat<f64>) -> Self {
        let mut channel = Self::new(mat.ncols(), mat.nrows());

        for row in 0..mat.nrows() {
            for col in 0..mat.ncols() {
                channel.set(row, col, mat[(row, col)]);
            }
        }

        channel
    }
}

struct Compressor {
    rank: usize,
}

impl Compressor {
    fn new(rank: usize) -> Compressor {
        Compressor { rank }
    }

    fn decompose(a: Mat<f64>) -> Svd<f64> {
        Svd::new_thin(a.as_ref()).unwrap()
    }

    fn reconstruct_channel(svd: &Svd<f64>, k: usize) -> Mat<f64> {
        let u = svd.U();
        let v = svd.V();
        let s = svd.S();

        let uk = u.subcols(0, k);
        let vtk = v.transpose().subrows(0, k);

        let scaled_u = Mat::from_fn(uk.nrows(), k, |i, j| uk[(i, j)] * s[j]);

        &scaled_u * &vtk
    }

    fn get_channel_matrix(channel: ImageChannelMatrix) -> Mat<f64> {
        Mat::<f64>::from_fn(channel.height, channel.width, |row, col| {
            channel.get(row, col)
        })
    }

    fn compress(self: Self, channel: ImageChannels) -> ImageChannels {
        let k = self.rank;
        let red_channel_matrix = Self::get_channel_matrix(channel.red);
        let green_channel_matrix = Self::get_channel_matrix(channel.green);
        let blue_channel_matrix = Self::get_channel_matrix(channel.blue);

        let red_channel_decomposition = Self::decompose(red_channel_matrix);
        let green_channel_decomposition = Self::decompose(green_channel_matrix);
        let blue_channel_decomposition = Self::decompose(blue_channel_matrix);

        let reconstructed_red_channel = Self::reconstruct_channel(&red_channel_decomposition, k);
        let reconstructed_green_channel =
            Self::reconstruct_channel(&green_channel_decomposition, k);
        let reconstrudted_blue_channel = Self::reconstruct_channel(&blue_channel_decomposition, k);

        ImageChannels {
            red: ImageChannelMatrix::from_mat(&reconstructed_red_channel),
            green: ImageChannelMatrix::from_mat(&reconstructed_green_channel),
            blue: ImageChannelMatrix::from_mat(&reconstrudted_blue_channel),
            alpha: channel.alpha,
        }
    }
}

fn main() {
    let args = Args::parse();
    let input_path = args.input;
    let singular_value = args.rank;
    let output_path = args.output;

    let img = ImageReader::open(input_path)
        .expect("Image not found")
        .decode()
        .expect("cound't decode the image")
        .to_rgba8();

    let image_channels = ImageChannels::from_rgba(img);
    let compressor = Compressor::new(singular_value);
    let compressed_image_channel = compressor.compress(image_channels);
    let compressed_rgba = compressed_image_channel.to_rgba();
    compressed_rgba.save(output_path).unwrap()
}
