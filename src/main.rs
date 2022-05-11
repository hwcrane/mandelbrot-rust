extern crate colorgrad;
extern crate image;
extern crate num;
extern crate rayon;

use image::{ImageBuffer, Rgb};
use num::complex::Complex;
use rayon::prelude::*;

const WIDTH: u64 = 15000;
const HEIGHT: u64 = 10000;
const ITERATIONS: u32 = 100;

fn main() {
    let mut pixels = vec![Rgb([0 as u8, 0 as u8, 0 as u8]); (WIDTH * HEIGHT) as usize];

    let colourgrad = colorgrad::spectral().sharp(11, 0.7);

    (pixels).par_iter_mut().enumerate().for_each(|(i, pixel)| {
        let x = i % WIDTH as usize;
        let y = i / WIDTH as usize;

        let num = pixel_to_complex(x, y, Complex::new(-2., -1.), Complex::new(1., 1.));
        let value = fract(num, ITERATIONS) as f64 * (1. / ITERATIONS as f64);
        let colour = colourgrad.at(value).rgba_u8();
        *pixel = Rgb([colour.0, colour.1, colour.2]);
    });

    let mut imgbuf = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = pixels[(x + WIDTH as u32 * y) as usize];
    }

    imgbuf.save("out.png").unwrap()
}

fn pixel_to_complex(
    x: usize,
    y: usize,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let re_plane_width = lower_right.re - upper_left.re;
    let im_plane_height = upper_left.im - lower_right.im;

    Complex {
        re: upper_left.re + x as f64 * re_plane_width / WIDTH as f64,
        im: upper_left.im - y as f64 * im_plane_height / HEIGHT as f64,
    }
}

fn fract(num: Complex<f64>, max: u32) -> u32 {
    let c1 = num;
    let mut c2 = Complex::new(0., 0.);
    for n in 1..max {
        if c2.norm_sqr() > 4. {
            return n;
        }
        c2 = c2 * c2 + c1
    }
    max
}
