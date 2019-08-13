use dcc_lsystem::image::fill_mut;
use dcc_lsystem::{constant, variable, LSystemBuilder};

use dcc_lsystem::turtle::{Heading, TaxiTurtle, Turtle};
use image::{ImageBuffer, Rgb};

fn main() {
    let mut builder = LSystemBuilder::new();

    let f = variable!(builder, "F");
    let p = constant!(builder, "+");
    let m = constant!(builder, "-");

    builder.axiom(vec![f]);
    builder.transformation_rule(f, vec![f, p, f, m, f, m, f, p, f]);

    // how many steps to do before rendering
    let steps = 3;

    let mut system = builder.finish();
    system.step_by(steps);

    // each edge will be this long
    let walk_distance = 30;

    // our final image has 3^(steps) horizontal boxes, so make our width a multiple of this
    // dont need this
    //let width = 3_u32.pow(step_limit - 1) * 2;

    let mut turtle = TaxiTurtle::new();
    turtle.set_heading(Heading::East);

    for token in system.get_state() {
        if *token == p {
            // turn left
            turtle.left();
        } else if *token == m {
            // turn right
            turtle.right();
        } else {
            // draw forward
            turtle.forward(walk_distance);
        }
    }

    // Now we want to actually draw the line
    let padding: u32 = 10;

    let (turtle_width, turtle_height, min_x, min_y) = turtle.inner().bounds();

    // We add some padding to the width reported by our turtle to make
    // our final image look a little nicer.
    let width = 2 * padding + turtle_width;
    let height = 2 * padding + turtle_height;

    let mut buffer = ImageBuffer::new(width, height);

    // Make the buffer entirely white
    fill_mut(&mut buffer, Rgb([255u8, 255u8, 255u8]));

    // Helper functions for converting between the coordinate system used
    // by the image crate and our coordinate system.  These functions also
    // take care of the padding for us.
    let xp = |x: i32| -> f32 { (x - min_x + padding as i32) as f32 };

    let yp = |y: i32| -> f32 { (height as i64 - (y - min_y + padding as i32) as i64) as f32 };

    // Draw the lines
    for (x1, y1, x2, y2) in turtle.inner().lines() {
        imageproc::drawing::draw_line_segment_mut(
            &mut buffer,
            (xp(*x1), yp(*y1)),
            (xp(*x2), yp(*y2)),
            Rgb([0u8, 0u8, 0u8]),
        );
    }

    buffer
        .save("koch_curve.png")
        .expect("Failed to save to output.png");
}
