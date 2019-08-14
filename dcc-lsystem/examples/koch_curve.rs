use dcc_lsystem_derive::TurtleContainer;
use image::Rgb;

use dcc_lsystem::lattice::LatticeTurtle;
use dcc_lsystem::renderer::TurtleRenderer;
use dcc_lsystem::turtle::{Heading, MovingTurtle};
use dcc_lsystem::{constant, variable, LSystemBuilder};

#[derive(TurtleContainer)]
struct KochCurveState {
    heading: Heading,

    #[turtle]
    turtle: LatticeTurtle,
}

impl KochCurveState {
    pub fn new() -> Self {
        Self {
            heading: Heading::East,
            turtle: LatticeTurtle::grid(),
        }
    }
}

fn main() {
    let steps = 3;
    let edge_length = 30;
    let thickness = 4.0;
    let padding = 10;

    // Begin by computing the final state of our LSystem
    let mut builder = LSystemBuilder::new();

    let f = variable!(builder, "F");
    let p = constant!(builder, "+");
    let m = constant!(builder, "-");

    builder.axiom(vec![f]);
    builder.transformation_rule(f, vec![f, p, f, m, f, m, f, p, f]);
    let mut system = builder.finish();
    system.step_by(steps);

    // Now set up our turtle-based renderer
    let mut renderer = TurtleRenderer::new(KochCurveState::new());

    renderer.register(p, |state| {
        state.heading = state.heading.left();
    });
    renderer.register(m, |state| {
        state.heading = state.heading.right();
    });
    renderer.register(f, move |state| {
        state.turtle.forward((
            edge_length * state.heading.dx(),
            edge_length * state.heading.dy(),
        ));
    });

    // Render the final result
    renderer
        .render(
            &system,
            padding,
            thickness,
            Rgb([255u8, 255u8, 255u8]),
            Rgb([0u8, 0u8, 100u8]),
        )
        .save("koch_curve.png")
        .expect("Failed to save to koch_curve.png");
}
