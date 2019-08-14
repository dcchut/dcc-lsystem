use dcc_lsystem::image::{fill_mut, draw_line_mut};
use dcc_lsystem::{constant, variable, LSystemBuilder};

use dcc_lsystem::lattice::LatticeTurtle;
use dcc_lsystem::turtle::{Heading, Turtle};
use image::{ImageBuffer, Rgb};

fn main() {
    let mut builder = LSystemBuilder::new();

    let x = variable!(builder, "X");
    let y = variable!(builder, "Y");
    let f = constant!(builder, "F");
    let p = constant!(builder, "+");
    let m = constant!(builder, "-");

    builder.axiom(vec![f, x]);
    builder.transformation_rule(x, vec![x,p,y,f,p]);
    builder.transformation_rule(y,vec![m,f,x,m,y]);

    // how many steps to do before rendering
    let steps = 15;

    let mut system = builder.finish();
    system.step_by(steps);

    // each edge will be this long
    let walk_distance = 75;
    let thickness = 3.0;

    let mut turtle = LatticeTurtle::grid();
    let mut heading = Heading::East;

    for token in system.get_state() {
        if *token == m {
            heading = heading.left();
        } else if *token == p {
            heading = heading.right();
        } else if *token == f {
            // draw forward
            turtle.forward((walk_distance * heading.dx(), walk_distance * heading.dy()));
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
        draw_line_mut(&mut buffer,
            xp(*x1) as u32,
            yp(*y1) as u32,
            xp(*x2) as u32,
            yp(*y2) as u32,
            thickness,
            Rgb([0u8,0u8,180u8])
        );
    }

    buffer
        .save("dragon_curve.png")
        .expect("Failed to save to output.png");
}
