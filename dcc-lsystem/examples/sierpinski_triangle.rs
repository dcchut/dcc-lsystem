use dcc_lsystem_derive::TurtleContainer;
use image::Rgb;

use dcc_lsystem::lattice::LatticeTurtle;
use dcc_lsystem::renderer::TurtleRenderer;
use dcc_lsystem::turtle::MovingTurtle;
use dcc_lsystem::{constant, variable, LSystemBuilder};

#[derive(TurtleContainer)]
struct SierpinskiTriangleState {
    angle: i32,

    #[turtle]
    turtle: LatticeTurtle,
}

impl SierpinskiTriangleState {
    pub fn new() -> Self {
        Self {
            angle: 0,
            turtle: LatticeTurtle::equiangular(),
        }
    }

    pub fn dx(&self) -> i32 {
        match self.angle {
            0 => 1,
            1 => -1,
            2 => 0,
            _ => panic!("Invalid angle encountered"),
        }
    }

    pub fn dy(&self) -> i32 {
        match self.angle {
            0 => 0,
            1 => 1,
            2 => -1,
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

    let f = variable!(builder, "F");
    let g = variable!(builder, "G");
    let p = constant!(builder, "+");
    let m = constant!(builder, "-");

    builder.axiom(vec![f, m, g, m, g]);
    builder.transformation_rule(f, vec![f, m, g, p, f, p, g, m, f]);
    builder.transformation_rule(g, vec![g, g]);

    let mut system = builder.finish();
    system.step_by(steps);

    // Set up the renderer
    let mut renderer = TurtleRenderer::new(SierpinskiTriangleState::new());

    renderer.register(p, |state| {
        state.angle = (state.angle + 1) % 3;
    });
    renderer.register(m, |state| {
        // 2 === -1 mod 3
        state.angle = (state.angle + 2) % 3;
    });
    renderer.register_multiple(&[f, g], move |state| {
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
        .save("sierpinski_triangle.png")
        .expect("Failed to save to sierpinski_triangle.png");
}
