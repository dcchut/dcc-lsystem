use image::Rgb;

use dcc_lsystem::renderer::{ImageRendererOptionsBuilder, Renderer};
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

    let options = ImageRendererOptionsBuilder::new()
        .padding(20)
        .thickness(8.0)
        .fill_color(Rgb([255u8, 255u8, 255u8]))
        .line_color(Rgb([0u8, 100u8, 0u8]))
        .build();

    renderer
        .render(&system, &options)
        .save("sierpinski_triangle.png")
        .expect("Failed to save to sierpinski_triangle.png");
}
