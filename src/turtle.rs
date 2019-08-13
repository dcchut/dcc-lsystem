use std::cmp::{max, min};
use std::f32::consts::FRAC_PI_2;

/// A simple trait for an integer-valued Turtle.
///
/// Any implementation of this trait should contain a `BaseTurtle` struct which
/// is referred to by the `inner` and `inner_mut` methods.  This BaseTurtle deals
/// with storing the turtle's current position, and drawing lines as appropriate.
///
/// The real meat and potatoes of this trait is the `forward` method, which is
/// how someone would actually move your turtle.  Your implementation should be responsible
/// for keeping track of the turtle's heading, and `forward` should move your turtle
/// in that direction (using `self.inner_mut().delta_move(dx, dy)`).
///
/// # Future
///
/// In the future the `Turtle` trait may be modified by the addition of a set_heading()
/// method generic over some `Heading` trait.
///
/// # Example
/// The following `DumbTurtle` only moves to the right.
///
/// ```rust
/// use dcc_lsystem::turtle::{BaseTurtle, Turtle};
///
/// struct DumbTurtle {
///     inner: BaseTurtle,
/// }
///
/// impl Turtle<i32> for DumbTurtle {
///     fn inner(&self) -> &BaseTurtle {
///         &self.inner
///     }
///
///     fn inner_mut(&mut self) -> &mut BaseTurtle {
///         &mut self.inner
///     }
///
///     fn forward(&mut self, distance: i32) {
///         self.inner_mut().delta_move(distance, 0);
///     }
/// }
/// ```
pub trait Turtle<T> {
    /// Returns a reference to the wrapped `BaseTurtle`.
    fn inner(&self) -> &BaseTurtle;

    /// Returns a mutable reference to the wrapped `BaseTurtle`.
    fn inner_mut(&mut self) -> &mut BaseTurtle;

    /// Moves the turtle forward by `distance`.
    fn forward(&mut self, distance: T);
}

#[derive(Clone, Debug)]
pub struct BaseTurtle {
    x: i32,
    y: i32,
    lines: Vec<(i32, i32, i32, i32)>,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
    pen_down: bool,
}

impl BaseTurtle {
    /// Creates a new `BaseTurtle` instance.
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            lines: Vec::new(),
            max_x: 0,
            max_y: 0,
            min_x: 0,
            min_y: 0,
            pen_down: true,
        }
    }

    /// Returns the current `x` coordinate of the turtle.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Returns the current `y` coordinate of the turtle.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Returns a slice containing all the lines `(x1, y1, x2, y2)` traversed by the turtle.
    pub fn lines(&self) -> &[(i32, i32, i32, i32)] {
        &self.lines
    }

    /// Set the current position of this turtle to `(x,y)`.
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.update_bounds();
    }

    fn update_bounds(&mut self) {
        self.min_x = min(self.min_x, self.x);
        self.min_y = min(self.min_y, self.y);
        self.max_x = max(self.max_x, self.x);
        self.max_y = max(self.max_y, self.y);
    }

    /// Moves the turtle by `(dx,dy)`.
    pub fn delta_move(&mut self, dx: i32, dy: i32) {
        let x2 = self.x + dx;
        let y2 = self.y + dy;

        if self.pen_down {
            self.lines.push((self.x, self.y, x2, y2));
        }

        self.x = x2;
        self.y = y2;

        self.update_bounds();
    }

    /// Returns `(total_width, total_height, min_x, min_y)`, where
    /// `total_width` (respectively `total_height) is the largest horizontal (respectively vertical) distance between any two points
    /// that the turtle visited, `min_x` (respectively `min_y`) is the smallest horizontal (respectively vertical) position that
    /// the turtle visited.
    ///
    /// This is useful for converting from turtle coordinates to a new coordinate system starting at `(0,0)`
    /// with width `total_width`, height `total_height`, and all positions have positive `x` and `y` coordinates.
    pub fn bounds(&self) -> (u32, u32, i32, i32) {
        (
            (self.max_x + self.min_x.abs()) as u32,
            (self.max_y + self.min_y.abs()) as u32,
            self.min_x,
            self.min_y,
        )
    }

    /// Puts the turtles pen down.
    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    /// Pulls the turtles pen up.
    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }
}

