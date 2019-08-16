use image::Rgb;

use dcc_lsystem::renderer::{ImageRendererOptions, Renderer};
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};

fn main() {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("A", TurtleAction::Forward(200))
        .token("B", TurtleAction::Forward(200))
        .token("+", TurtleAction::Rotate(60))
        .token("-", TurtleAction::Rotate(-60))
        .axiom("A")
        .rule("A => B - A - B")
        .rule("B => A + B + A");

    let (mut system, renderer) = builder.finish();
    system.step_by(7);

    let options =
        ImageRendererOptions::new(20, 15.0, Rgb([255u8, 255u8, 255u8]), Rgb([0u8, 100u8, 0u8]));

    renderer
        .render(&system, &options)
        .save("sierpinski_arrowhead.png")
        .expect("Failed to save to sierpinski_arrowhead.png");
}
