use std::collections::HashMap;

use rand::Rng;
use regex::Regex;

use dcc_lsystem_derive::TurtleContainer;
use lazy_static::lazy_static;

use crate::renderer::TurtleRenderer;
use crate::{ArenaId, LSystem, LSystemBuilder};
use std::f64::consts::FRAC_PI_2;

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
/// # Example
/// The following `DumbTurtle` only moves to the right.
///
/// ```rust
/// use dcc_lsystem::turtle::{BaseTurtle, MovingTurtle};
///
/// struct DumbTurtle {
///     inner: BaseTurtle,
/// }
///
/// impl MovingTurtle for DumbTurtle {
///     type Item = i32;
///
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
///
pub trait MovingTurtle {
    type Item;

    /// Returns a reference to the wrapped `BaseTurtle`.
    fn inner(&self) -> &BaseTurtle;

    /// Returns a mutable reference to the wrapped `BaseTurtle`.
    fn inner_mut(&mut self) -> &mut BaseTurtle;

    /// Moves the turtle forward by `distance`.
    fn forward(&mut self, distance: Self::Item);
}

/// This trait indicates that the implementor contains a turtle for us to play with.
///
/// It's a bit annoying to have to implement this trait everywhere, so using the `dcc-lsystem-derive`
/// crate you can do the following:
///
/// ```rust
/// use dcc_lsystem::turtle::{SimpleTurtle, TurtleContainer};
/// use dcc_lsystem_derive::TurtleContainer;
///
/// #[derive(TurtleContainer)]
/// struct BasicContainer {
///     #[turtle]
///     inner : SimpleTurtle,
///
///     /* <----- some other fields ----- */
/// }
/// ```
///
/// which is roughly equivalent to the following:
///
/// ```rust
/// use dcc_lsystem::turtle::{SimpleTurtle, TurtleContainer, MovingTurtle};
///
/// struct BasicContainer {
///     inner : SimpleTurtle,
///
///     /* <----- some other fields ----- */
/// }
///
/// impl TurtleContainer for BasicContainer {
///     type Item = <SimpleTurtle as MovingTurtle>::Item;
///
///     fn inner(&self) -> &MovingTurtle<Item = Self::Item> {
///         &self.inner
///     }
/// }
///```
pub trait TurtleContainer {
    type Item;

    fn inner(&self) -> &dyn MovingTurtle<Item = Self::Item>;
}

/// Every turtle contains a turtle.
impl<T> TurtleContainer for dyn MovingTurtle<Item = T> {
    type Item = T;

    fn inner(&self) -> &dyn MovingTurtle<Item = Self::Item> {
        self
    }
}

pub trait Stack: MovingTurtle {
    /// Push the current state of this turtle onto a stack.
    fn push(&mut self);

    /// Pop the current state of this turtle onto a stack.
    fn pop(&mut self);
}

#[derive(Clone, Debug)]
pub struct BaseTurtle {
    x: f64,
    y: f64,
    lines: Vec<(f64, f64, f64, f64)>,
    max_x: f64,
    max_y: f64,
    min_x: f64,
    min_y: f64,
    pen_down: bool,
}

impl BaseTurtle {
    /// Creates a new `BaseTurtle` instance.
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            lines: Vec::new(),
            max_x: 0.0,
            max_y: 0.0,
            min_x: 0.0,
            min_y: 0.0,
            pen_down: true,
        }
    }

    /// Returns the current `x` coordinate of the turtle.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the current `y` coordinate of the turtle.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns a slice containing all the lines `(x1, y1, x2, y2)` traversed by the turtle.
    pub fn lines(&self) -> &[(f64, f64, f64, f64)] {
        &self.lines
    }

    /// Set the current position of this turtle to `(x,y)`.
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.update_bounds();
    }

    fn update_bounds(&mut self) {
        self.min_x = self.min_x.min(self.x);
        self.min_y = self.min_y.min(self.y);
        self.max_x = self.max_x.max(self.x);
        self.max_y = self.max_y.max(self.y);
    }

    /// Moves the turtle by `(dx,dy)`.
    pub fn delta_move(&mut self, dx: f64, dy: f64) {
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
            self.min_x as i32,
            self.min_y as i32,
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

/// Represents the cardinal directions.
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

    pub fn dx(self) -> i32 {
        match self {
            Heading::West => -1,
            Heading::East => 1,
            _ => 0,
        }
    }

    pub fn dy(self) -> i32 {
        match self {
            Heading::North => 1,
            Heading::South => -1,
            _ => 0,
        }
    }
}

