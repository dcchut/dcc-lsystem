use image::Rgb;

use dcc_lsystem::image_renderer::save_png;
use dcc_lsystem::renderer::ImageRendererOptionsBuilder;
use dcc_lsystem::renderer::Renderer;
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};
use dcc_lsystem::LSystemError;
use std::path::Path;

fn main() -> Result<(), LSystemError> {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("X", TurtleAction::Nothing)?
        .token("Y", TurtleAction::Nothing)?
        .token("F", TurtleAction::Forward(30))?
        .token("+", TurtleAction::Rotate(-90))?
        .token("-", TurtleAction::Rotate(90))?
        .axiom("F X")?
        .rule("X => X + Y F +")?
        .rule("Y => - F X - Y")?;

    let (mut system, renderer) = builder.finish()?;
    system.step_by(15);

    let options = ImageRendererOptionsBuilder::new()
        .padding(10)
        .thickness(8.0)
        .fill_color(Rgb([255u8, 255u8, 255u8]))
        .line_color(Rgb([100u8, 0u8, 0u8]))
        .build();

    let buffer = renderer.render(&system, &options);
    save_png(&buffer, Path::new("dragon_curve.png"))?;

    Ok(())
}