impl Default for BaseTurtle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Heading {
    North,
    South,
    East,
    West,
}

impl Heading {
    /// Returns the `Heading` that is 90 degrees left of this one.
    pub fn left(self) -> Self {
        match self {
            Heading::North => Heading::West,
            Heading::West => Heading::South,
            Heading::South => Heading::East,
            Heading::East => Heading::North,
        }
    }

    /// Returns the `Heading` that is 90 degrees right of this one.
    pub fn right(self) -> Self {
        // Don't judge me...
        self.left().left().left()
    }
}

#[derive(Clone, Debug)]
pub struct TaxiTurtle {
    turtle: BaseTurtle,
    heading: Heading,
    pen_down: bool,
}

impl TaxiTurtle {
    /// Return a new `TaxiTurtle` instance.
    pub fn new() -> Self {
        Self {
            turtle: BaseTurtle::new(),
            heading: Heading::East,
            pen_down: true,
        }
    }

    /// Makes the turtle turn 90 degrees left of its current heading.
    pub fn left(&mut self) {
        self.heading = self.heading.left();
    }

    /// Makes the turtle turn 90 degrees right of its current heading.
    pub fn right(&mut self) {
        self.heading = self.heading.right();
    }

    /// Set the heading of this turtle.
    pub fn set_heading(&mut self, heading: Heading) {
        self.heading = heading;
    }
}

impl Turtle<i32> for TaxiTurtle {
    fn inner(&self) -> &BaseTurtle {
        &self.turtle
    }

    fn inner_mut(&mut self) -> &mut BaseTurtle {
        &mut self.turtle
    }

    fn forward(&mut self, distance: i32) {
        let dx = match self.heading {
            Heading::East => distance,
            Heading::West => -distance,
            _ => 0,
        };

        let dy = match self.heading {
            Heading::North => distance,
            Heading::South => -distance,
            _ => 0,
        };

        if self.pen_down {
            self.turtle.delta_move(dx, dy);
        }
    }
}

impl Default for TaxiTurtle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct StackTurtle {
    turtle: BaseTurtle,
    heading: f32,
    stack: Vec<(i32, i32, f32)>,
    pen_down: bool,
}

impl StackTurtle {
    /// Return a new `StackTurtle` instance.
    pub fn new() -> Self {
        Self {
            turtle: BaseTurtle::new(),
            heading: FRAC_PI_2,
            stack: Vec::new(),
            pen_down: true,
        }
    }

    /// Pushes the current position and heading of the turtle onto the stack.
    pub fn push(&mut self) {
        self.stack
            .push((self.turtle.x(), self.turtle.y(), self.heading));
    }

    /// Pops the position and heading off the stack.
    pub fn pop(&mut self) {
        let (x, y, heading) = self.stack.pop().expect("Called pop on empty stack");

        self.turtle.set_position(x, y);
        self.heading = heading;
    }

    /// Turns the turtle left by the given angle (in radians).
    pub fn left(&mut self, angle: f32) {
        self.heading += angle;
    }

    /// Turns the turtle right by the given angle (in radians).
    pub fn right(&mut self, angle: f32) {
        self.heading -= angle;
    }

    /// Set the current heading of the turtle (in radians).
    pub fn set_heading(&mut self, heading: f32) {
        self.heading = heading;
    }
}

impl Turtle<i32> for StackTurtle {
    fn inner(&self) -> &BaseTurtle {
        &self.turtle
    }

    fn inner_mut(&mut self) -> &mut BaseTurtle {
        &mut self.turtle
    }

    fn forward(&mut self, distance: i32) {
        let dx = self.heading.cos() * distance as f32;
        let dy = self.heading.sin() * distance as f32;

        if self.pen_down {
            self.turtle.delta_move(dx as i32, dy as i32);
        }
    }
}

impl Default for StackTurtle {
    fn default() -> Self {
        Self::new()
    }
}
