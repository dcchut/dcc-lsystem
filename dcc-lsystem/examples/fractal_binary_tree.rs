use std::f32::consts::FRAC_PI_4;

use dcc_lsystem_derive::TurtleContainer;
use image::Rgb;

use dcc_lsystem::renderer::TurtleRenderer;
use dcc_lsystem::turtle::{MovingTurtle, SimpleTurtle, Stack};
use dcc_lsystem::{constant, variable, LSystemBuilder};

#[derive(TurtleContainer)]
struct FractalBinaryTreeState {
    #[turtle]
    turtle: SimpleTurtle,
}

impl FractalBinaryTreeState {
    pub fn new() -> Self {
        Self {
            turtle: SimpleTurtle::new(),
        }
    }
}

fn main() {
    let steps = 9;
    let angle = FRAC_PI_4;
    let move_distance = 200;
    let thickness = 15.5;
    let padding = 20;

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
    system.step_by(steps);

    let mut renderer = TurtleRenderer::new(FractalBinaryTreeState::new());

    renderer.register(lsb, move |state| {
        state.turtle.push();
        state.turtle.left(angle);
    });

    renderer.register(rsb, move |state| {
        state.turtle.pop();
        state.turtle.right(angle);
    });

    renderer.register_multiple(&[zero, one], move |state| {
        state.turtle.forward(move_distance);
    });

    renderer
        .render(
            &system,
            padding,
            thickness,
            Rgb([255u8, 255u8, 255u8]),
            Rgb([0u8, 100u8, 100u8]),
        )
        .save("fractal_binary_tree.png")
        .expect("Failed to save to fractal_binary_tree.png");
}
