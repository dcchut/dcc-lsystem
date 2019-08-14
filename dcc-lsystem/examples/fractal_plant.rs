use dcc_lsystem_derive::TurtleContainer;
use image::Rgb;

use dcc_lsystem::renderer::TurtleRenderer;
use dcc_lsystem::turtle::{MovingTurtle, SimpleTurtle, Stack};
use dcc_lsystem::{constant, variable, LSystemBuilder};

#[derive(TurtleContainer)]
struct FractalPlantState {
    angle: i32,
    angle_stack: Vec<i32>,

    #[turtle]
    turtle: SimpleTurtle,
}

impl FractalPlantState {
    pub fn new() -> Self {
        Self {
            angle: 0,
            turtle: SimpleTurtle::new(),
            angle_stack: Vec::new(),
        }
    }
}

fn main() {
    let rotation = 70;
    let line_length = 200;
    let angle = 25;
    let steps = 6;
    let thickness = 18.0;
    let padding = 20;

    let mut builder = LSystemBuilder::new();

    let x = variable!(builder, "X");
    let f = variable!(builder, "F");
    let p = constant!(builder, "+");
    let m = constant!(builder, "-");
    let lsb = constant!(builder, "[");
    let rsb = constant!(builder, "]");

    builder.axiom(vec![x]);
    builder.transformation_rule(
        x,
        vec![
            f, p, lsb, lsb, x, rsb, m, x, rsb, m, f, lsb, m, f, x, rsb, p, x,
        ],
    );
    builder.transformation_rule(f, vec![f, f]);

    // build our system and step forward a couple of iterations
    let mut system = builder.finish();
    system.step_by(steps);

    // Setup our renderer's initial state
    let mut renderer = TurtleRenderer::new(FractalPlantState::new());

    // F => Go forwards in the current direction
    renderer.register(f, move |state| {
        state
            .turtle
            .set_heading((((rotation + state.angle * angle) % 360) as f32).to_radians());
        state.turtle.forward(line_length);
    });
    // + => Turn left by angle degrees
    renderer.register(p, |state| {
        state.angle -= 1;
    });
    // - => Turn right by angle degrees
    renderer.register(m, |state| {
        state.angle += 1;
    });
    // [ => Save the current position and angle
    renderer.register(lsb, |state| {
        state.turtle.push();
        state.angle_stack.push(state.angle);
    });
    // ] => Recover the latest saved position and angle
    renderer.register(rsb, |state| {
        state.turtle.pop();
        state.angle = state.angle_stack.pop().expect("Called pop on empty stack");
    });

    // Render the resulting image based on the LSystem's state
    renderer
        .render(
            &system,
            padding,
            thickness,
            Rgb([255u8, 255u8, 255u8]),
            Rgb([0u8, 100u8, 0u8]),
        )
        .save("fractal_plant.png")
        .expect("Failed to save to fractal_plant.png");
}
