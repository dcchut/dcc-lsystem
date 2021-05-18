use image::Rgb;

use dcc_lsystem::image_renderer::save_png;
use dcc_lsystem::renderer::{ImageRendererOptionsBuilder, Renderer};
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};
use std::path::Path;

fn main() {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("F", TurtleAction::Forward(30))
        .token("+", TurtleAction::Rotate(90))
        .token("-", TurtleAction::Rotate(-90))
        .axiom("F")
        .rule("F => F + F - F - F + F");

    let (mut system, renderer) = builder.finish();
    system.step_by(7);

    let options = ImageRendererOptionsBuilder::new()
        .padding(10)
        .thickness(4.0)
        .fill_color(Rgb([255u8, 255u8, 255u8]))
        .line_color(Rgb([0u8, 0u8, 100u8]))
        .build();

    let buffer = renderer.render(&system, &options);
    save_png(&buffer, Path::new("koch_curve.png"));
}
