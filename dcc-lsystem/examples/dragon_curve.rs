use dcc_lsystem_derive::TurtleContainer;
use image::Rgb;

use dcc_lsystem::lattice::LatticeTurtle;
use dcc_lsystem::renderer::TurtleRenderer;
use dcc_lsystem::turtle::{Heading, MovingTurtle};
use dcc_lsystem::{constant, variable, LSystemBuilder};

#[derive(TurtleContainer)]
struct DragonCurveState {
    heading: Heading,

    #[turtle]
    turtle: LatticeTurtle,
}

impl DragonCurveState {
    pub fn new() -> Self {
        Self {
            heading: Heading::East,
            turtle: LatticeTurtle::grid(),
        }
    }
}

fn main() {
    let steps = 15;
    let edge_length = 30;
    let thickness = 8.0;
    let padding = 10;

    // Set up our LSystem
    let mut builder = LSystemBuilder::new();

    let x = variable!(builder, "X");
    let y = variable!(builder, "Y");
    let f = constant!(builder, "F");
    let p = constant!(builder, "+");
    let m = constant!(builder, "-");

    builder.axiom(vec![f, x]);
    builder.transformation_rule(x, vec![x, p, y, f, p]);
    builder.transformation_rule(y, vec![m, f, x, m, y]);
    let mut system = builder.finish();
    system.step_by(steps);

    let mut renderer = TurtleRenderer::new(DragonCurveState::new());

    renderer.register(m, |state| {
        state.heading = state.heading.left();
    });
    renderer.register(p, |state| {
        state.heading = state.heading.right();
    });
    renderer.register(f, move |state| {
        state.turtle.forward((
            edge_length * state.heading.dx(),
            edge_length * state.heading.dy(),
        ));
    });

    // Render the resulting image based on the LSystem's state
    renderer
        .render(
            &system,
            padding,
            thickness,
            Rgb([255u8, 255u8, 255u8]),
            Rgb([100u8, 0u8, 0u8]),
        )
        .save("dragon_curve.png")
        .expect("Failed to save to dragon_curve.png");
}
