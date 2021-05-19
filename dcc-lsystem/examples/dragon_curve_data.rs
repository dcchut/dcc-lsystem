use dcc_lsystem::renderer::DataRendererOptions;
use dcc_lsystem::renderer::Renderer;
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};
use dcc_lsystem::LSystemError;

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

    let rv = renderer.render(&system, &DataRendererOptions::default());
    println!("Dragon curve builder generated {} lines", rv.len());

    Ok(())
}
