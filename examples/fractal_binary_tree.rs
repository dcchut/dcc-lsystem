use dcc_lsystem::image::{draw_line_mut, fill_mut};
use dcc_lsystem::turtle::StackTurtle;
use dcc_lsystem::{constant, variable, ArenaId, LSystemBuilder};

use image::{ImageBuffer, Rgb};
use std::f32::consts::FRAC_PI_4;

fn main() {
    let mut builder = LSystemBuilder::new();

    let zero = variable!(builder, "0");
    let one = variable!(builder, "1");
    let lsb = constant!(builder, "[");
    let rsb = constant!(builder, "]");

    // our axiom (i.e. initial condition) is 0
    builder.axiom(vec![zero]);

    // we have two transformation rules: 1 -> 11, and 0 -> 1[0]0
    builder.transformation_rule(one, vec![one, one]);
    builder.transformation_rule(zero, vec![one, lsb, zero, rsb, zero]);

    // build our system and step forward a couple of iterations
    let mut system = builder.finish();
    system.step_by(7);

    // We use our StackTurtle to remember where we should draw each line in our binary tree
    let mut turtle = StackTurtle::new();

    let move_distance = 200.0;
    let angle = FRAC_PI_4;

    for token in system.get_state() {
        if *token == lsb {
            // Push our current position and heading onto the stack, then turn left 45 degrees
            turtle.push();
            turtle.left(angle);
        } else if *token == rsb {
            // Pop our current position and heading off the stack, then turn right 45 degrees
            turtle.pop();
            turtle.right(angle);
        } else {
            // otherwise just move forwards
            turtle.forward(move_distance);
        }
    }

    // Now we want to actually draw the line
    let thickness = 15.5;
    let padding: u32 = 20;

    let (turtle_width, turtle_height, min_x, min_y) = turtle.bounds();

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
    let xp = |x: i32| -> u32 { (x - min_x + padding as i32) as u32 };

    let yp = |y: i32| -> u32 { (height as i64 - (y - min_y + padding as i32) as i64) as u32 };

    // Draw the lines
    for (x1, y1, x2, y2) in turtle.lines() {
        draw_line_mut(
            &mut buffer,
            xp(*x1),
            yp(*y1),
            xp(*x2),
            yp(*y2),
            thickness,
            Rgb([0u8, 0u8, 0u8]),
        );
    }

    buffer
        .save("fractal_binary_tree.png")
        .expect("Failed to save to output.png");
}
