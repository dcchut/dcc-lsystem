use image::Rgb;

use dcc_lsystem::renderer::{ImageRendererOptions, Renderer};
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};

fn main() {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("F", TurtleAction::Forward(200))
        .token("G", TurtleAction::Forward(200))
        .token("+", TurtleAction::Rotate(120))
        .token("-", TurtleAction::Rotate(-120))
        .axiom("F - G - G")
        .rule("F => F - G + F + G - F")
        .rule("G => G G");

    let (mut system, renderer) = builder.finish();
    system.step_by(7);

    let options =
        ImageRendererOptions::new(20, 8.0, Rgb([255u8, 255u8, 255u8]), Rgb([0u8, 100u8, 0u8]));

    renderer
        .render(&system, &options)
        .save("sierpinski_triangle.png")
        .expect("Failed to save to sierpinski_triangle.png");
}
