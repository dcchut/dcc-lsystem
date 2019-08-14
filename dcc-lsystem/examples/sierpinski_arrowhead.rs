use dcc_lsystem_derive::TurtleContainer;
use image::Rgb;

use dcc_lsystem::lattice::LatticeTurtle;
use dcc_lsystem::renderer::TurtleRenderer;
use dcc_lsystem::turtle::MovingTurtle;
use dcc_lsystem::{constant, variable, LSystemBuilder};

#[derive(TurtleContainer)]
struct SierpinskiArrowheadState {
    angle: i32,

    #[turtle]
    turtle: LatticeTurtle,
}

impl SierpinskiArrowheadState {
    pub fn new() -> Self {
        Self {
            angle: 0,
            turtle: LatticeTurtle::equiangular(),
        }
    }

    pub fn dx(&self) -> i32 {
        match self.angle {
            0 => 1,
            1 => 0,
            2 => -1,
            3 => -1,
            4 => 0,
            5 => 1,
            _ => panic!("Invalid angle encountered"),
        }
    }

    pub fn dy(&self) -> i32 {
        match self.angle {
            0 => 0,
            1 => 1,
            2 => 1,
            3 => 0,
            4 => -1,
            5 => -1,
            _ => panic!("Invalid angle encountered"),
        }
    }
}

fn main() {
    let steps = 7;
    let line_length = 200;
    let thickness = 8.0;
    let padding = 20;

    // Build up our LSystem
    let mut builder = LSystemBuilder::new();

    let a = variable!(builder, "A");
    let b = variable!(builder, "B");
    let p = constant!(builder, "+");
    let m = constant!(builder, "-");

    builder.axiom(vec![a]);
    builder.transformation_rule(a, vec![b, m, a, m, b]);
    builder.transformation_rule(b, vec![a, p, b, p, a]);

    let mut system = builder.finish();
    system.step_by(steps);

    // Set up the renderer
    let mut renderer = TurtleRenderer::new(SierpinskiArrowheadState::new());

    renderer.register(p, |state| {
        state.angle = (state.angle + 1) % 6;
    });
    renderer.register(m, |state| {
        // 5 === -1 mod 6
        state.angle = (state.angle + 5) % 6;
    });
    renderer.register_multiple(&[a, b], move |state| {
        state
            .turtle
            .forward((line_length * state.dx(), line_length * state.dy()));
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
        .save("sierpinski_arrowhead.png")
        .expect("Failed to save to sierpinski_arrowhead.png");
}
