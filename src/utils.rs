use std::fs::File;
use std::str::FromStr;

use image::{ExtendedColorType, ImageEncoder, ImageError};
use image::codecs::png::PngEncoder;
use num::Complex;

pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), ImageError> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);

    encoder.write_image(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ExtendedColorType::L8,
    )?;

    Ok(())
}

pub fn escape_time(c: Complex<f64>, limit: i32) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i as usize);
        }

        z = z * z + c;
    }

    None
}

//Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
pub fn parse_pair<T: FromStr>(value: &str, separator: char) -> Option<(T, T)> {
    match value.find(separator) {
        None => None,
        Some(index) => {
            let left_of_index = T::from_str(&value[..index]);
            let right_of_index = T::from_str(&value[index + separator.len_utf8()..]);

            match (left_of_index, right_of_index) {
                (Ok(x), Ok(y)) => Some((x, y)),
                _ => None,
            }
        }
    }
}

pub fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

/// Parse a pair of floating-point numbers separated by a comma as a complex
/// number.
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] = match escape_time(point, 600) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use num::Complex;

    use crate::utils::{parse_complex, parse_pair, pixel_to_point};

    use super::*;

    #[test]
    fn test_pixel_to_point() {
        assert_eq!(
            pixel_to_point(
                (100, 200),
                (25, 175),
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 },
            ),
            Complex {
                re: -0.5,
                im: -0.75,
            }
        );
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("", 'x'), None);

        assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
        assert_eq!(parse_pair::<i32>("10x20", 'x'), Some((10, 20)));
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(
            parse_complex("1.25,-0.0625"),
            Some(Complex {
                re: 1.25,
                im: -0.0625,
            })
        );
        assert_eq!(parse_complex(",-0.0625"), None);
    }

    #[test]
    fn test_render() {
        let bounds = (3, 3);
        let upper_left = Complex { re: -2.0, im: 1.0 };
        let lower_right = Complex { re: 1.0, im: -1.0 };
        let mut pixels = vec![0; bounds.0 * bounds.1];

        render(&mut pixels, bounds, upper_left, lower_right);

        // Expected values based on the escape time mock function
        let expected_pixels = vec![254, 252, 0, 254, 244, 0, 254, 244, 0];

        assert_eq!(pixels, expected_pixels);
    }
}