/// A simple turtle implementation.  It has the following features:
///
/// * You can change direction! (`turtle.set_heading(f32)`, `turtle.left(f32)`, and `turtle.right(f32)`)
/// * You can make it move! (`turtle.forward(i32)`)
/// * Stacks! (`turtle.push()` and `turtle.pop()`)
///
/// and some other features.
#[derive(Clone, Debug)]
pub struct SimpleTurtle {
    turtle: BaseTurtle,
    heading: f64,
    stack: Vec<(f64, f64, f64)>,
    pen_down: bool,
}

impl SimpleTurtle {
    /// Return a new `StackTurtle` instance.
    pub fn new() -> Self {
        Self {
            turtle: BaseTurtle::new(),
            heading: FRAC_PI_2,
            stack: Vec::new(),
            pen_down: true,
        }
    }

    /// Turns the turtle left by the given angle (in radians).
    pub fn left(&mut self, angle: f64) {
        self.heading += angle;
    }

    /// Turns the turtle right by the given angle (in radians).
    pub fn right(&mut self, angle: f64) {
        self.heading -= angle;
    }

    /// Set the current heading of the turtle (in radians).
    pub fn set_heading(&mut self, heading: f64) {
        self.heading = heading;
    }
}

impl Stack for SimpleTurtle {
    /// Pushes the current position and heading of the turtle onto the stack.
    fn push(&mut self) {
        self.stack
            .push((self.turtle.x(), self.turtle.y(), self.heading));
    }

    /// Pops the position and heading off the stack.
    fn pop(&mut self) {
        let (x, y, heading) = self.stack.pop().expect("Called pop on empty stack");

        self.turtle.set_position(x, y);
        self.heading = heading;
    }
}

impl MovingTurtle for SimpleTurtle {
    type Item = i32;

    fn inner(&self) -> &BaseTurtle {
        &self.turtle
    }

    fn inner_mut(&mut self) -> &mut BaseTurtle {
        &mut self.turtle
    }

    fn forward(&mut self, distance: i32) {
        let dx = self.heading.cos() * (distance as f64);
        let dy = self.heading.sin() * (distance as f64);

        if self.pen_down {
            self.turtle.delta_move(dx, dy);
        }
    }
}

impl Default for SimpleTurtle {
    fn default() -> Self {
        Self::new()
    }
}

/// The state modified by a `TurtleLSystemRenderer`.  Each `TurtleAction` corresponds
/// to a modifier of the form `Fn(&mut TurtleLSystemState)`.
#[derive(TurtleContainer)]
pub struct TurtleLSystemState {
    angle: i32,
    angle_stack: Vec<i32>,

    #[turtle]
    turtle: SimpleTurtle,
}

impl TurtleLSystemState {
    /// Create a new state.
    pub fn new() -> Self {
        Self {
            angle: 0,
            angle_stack: Vec::new(),
            turtle: SimpleTurtle::new(),
        }
    }
}

impl Default for TurtleLSystemState {
    fn default() -> Self {
        Self::new()
    }
}

/// A `TurtleLSystemBuilder` is used to generate an L-system and a turtle
/// based renderer based don this L-system.
#[derive(Clone)]
pub struct TurtleLSystemBuilder {
    builder: LSystemBuilder,
    actions: HashMap<ArenaId, TurtleAction>,
    tokens: HashMap<String, ArenaId>,
    global_rotate: i32,
}

impl TurtleLSystemBuilder {
    /// Create a new `TurtleLSystemBuilder` instance.
    pub fn new() -> Self {
        Self {
            builder: LSystemBuilder::new(),
            actions: HashMap::new(),
            tokens: HashMap::new(),
            global_rotate: 0,
        }
    }

    /// Apply a global rotation to the builder.  This is useful for modifying the orientation
    /// of the data passed to a `Renderer`.
    pub fn rotate(&mut self, angle: i32) -> &mut Self {
        self.global_rotate = angle;

        self
    }

    /// Associate a token and corresponding action to this builder.
    pub fn token<S: Into<String>>(&mut self, token: S, action: TurtleAction) -> &mut Self {
        let ident = token.into();

        let token = self.builder.token(ident.clone());

        self.tokens.insert(ident, token);
        self.actions.insert(token, action);

        self
    }

