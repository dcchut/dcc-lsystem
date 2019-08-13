use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_convex_polygon_mut;
use imageproc::drawing::Point;
use std::f32::consts::FRAC_PI_2;

pub fn fill_mut(buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, color: Rgb<u8>) {
    for pixel in buffer.pixels_mut() {
        *pixel = color;
    }
}

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

    // Compute the angle from the first point to the second point
    let angle = {
        if x1 == x2 {
            FRAC_PI_2
        } else {
            (((y2 - y1) / (x2 - x1)) as f32).atan()
        }
    };

    // Compute the angle of the line perpendicular to the line from the first
    // point to the second point
    let perpendicular_angle = angle + FRAC_PI_2;

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

    // Draw the convex shape
    draw_convex_polygon_mut(buffer, &[p1, p3, p4, p2], color);
}
