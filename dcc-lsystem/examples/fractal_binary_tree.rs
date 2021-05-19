use image::Rgb;

use dcc_lsystem::renderer::ImageRendererOptionsBuilder;
use dcc_lsystem::renderer::Renderer;
use dcc_lsystem::turtle::TurtleAction;
use dcc_lsystem::turtle::TurtleLSystemBuilder;
use dcc_lsystem::LSystemError;

fn main() -> Result<(), LSystemError> {
    let mut builder = TurtleLSystemBuilder::new();

    builder
        .token("0", TurtleAction::Forward(50))?
        .token("1", TurtleAction::Forward(50))?
        .token("L", TurtleAction::Rotate(45))?
        .token("R", TurtleAction::Rotate(-45))?
        .token("[", TurtleAction::Push)?
        .token("]", TurtleAction::Pop)?
        .axiom("0")?
        .rule("1 => 1 1")?
        .rule("0 => 1 [ L 0 ] R 0")?
        .rotate(90);

    let (mut system, renderer) = builder.finish()?;
    system.step_by(9);

    let options = ImageRendererOptionsBuilder::new()
        .padding(20)
        .thickness(5.5)
        .fill_color(Rgb([255u8, 255u8, 255u8]))
        .line_color(Rgb([0u8, 100u8, 100u8]))
        .build();

    renderer
        .render(&system, &options)
        .save("fractal_binary_tree.png")
        .expect("Failed to save to fractal_binary_tree.png");

    Ok(())
}
