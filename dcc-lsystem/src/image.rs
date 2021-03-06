use std::f64::consts::FRAC_PI_2;

use image::{ImageBuffer, Rgb};
use imageproc::drawing::{draw_filled_circle_mut, draw_polygon_mut};
use imageproc::point::Point;

///  Modified every pixel of `buffer` to be the provided color.
///
/// # Example
/// ```rust,no_run
/// use image::{ImageBuffer, Rgb};
/// use dcc_lsystem::image::fill_mut;
///
/// let mut  buffer : ImageBuffer<Rgb<u8>, Vec<u8>> = unimplemented!();
///
/// // Make our image entirely black.
/// fill_mut(&mut buffer, Rgb([0u8,0u8,0u8]));
/// ```
pub fn fill_mut(buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, color: Rgb<u8>) {
    for pixel in buffer.pixels_mut() {
        *pixel = color;
    }
}

#[inline(always)]
fn _r(x: f64) -> i32 {
    x.round() as i32
}

/// Draws a line to `buffer` between `(x1,y1)` and `(x2,y2)`.
pub fn draw_line_mut(
    buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    thickness: f64,
    color: Rgb<u8>,
) {
    assert!(thickness > 0.0);

    // A thick line is really just a rectangle.  It seems that
    // imageproc::drawing::draw_filled_rect() only allows drawing axis-aligned
    // rectangles, so we'd need to think a little harder to use that method here.
    //
    // Instead, we compute the four vertices of our rectangle and use the
    // the draw_polygon_mut function from imageproc.  This works fairly well,
    // though there are some noticeable issues where two lines connect.  As a workaround,
    // we also draw a circle at the start and endpoint of our line - this covers up most
    // of the badness, and gives a reasonable looking end result.

    // Compute the angle between the first and second points
    let angle = {
        if (x1 - x2).abs() < f64::EPSILON {
            FRAC_PI_2
        } else {
            ((y2 - y1) / (x2 - x1)).atan()
        }
    };

    // Compute the angle between the line perpendicular to P1 -> P2
    // and the x-axis.
    let perpendicular_angle = angle + FRAC_PI_2;

    let dx = thickness * perpendicular_angle.cos();
    let dy = thickness * perpendicular_angle.sin();

    // We get the vertices of our rectangle by extending out in the perpendicular
    // direction from our starting point.
    let p1 = Point::new(_r(x1 + dx), _r(y1 + dy));
    let p2 = Point::new(_r(x1 - dx), _r(y1 - dy));
    let p3 = Point::new(_r(x2 + dx), _r(y2 + dy));
    let p4 = Point::new(_r(x2 - dx), _r(y2 - dy));

    // Now we just draw the line
    if p1 != p2 {
        // imageproc will panic if the first and last points in the polygon are the same.
        draw_polygon_mut(buffer, &[p1, p3, p4, p2], color);
    }
    draw_filled_circle_mut(buffer, (_r(x1), _r(y1)), _r(thickness / 1.5), color);
    draw_filled_circle_mut(buffer, (_r(x2), _r(y2)), _r(thickness / 1.5), color);
}
