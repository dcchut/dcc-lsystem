use std::cmp::{max, min};
use std::f32::consts::FRAC_PI_2;

#[derive(Clone, Debug)]
pub struct StackTurtle {
    x: i32,
    y: i32,
    heading: f32,
    lines: Vec<(i32, i32, i32, i32)>,
    stack: Vec<(i32, i32, f32)>,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}

impl StackTurtle {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            heading: FRAC_PI_2,
            lines: Vec::new(),
            stack: Vec::new(),
            max_x: 0,
            max_y: 0,
            min_x: 0,
            min_y: 0,
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
        (
            (self.max_x + self.min_x.abs()) as u32,
            (self.max_y + self.min_y.abs()) as u32,
            self.min_x,
        )
    }

    pub fn lines(&self) -> &[(i32, i32, i32, i32)] {
        &self.lines
    }
}

impl Default for StackTurtle {
    fn default() -> Self {
        Self::new()
    }
}
