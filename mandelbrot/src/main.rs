extern crate num;
extern crate image;
use num::Complex;
use std::{str, usize};
use std::{str::FromStr, u8};
use image::{ColorType, ImageError};
use image::png::PngEncoder;
use std::fs::File;
use::std::io::Write;

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex {re: 0.0, im: 0.0};
    loop {
        z = z * z + c;
    }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}
#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,10", ','), Some((10, 10)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

// Parse apair of floating-point mumbers separated by acomma as a complexnumber.__rust_force_expr!
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None 
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"),
        Some(Complex { re: 1.25, im: -0.0625}));

    assert_eq!(parse_complex(",-0.0625"), None);
}

// 出力される画像のピクセルの位置を取り、対応する複素平面上の点を返す。
//
// `bounds` は出力画像の幅と高さをピクセル単位で与える。
// `pixel` は画像上の特定のピクセルを(行, 列)ペアの形で指定する。
// 仮引数 `upper_left`, `lower_right`は、出力画像に描画する複素平面を左上と左下で指定する。
fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize),
    upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re -upper_left.re,
                    upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}
#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
        Complex {re: -1.0, im: 1.0},
        Complex {re: 1.0, im: -1.0}), Complex { re: -0.5, im: -0.5});
}

// Render a rectangle of the Mandelbrot set into a buffer of pixels.
fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0 ..bounds.1 {
        for column in 0 ..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = 
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
                
        }
    }
}


// 大きさが `bounds` で指定されたバッファ `blxels` を `filename` で指定されたファイルに書き出す
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), ImageError> {

    let output = File::create(filename)?;

    let encoder = PngEncoder::new(output);
    encoder.encode(&pixels,
        bounds.0 as u32, bounds.1 as u32,
        ColorType::L8)?;
    Ok(())
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() !=5 {
        writeln!(std::io::stderr(), "
            Usage: mandelbrot Filename PIXELS UPPERLEFT LOWERRIGHT

                Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20
            ", args[0]
        ).unwrap();
        std::process::exit(1)
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left corner point");
    let lower_light = parse_complex(&args[4])
        .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 8 ;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = 
                    pixel_to_point(bounds, (0, top),
                        upper_left, lower_light);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height),
                        upper_left, lower_light);

                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
    }

    write_image(&args[1], &pixels, bounds)
        .expect("error writing PNG file");

}
