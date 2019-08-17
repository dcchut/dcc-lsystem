use image::Rgb;

use dcc_lsystem::renderer::{Renderer, VideoRendererOptions};
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};

fn main() {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("F", TurtleAction::Forward(30))
        .token("+", TurtleAction::Rotate(90))
        .token("-", TurtleAction::Rotate(-90))
        .axiom("F")
        .rule("F => F + F - F - F + F");

    let (mut system, renderer) = builder.finish();
    system.step_by(5);

    let options = VideoRendererOptions::new(
        "koch_curve.gif",
        20,
        0,
        10,
        4.0,
        Rgb([255u8, 255u8, 255u8]),
        Rgb([0u8, 0u8, 100u8]),
        true,
    );

    renderer.render(&system, &options);
}
