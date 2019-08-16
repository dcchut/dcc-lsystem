use image::Rgb;

use dcc_lsystem::renderer::{ImageRendererOptions, Renderer};
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};

fn main() {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("X", TurtleAction::Nothing)
        .token("F", TurtleAction::Forward(200))
        .token("+", TurtleAction::Rotate(25))
        .token("-", TurtleAction::Rotate(-25))
        .token("[", TurtleAction::Push)
        .token("]", TurtleAction::Pop)
        .axiom("X")
        .rule("X => F + [ [ X ] - X ] - F [ - F X ] + X")
        .rule("F => F F")
        .rotate(70);

    let (mut system, renderer) = builder.finish();
    system.step_by(6);

    let options =
        ImageRendererOptions::new(20, 18.0, Rgb([255u8, 255u8, 255u8]), Rgb([0u8, 100u8, 0u8]));

    renderer
        .render(&system, &options)
        .save("fractal_plant.png")
        .expect("Failed to save fractal_plant.png");
}