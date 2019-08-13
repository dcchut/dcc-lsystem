use dcc_lsystem::image::{draw_line_mut, fill_mut};
use dcc_lsystem::lattice::{Lattice, LatticeTurtle};
use dcc_lsystem::turtle::Turtle;
use dcc_lsystem::{constant, variable, LSystemBuilder};
use image::{ImageBuffer, Rgb};
use std::f32::consts::FRAC_PI_3;

fn main() {
    let mut builder = LSystemBuilder::new();

    let f = variable!(builder, "F");
    let g = variable!(builder, "G");
    let p = constant!(builder, "+");
    let m = constant!(builder, "-");

    builder.axiom(vec![f, m, g, m, g]);
    builder.transformation_rule(f, vec![f, m, g, p, f, p, g, m, f]);
    builder.transformation_rule(g, vec![g, g]);

    // build our system and step forward a couple of iterations
    let mut system = builder.finish();
    system.step_by(6);

    let line_length = 200.0;
    let thickness = 6.0;
    let mut current_angle = 0;
    let padding: u32 = 20;

    let lattice = Lattice::new(
        (line_length, 0.0),
        (
            line_length * (FRAC_PI_3).cos(),
            line_length * (FRAC_PI_3).sin(),
        ),
    );

    let mut turtle = LatticeTurtle::new(lattice);

    for token in system.get_state() {
        if *token == p {
            current_angle = (current_angle + 1) % 3;
        } else if *token == m {
            current_angle = (current_angle + 2) % 3;
        } else {
            let dx = match current_angle {
                0 => 1,
                1 => -1,
                2 => 0,
                _ => panic!(),
            };

            let dy = match current_angle {
                0 => 0,
                1 => 1,
                2 => -1,
                _ => panic!(),
            };

            turtle.forward((dx, dy));
        }
    }

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
    let xp = |x: i32| -> u32 { (x - min_x + padding as i32) as u32 };

    let yp = |y: i32| -> u32 { (height as i64 - (y - min_y + padding as i32) as i64) as u32 };

    // Draw the lines
    for (x1, y1, x2, y2) in turtle.inner().lines() {
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
        .save("sierpinski_triangle.png")
        .expect("Failed to save to output.png");
}
