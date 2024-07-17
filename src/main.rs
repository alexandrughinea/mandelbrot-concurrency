use num::Complex;
use std::str::FromStr;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }

        z = z * z + c;
    }

    None
}

//Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
fn parse_pair<T: FromStr>(value: &str, separator: char) -> Option<(T, T)> {
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

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", 'x'), None);

    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10x20", 'x'), Some((10, 20)));
}

fn main() {
    let z = Complex { re: 5.9, im: 0.0 };
    println!("complex: {:?}", escape_time(z, 20));
}
