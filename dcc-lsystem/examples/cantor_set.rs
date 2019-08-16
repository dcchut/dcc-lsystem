use image::{ImageBuffer, Rgb};

use dcc_lsystem::image::fill_mut;
use dcc_lsystem::LSystemBuilder;

fn main() {
    let mut builder = LSystemBuilder::new();

    let a = builder.token("A");
    let b = builder.token("B");

    builder.axiom(vec![a]);
    builder.transformation_rule(a, vec![a, b, a]);
    builder.transformation_rule(b, vec![b, b, b]);

    let mut system = builder.finish();

    // the total number of states (including the initial state!) to render
    let step_limit = 6;

    // At state number `step_limit`, our diagram has 3^(step_limit - 1) bars,
    // so we make the width of our image an integer multiple of this number.
    let width = 3_u32.pow(step_limit - 1) * 2;

    // the vertical spacing between each bar in the render
    let vertical_spacing = 5;

    let mut lines = Vec::new();

    for index in 0..step_limit {
        let state = system.get_state();
        let bar_width: u32 = width / state.len() as u32;

        let mut x: u32 = 0;
        let y = vertical_spacing * index;

        for token in state {
            if *token == a {
                // draw a line
                lines.push((x, y, x + bar_width, y));
            }
            x += bar_width;
        }

        system.step();
    }

    let padding: u32 = 5;
    let render_width = 2 * padding + width as u32;
    let render_height = 2 * padding + vertical_spacing * (step_limit - 1);

    let mut buffer = ImageBuffer::new(render_width, render_height);

    // Make the buffer entirely white
    fill_mut(&mut buffer, Rgb([255u8, 255u8, 255u8]));

    // Draw the lines
    for (x1, y1, x2, y2) in lines.into_iter() {
        for x in x1..=x2 {
            for y in y1..=y2 {
                let pixel = buffer.get_pixel_mut(x + padding, y + padding);
                *pixel = Rgb([0u8, 0u8, 0u8]);
            }
        }
    }

    buffer
        .save("cantor_set.png")
        .expect("Failed to save to cantor_set.png");
}
