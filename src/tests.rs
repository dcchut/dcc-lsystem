use crate::*;

#[test]
fn basic_algae_test() {
    let mut builder = LSystemBuilder::new();

    let a = builder.token("A", TokenType::Variable);
    let b = builder.token("B", TokenType::Variable);

    builder.axiom(vec![a]);
    builder.transformation_rule(a, vec![a, b]);
    builder.transformation_rule(b, vec![a]);

    let mut system = builder.finish();

    system.step_by(7);

    assert_eq!(system.render(), "ABAABABAABAABABAABABAABAABABAABAAB");
}

#[test]
fn fractal_binary_tree() {
    let mut builder = LSystemBuilder::new();

    let zero = variable!(builder, "0");
    let one = variable!(builder, "1");
    let left_square_bracket = constant!(builder, "[");
    let right_square_bracket = constant!(builder, "]");

    builder.axiom(vec![zero]);
    builder.transformation_rule(one, vec![one, one]);
    builder.transformation_rule(
        zero,
        vec![one, left_square_bracket, zero, right_square_bracket, zero],
    );

    let mut system = builder.finish();

    assert_eq!(system.render(), "0");

    system.step();
    assert_eq!(system.render(), "1[0]0");

    system.step();
    assert_eq!(system.render(), "11[1[0]0]1[0]0");

    system.step();
    assert_eq!(system.render(), "1111[11[1[0]0]1[0]0]11[1[0]0]1[0]0");
}

#[test]
fn macro_usage() {
    let mut builder = LSystemBuilder::new();

    let x = variable!(builder, "X");
    let y = variable!(builder, "Y");

    assert_eq!(x, ArenaId(0));
    assert_eq!(y, ArenaId(1));
}