    /// Set the axiom  for this builder.
    pub fn axiom(&mut self, ident: &str) -> &mut Self {
        let mut axiom = Vec::new();

        for part in ident.split_whitespace() {
            let token = self.get_token(part).expect("Invalid axiom");

            axiom.push(token);
        }

        assert_ne!(axiom.len(), 0);

        self.builder.axiom(axiom);

        self
    }

    fn get_token(&self, token: &str) -> Option<ArenaId> {
        self.tokens.get(token).cloned()
    }

    /// Add a transformation rule to the builder.
    pub fn rule<'a, S: Into<&'a str>>(&mut self, rule: S) -> &mut Self {
        let rule = rule.into();

        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s*(\w)\s*=>\s*((?:\s*\S+\s*)*)\s*").unwrap();
        }

        let cap = RE.captures(rule).expect("Invalid rule");

        // The LHS of our rule
        let lhs = self
            .get_token(&cap[1])
            .unwrap_or_else(|| panic!("Invalid token: {}", &cap[1]));

        // Construct the RHS of our rule
        let mut rule = Vec::new();

        for token in cap[2].split_whitespace() {
            let token = self
                .get_token(token)
                .unwrap_or_else(|| panic!("Invalid token: {}", token));

            rule.push(token);
        }

        // Add the rule to our builder
        self.builder.transformation_rule(lhs, rule);

        self
    }

    /// Consumes the builder, returning the generated `LSystem` and a `Renderer`
    /// which can associate tokens in the `LSystem` to turtle actions.
    pub fn finish(self) -> (LSystem, TurtleRenderer<TurtleLSystemState>) {
        let mut renderer = TurtleRenderer::new(TurtleLSystemState::new());

        // Register the processing functions for each action
        for (id, action) in self.actions.into_iter() {
            match action {
                TurtleAction::Push => {
                    renderer.register(id, |state| {
                        state.turtle.push();
                        state.angle_stack.push(state.angle);
                    });
                }
                TurtleAction::Pop => {
                    renderer.register(id, |state| {
                        state.turtle.pop();
                        state.angle = state.angle_stack.pop().expect("Popped with empty stack");
                    });
                }
                TurtleAction::Forward(distance) => {
                    let current_global_rotate = self.global_rotate;

                    renderer.register(id, move |state| {
                        state.turtle.set_heading(
                            ((current_global_rotate + state.angle) as f64).to_radians(),
                        );
                        state.turtle.forward(distance);
                    });
                }
                TurtleAction::Rotate(angle) => {
                    renderer.register(id, move |state| {
                        state.angle = (state.angle + angle) % 360;
                    });
                }
                TurtleAction::StochasticRotate(distribution) => {
                    renderer.register(id, move |state| {
                        state.angle = (state.angle + distribution.sample()) % 360;
                    });
                }
                TurtleAction::StochasticForward(distribution) => {
                    let current_global_rotate = self.global_rotate;

                    renderer.register(id, move |state| {
                        state.turtle.set_heading(
                            ((current_global_rotate + state.angle) as f64).to_radians(),
                        );
                        state.turtle.forward(distribution.sample());
                    });
                }
                TurtleAction::Nothing => {}
            }
        }

        (self.builder.finish(), renderer)
    }
}

impl Default for TurtleLSystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// An integer-valued probability distribution.
///
/// We need to be able to clone Box<dyn Distribution>, so we use the wonderful
/// `dyn_clone` crate to allow for this.
pub trait Distribution: dyn_clone::DynClone {
    /// Take a sample from this distribution.
    fn sample(&self) -> i32;
}

dyn_clone::clone_trait_object!(Distribution);

/// A uniform distribution on a closed interval.
#[derive(Clone)]
pub struct Uniform {
    lower: i32,
    upper: i32,
}

impl Uniform {
    /// Creates a new uniform distribution on the interval [lower, upper].
    ///
    /// # Panics
    /// Will panic is `lower` > `upper`
    pub fn new(lower: i32, upper: i32) -> Self {
        assert!(lower <= upper);
        Self { lower, upper }
    }
}

impl Distribution for Uniform {
    fn sample(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.lower..=self.upper)
    }
}

/// A constant distribution
impl Distribution for i32 {
    fn sample(&self) -> i32 {
        *self
    }
}

/// The possible actions we can associate to tokens in our `LSystem`.
#[derive(Clone)]
pub enum TurtleAction {
    Nothing,
    Rotate(i32),
    Forward(i32),
    StochasticRotate(Box<dyn Distribution>),
    StochasticForward(Box<dyn Distribution>),
    Push,
    Pop,
}
