use dcc_lsystem::{LSystemBuilder, variable, constant, ArenaId};

use image::{Rgb, ImageBuffer};
use imageproc::drawing::draw_convex_polygon_mut;
use imageproc::drawing::Point;
use std::cmp::{max, min};
use std::f32::consts::{FRAC_PI_2,FRAC_PI_4};

pub fn fill_mut(buffer: &mut ImageBuffer<Rgb<u8>,Vec<u8>>, color: Rgb<u8>) {
    for pixel in buffer.pixels_mut() {
        *pixel = color;
    }
}

pub fn draw_line_mut(buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x1 : u32, y1 : u32, x2 : u32, y2 : u32, thickness : f32, color: Rgb<u8>) {
    assert!(thickness > 0.0);

    // Compute the angle from the first point to the second point
    let angle = {
        if x1 == x2 {
            FRAC_PI_2
        } else {
            (((y2 - y1) / (x2 - x1)) as f32).atan()
        }
    };

    // Compute the angle of the line perpendicular to the line from the first
    // point to the second point
    let perpendicular_angle = angle + FRAC_PI_2;

    let p1 = Point::new((x1 as f32 + thickness * perpendicular_angle.cos()) as i32,
                        (y1 as f32 + thickness * perpendicular_angle.sin()) as i32);
    let p2 = Point::new((x1 as f32 - thickness * perpendicular_angle.cos()) as i32,
                        (y1 as f32 - thickness * perpendicular_angle.sin()) as i32);
    let p3 = Point::new((x2 as f32 + thickness * perpendicular_angle.cos()) as i32,
                        (y2 as f32 + thickness * perpendicular_angle.sin()) as i32);
    let p4 = Point::new((x2 as f32 - thickness * perpendicular_angle.cos()) as i32,
                        (y2 as f32 - thickness * perpendicular_angle.sin()) as i32);

    // Draw the convex shape
    draw_convex_polygon_mut(
        buffer,
        &[p1,p3,p4,p2],
        color
    );
}


pub fn binary_tree_lsystem_builder() -> (LSystemBuilder, ArenaId, ArenaId, ArenaId, ArenaId) {
    let mut builder = LSystemBuilder::new();

    let zero = variable!(builder, "0");
    let one = variable!(builder, "1");
    let lsb = constant!(builder, "[");
    let rsb = constant!(builder, "]");

    (builder, zero, one, lsb, rsb)
}

struct StackTurtle {
    x : i32,
    y : i32,
    heading : f32,
    lines : Vec<(i32,i32,i32,i32)>,
    stack : Vec<(i32,i32,f32)>,
    max_x : i32,
    max_y : i32,
    min_x : i32,
    min_y : i32,
}

impl StackTurtle {
    pub fn new() -> Self {
        Self {
            x : 0,
            y : 0,
            heading : FRAC_PI_2,
            lines : Vec::new(),
            stack : Vec::new(),
            max_x : 0,
            max_y : 0,
            min_x : 0,
            min_y : 0,
        }
    }

    pub fn push(&mut self) {
        // push our current position and heading onto the stack
        self.stack.push((self.x, self.y, self.heading));
    }

    pub fn pop(&mut self) {
        let (x, y, heading) = self.stack.pop().expect("Called pop on empty stack");

        self.x = x;
        self.y = y;
        self.heading = heading;
    }

    pub fn left(&mut self, angle: f32) {
        self.heading += angle;
    }

    pub fn right(&mut self, angle: f32) {
        self.heading -= angle;
    }

    pub fn forward(&mut self, distance: f32) {
        let x2 = (self.x as f32 + self.heading.cos() * distance) as i32;
        let y2 = (self.y as f32 + self.heading.sin() * distance) as i32;

        self.lines.push((self.x, self.y, x2, y2));

        self.x = x2;
        self.y = y2;

        // update our max and min values
        self.max_x = max(self.x, self.max_x);
        self.max_y = max(self.y, self.max_y);
        self.min_x = min(self.x, self.min_x);
        self.min_y = min(self.y, self.min_y);
    }

    pub fn bounds(&self) -> (u32, u32, i32) {
        ((self.max_x + self.min_x.abs()) as u32,
        (self.max_y + self.min_y.abs()) as u32,
        self.min_x)
    }

    pub fn lines(&self) -> &[(i32, i32, i32, i32)] {
        &self.lines
    }
}

fn main() {
    let (mut builder, zero, one, lsb, rsb) = binary_tree_lsystem_builder();

    // our axiom (i.e. initial condition) is 0
    builder.axiom(vec![zero]);

    // we have two transformation rules: 1 -> 11, and 0 -> 1[0]0
    builder.transformation_rule(one, vec![one, one]);
    builder.transformation_rule(zero, vec![one, lsb, zero, rsb, zero]);

    // build our system and step forward a couple of iterations
    let mut system = builder.finish();
    system.step_by(4);

    // We use our StackTurtle to remember where we should draw each line in our binary tree
    let mut turtle = StackTurtle::new();

    let move_distance = 200.0;
    let angle = FRAC_PI_4;

    for token in system.get_state() {
        if *token == lsb {
            // Push our current position and heading onto the stack, then turn left 45 degrees
            turtle.push();
            turtle.left(angle);
        } else if *token == rsb {
            // Pop our current position and heading off the stack, then turn right 45 degrees
            turtle.pop();
            turtle.right(angle);
        } else {
            // otherwise just move forwards
            turtle.forward(move_distance);
        }
    }

    // Now we want to actually draw the line
    let thickness = 15.5;
    let padding: u32 = 20;

    let (turtle_width, turtle_height, min_x) = turtle.bounds();

    // We add some padding to the width reported by our turtle to make
    // our final image look a little nicer.
    let width = 2 * padding + turtle_width;
    let height = 2 * padding + turtle_height;

    let mut buffer = ImageBuffer::new(width, height);

    // Make the buffer entirely white
    fill_mut(&mut buffer, Rgb([255u8, 255u8, 255u8]));

    // Helper functions for converting between the coordinate system used
    // by the image crate and our coordinate system.  These functions also
    // take care of the padding for us.
    let xp = |x : i32| -> u32 {
        (x - min_x + padding as i32) as u32
    };

    let yp = |y : i32| -> u32 {
        (height as i64 - (y + padding as i32) as i64) as u32
    };

    // Draw the lines
    for (x1, y1, x2, y2) in turtle.lines() {
        draw_line_mut(&mut buffer,
                      xp(*x1),
                      yp(*y1),
                      xp(*x2),
                      yp(*y2),
                      thickness,
                      Rgb([0u8, 0u8, 0u8]));
    }

    buffer.save("fractal_binary_tree.png").expect("Failed to save to output.png");
}