use image::Rgb;

use dcc_lsystem::renderer::{ImageRendererOptionsBuilder, Renderer};
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};
use dcc_lsystem::LSystemError;

fn main() -> Result<(), LSystemError> {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("A", TurtleAction::Forward(200))?
        .token("B", TurtleAction::Forward(200))?
        .token("+", TurtleAction::Rotate(60))?
        .token("-", TurtleAction::Rotate(-60))?
        .axiom("A")?
        .rule("A => B - A - B")?
        .rule("B => A + B + A")?;

    let (mut system, renderer) = builder.finish()?;
    system.step_by(7);

    let options = ImageRendererOptionsBuilder::new()
        .padding(20)
        .thickness(15.0)
        .fill_color(Rgb([255u8, 255u8, 255u8]))
        .line_color(Rgb([0u8, 100u8, 0u8]))
        .build();

    renderer
        .render(&system, &options)
        .save("sierpinski_arrowhead.png")
        .expect("Failed to save to sierpinski_arrowhead.png");

    Ok(())
}
