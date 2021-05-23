use image::Rgb;

use dcc_lsystem::renderer::{Renderer, VideoRendererOptionsBuilder};
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};
use dcc_lsystem::LSystemError;

fn main() -> Result<(), LSystemError> {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("F", TurtleAction::Forward(30))?
        .token("+", TurtleAction::Rotate(90))?
        .token("-", TurtleAction::Rotate(-90))?
        .axiom("F")?
        .rule("F => F + F - F - F + F")?;

    let (mut system, renderer) = builder.finish()?;
    system.step_by(4);

    let options = VideoRendererOptionsBuilder::new()
        .filename("koch_curve.gif")
        .fps(20)
        .skip_by(5)
        .padding(10)
        .thickness(4.0)
        .fill_color(Rgb([255u8, 255u8, 255u8]))
        .line_color(Rgb([0u8, 0u8, 100u8]))
        .progress_bar(true)
        .build();

    renderer.render(&system, &options)?;

    Ok(())
}
