use num::Complex;

/// Determine if `c` is in the Mandelbrot set or not, based in part
/// on the `limit` parameter, which specifies how many "attempts"
/// the program gets to figure it out.
/// 
/// If `c` is not a member, returns `Some(i)` where `i` is the number
/// of iterations it took for `c` to leave the circle of radius 2 centered
/// at the origin. If `c` is a member of the set, i.e., if we reached the 
/// iteration limit without being able to prove that `c` is _not_ a member,
/// return `None`
fn in_mandelbrot_set(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z * z + c;

        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im + pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    let answer = pixel_to_point(
        (100, 100), 
        (25, 75),
        Complex { re: -1.0, im:  1.0 },
        Complex { re:  1.0, im: -1.0 }
    );

    let expected = Complex { re: -0.5, im: -0.5 }); 

    assert_eq!(answer, expected);
}
