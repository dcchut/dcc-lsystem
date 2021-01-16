use std::f32::consts::FRAC_PI_2;

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

/// Draws a line to `buffer` between `(x1,y1)` and `(x2,y2)`.
pub fn draw_line_mut(
    buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    thickness: f32,
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
        if x1 == x2 {
            FRAC_PI_2
        } else {
            ((i64::from(y2) - i64::from(y1)) as f64 / (i64::from(x2) - i64::from(x1)) as f64).atan()
                as f32
        }
    };

    // Compute the angle between the line perpendicular to P1 -> P2
    // and the x-axis.
    let perpendicular_angle = angle + FRAC_PI_2;

    // We get the vertices of our rectangle by extending out in the perpendicular
    // direction from our starting point.
    let p1 = Point::new(
        (x1 as f32 + thickness * perpendicular_angle.cos()) as i32,
        (y1 as f32 + thickness * perpendicular_angle.sin()) as i32,
    );
    let p2 = Point::new(
        (x1 as f32 - thickness * perpendicular_angle.cos()) as i32,
        (y1 as f32 - thickness * perpendicular_angle.sin()) as i32,
    );
    let p3 = Point::new(
        (x2 as f32 + thickness * perpendicular_angle.cos()) as i32,
        (y2 as f32 + thickness * perpendicular_angle.sin()) as i32,
    );
    let p4 = Point::new(
        (x2 as f32 - thickness * perpendicular_angle.cos()) as i32,
        (y2 as f32 - thickness * perpendicular_angle.sin()) as i32,
    );

    // Now we just draw the line
    draw_polygon_mut(buffer, &[p1, p3, p4, p2], color);
    draw_filled_circle_mut(
        buffer,
        (x1 as i32, y1 as i32),
        (thickness / 1.5) as i32,
        color,
    );
    draw_filled_circle_mut(
        buffer,
        (x2 as i32, y2 as i32),
        (thickness / 1.5) as i32,
        color,
    );
}
