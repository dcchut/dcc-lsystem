use image::Rgb;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use dcc_lsystem::renderer::ImageRendererOptions;
use dcc_lsystem::renderer::Renderer;
use dcc_lsystem::turtle::{TurtleAction, TurtleLSystemBuilder};

fn valid_rule(rule: &Vec<&str>) -> bool {
    if rule.len() == 0 {
        return false;
    }

    let r = rule.join("");

    if r.contains("+-") || !r.contains("+") {
        return false;
    }

    let mut level = 0;

    for c in rule {
        if c == &"+" {
            level += 1;
        } else if c == &"-" {
            if level == 0 {
                return false;
            }
            level -= 1;
        }
    }

    level == 0
}

pub fn main() {
    'processing: loop {
        // generate random axiom out of L, R, F, X, Y
        // and random rules for X, Y
        let mut rng = thread_rng();

        // generate a random axiom
        let axiom_length = rng.gen_range(0, 2);
        let mut axiom = vec!["X"];
        let choices = ["L", "R", "F", "X", "Y"];
        let weighted_choices = [
            ("F", rng.gen_range(1, 8)),
            ("X", rng.gen_range(2, 4)),
            ("Y", rng.gen_range(2, 4)),
            ("L", rng.gen_range(2, 6)),
            ("R", rng.gen_range(2, 6)),
            ("+", rng.gen_range(4, 8)),
            ("-", rng.gen_range(4, 8)),
        ];

        for _ in 0..axiom_length {
            axiom.push(choices.choose(&mut rng).unwrap().clone());
        }

        // generate a random X rule
        let mut x_rule = Vec::new();

        while !valid_rule(&x_rule) {
            x_rule.clear();

            let x_rule_length = rng.gen_range(4, 10);

            for _ in 0..x_rule_length {
                x_rule.push(
                    weighted_choices
                        .choose_weighted(&mut rng, |item| item.1)
                        .unwrap()
                        .0
                        .clone(),
                );
            }
        }

        let mut y_rule = Vec::new();

        while !valid_rule(&y_rule) {
            y_rule.clear();
            // generate a random Y rule
            let y_rule_length = rng.gen_range(4, 10);

            for _ in 0..y_rule_length {
                y_rule.push(
                    weighted_choices
                        .choose_weighted(&mut rng, |item| item.1)
                        .unwrap()
                        .0
                        .clone(),
                );
            }
        }

        let mut builder = TurtleLSystemBuilder::new();

        // Build our system up
        builder
            .token("L", TurtleAction::Rotate(25))
            .token("R", TurtleAction::Rotate(-25))
            .token("F", TurtleAction::Forward(100))
            //.token("L", TurtleAction::StochasticRotate(Box::new(Uniform::new(60,70))))
            //.token("L", TurtleAction::Rotate(90))
            //.token("R", TurtleAction::StochasticRotate(Box::new(Uniform::new(-70,-60))))
            //.token("R", TurtleAction::Rotate(-90))
            //.token("F", TurtleAction::StochasticForward(Box::new(Uniform::new(5,10))))
            //.token("F", TurtleAction::Forward(10))
            .token("+", TurtleAction::Push)
            .token("-", TurtleAction::Pop)
            .token("X", TurtleAction::Nothing)
            .token("Y", TurtleAction::Nothing)
            .axiom(&axiom.join(" "))
            .rule(format!("X => {}", x_rule.join(" ")).as_str())
            .rule(format!("Y => {}", y_rule.join(" ")).as_str());

        // Consume the builder to construct an LSystem and the associated renderer
        let (mut system, renderer) = builder.finish();

        let options =
            ImageRendererOptions::new(20, 1.0, Rgb([0u8, 0u8, 0u8]), Rgb([218u8, 112u8, 214u8]));

        // Iterate the system a few times
        system.step_by(10);

        // Render away
        let buffer = renderer.render(&system, &options);

        if buffer.width() < 1000 || buffer.height() < 1000 {
            continue 'processing;
        }

        buffer.save("test_render.png").expect("Saving file failed");

        break;
    }
}
