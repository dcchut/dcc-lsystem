//! Contains a collection of turtles which can be used to interpret the state of an LSystem
//! as a rendering.
use std::collections::HashMap;

use rand::Rng;
use regex::Regex;

use dcc_lsystem_derive::TurtleContainer;
use lazy_static::lazy_static;

use crate::renderer::TurtleRenderer;
use crate::{ArenaId, LSystem, LSystemBuilder, LSystemError};
use std::f64::consts::FRAC_PI_2;

/// A simple Turtle trait.
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
///     type Item = f64;
///
///     fn inner(&self) -> &BaseTurtle {
///         &self.inner
///     }
///
///     fn inner_mut(&mut self) -> &mut BaseTurtle {
///         &mut self.inner
///     }
///
///     fn forward(&mut self, distance: f64) {
///         self.inner_mut().delta_move(distance, 0.0);
///     }
/// }
/// ```
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
///     inner: SimpleTurtle,
///     /* <----- some other fields ----- */
/// }
/// ```
///
/// which is roughly equivalent to the following:
///
/// ```rust
/// use dcc_lsystem::turtle::{MovingTurtle, SimpleTurtle, TurtleContainer};
///
/// struct BasicContainer {
///     inner: SimpleTurtle,
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
/// ```
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

/// The basic work horse-turtle.  Keeps track of where it is, where it's been, and
/// whether the pen that our turtle is wielding is down.
///
/// # Example
/// ```rust
/// use dcc_lsystem::turtle::BaseTurtle;
///
/// let mut turtle = BaseTurtle::new();
/// assert_eq!(turtle.x(), 0.0);
/// assert_eq!(turtle.y(), 0.0);
///
/// // Move the turtle to (1.0, 1.0)
/// turtle.delta_move(1.0, 1.0);
/// assert_eq!(turtle.x(), 1.0);
/// assert_eq!(turtle.y(), 1.0);
///
/// // The turtle should have a line from (0., 0.) to (1., 1.)
/// assert_eq!(turtle.lines(), &[(0., 0., 1., 1.)]);
///
/// // Lifting the pen up means moving won't cause an additional line to be created.
/// turtle.pen_up();
/// turtle.delta_move(1.0, 0.0);
/// assert_eq!(turtle.x(), 2.0);
/// assert_eq!(turtle.y(), 1.0);
/// assert_eq!(turtle.lines().len(), 1);
/// ```
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
    /// Creates a new [`BaseTurtle`] instance.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::BaseTurtle;
    ///
    /// let turtle = BaseTurtle::new();
    /// assert_eq!(turtle.x(), 0.0);
    /// assert_eq!(turtle.y(), 0.0);
    /// ```
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
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::BaseTurtle;
    ///
    /// let mut turtle = BaseTurtle::new();
    /// assert_eq!(turtle.x(), 0.0);
    ///
    /// turtle.delta_move(-15.4, 0.0);
    /// assert_eq!(turtle.x(), -15.4);
    /// ```
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the current `y` coordinate of the turtle.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::BaseTurtle;
    ///
    /// let mut turtle = BaseTurtle::new();
    /// assert_eq!(turtle.y(), 0.0);
    ///
    /// turtle.delta_move(0.0, 14.0);
    /// assert_eq!(turtle.y(), 14.0);
    /// ```
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns a slice containing all the lines `(x1, y1, x2, y2)` traversed by the turtle.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::BaseTurtle;
    ///
    /// let mut turtle = BaseTurtle::new();
    /// assert!(turtle.lines().is_empty());
    ///
    /// turtle.delta_move(5.0, -5.0);
    /// turtle.delta_move(1.0, 1.0);
    ///
    /// assert_eq!(turtle.lines(), &[(0., 0., 5., -5.), (5., -5., 6., -4.)]);
    /// ```
    pub fn lines(&self) -> &[(f64, f64, f64, f64)] {
        &self.lines
    }

    /// Set the current position of this turtle to `(x,y)`.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::BaseTurtle;
    ///
    /// let mut turtle = BaseTurtle::new();
    /// turtle.set_position(99.0, 200.0);
    ///
    /// assert_eq!(turtle.x(), 99.0);
    /// assert_eq!(turtle.y(), 200.0);
    /// ```
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
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::BaseTurtle;
    ///
    /// // Turtle is initially at (0., 0.)
    /// let mut turtle = BaseTurtle::new();
    ///
    /// turtle.delta_move(5.0, 5.0);
    /// assert_eq!(turtle.x(), 5.0);
    /// assert_eq!(turtle.y(), 5.0);
    ///
    /// turtle.delta_move(2.0, -8.0);
    /// assert_eq!(turtle.x(), 7.0);
    /// assert_eq!(turtle.y(), -3.0);
    /// ```
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
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::BaseTurtle;
    ///
    /// let mut turtle = BaseTurtle::new();
    /// assert_eq!(turtle.bounds(), (0., 0., 0., 0.));
    ///
    /// turtle.set_position(5.0, 5.0);
    /// turtle.set_position(-4.0, -3.0);
    ///
    /// assert_eq!(turtle.bounds(), (9.0, 8.0, -4.0, -3.0));
    /// ```
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        (
            (self.max_x + self.min_x.abs()),
            (self.max_y + self.min_y.abs()),
            self.min_x,
            self.min_y,
        )
    }

    /// Puts the turtles pen down.  While the pen is down the turtle will draw a line
    /// when it moves.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::BaseTurtle;
    ///
    /// let mut turtle = BaseTurtle::new();
    /// turtle.pen_down();
    ///
    /// // Moving the turtle causes a line to be drawn
    /// turtle.delta_move(3.0, -4.0);
    /// assert_eq!(turtle.lines(), &[(0., 0., 3.0, -4.0)]);
    /// ```
    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    /// Pulls the turtles pen up.  While the pen is up the turtle will not draw lines
    /// when it moves.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::BaseTurtle;
    ///
    /// let mut turtle = BaseTurtle::new();
    /// turtle.pen_up();
    ///
    /// // Moving the turtle with the pen up doesn't draw a line
    /// turtle.delta_move(3.0, -4.0);
    /// assert!(turtle.lines().is_empty());
    /// ```
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
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::Heading;
    ///
    /// let heading = Heading::North;
    /// assert_eq!(heading.left(), Heading::West);
    /// ```
    pub fn left(self) -> Self {
        match self {
            Heading::North => Heading::West,
            Heading::West => Heading::South,
            Heading::South => Heading::East,
            Heading::East => Heading::North,
        }
    }

    /// Returns the `Heading` that is 90 degrees right of this one.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::Heading;
    ///
    /// let heading = Heading::North;
    /// assert_eq!(heading.right(), Heading::East);
    /// ```
    pub fn right(self) -> Self {
        // Don't judge me...
        self.left().left().left()
    }

    /// Returns a horizontal shift (-1, 0, or 1) based on the current heading.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::Heading;
    ///
    /// assert_eq!(Heading::East.dx(), 1);
    /// assert_eq!(Heading::West.dx(), -1);
    /// assert_eq!(Heading::North.dx(), 0);
    /// assert_eq!(Heading::South.dx(), 0);
    /// ```
    pub fn dx(self) -> i32 {
        match self {
            Heading::West => -1,
            Heading::East => 1,
            _ => 0,
        }
    }

    /// Returns a vertical shift (-1, 0, or 1) base on the current heading.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::turtle::Heading;
    ///
    /// assert_eq!(Heading::North.dy(), 1);
    /// assert_eq!(Heading::South.dy(), -1);
    /// assert_eq!(Heading::East.dy(), 0);
    /// assert_eq!(Heading::West.dy(), 0);
    /// ```
    pub fn dy(self) -> i32 {
        match self {
            Heading::North => 1,
            Heading::South => -1,
            _ => 0,
        }
    }
}

/// A simple turtle implementation.
///
/// * You can change direction! (see [`SimpleTurtle::set_heading`], [`SimpleTurtle::left`], and  [`SimpleTurtle::right`])
/// * You can make it move! (see [`SimpleTurtle::forward`])
/// * Stacks! (see [`SimpleTurtle::push`] and [`SimpleTurtle::pop`])
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

    /// Pops the position and heading off the stack.  If the stack is empty
    /// then popping will do nothing.
    fn pop(&mut self) {
        if let Some((x, y, heading)) = self.stack.pop() {
            self.turtle.set_position(x, y);
            self.heading = heading;
        }
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
    pub fn token<S: Into<String>>(
        &mut self,
        token: S,
        action: TurtleAction,
    ) -> Result<&mut Self, LSystemError> {
        let ident = token.into();

        let token = self.builder.token(ident.clone())?;

        self.tokens.insert(ident, token);
        self.actions.insert(token, action);

        Ok(self)
    }

    /// Set the axiom  for this builder.
    pub fn axiom(&mut self, ident: &str) -> Result<&mut Self, LSystemError> {
        let mut axiom = Vec::new();

        for part in ident.split_whitespace() {
            let token = self.get_token(part)?;

            axiom.push(token);
        }

        assert_ne!(axiom.len(), 0);

        self.builder.axiom(axiom)?;

        Ok(self)
    }

    fn get_token(&self, token: &str) -> Result<ArenaId, LSystemError> {
        self.tokens
            .get(token)
            .cloned()
            .ok_or_else(|| LSystemError::UnknownToken(token.to_string()))
    }

    /// Add a transformation rule to the builder.
    pub fn rule<'a, S: Into<&'a str>>(&mut self, rule: S) -> Result<&mut Self, LSystemError> {
        let rule = rule.into();

        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s*(\w)\s*=>\s*((?:\s*\S+\s*)*)\s*").unwrap();
        }

        let cap = RE
            .captures(rule)
            .ok_or_else(|| LSystemError::InvalidRule(rule.to_string()))?;

        // The LHS of our rule
        let lhs = self.get_token(&cap[1])?;

        // Construct the RHS of our rule
        let mut rule = Vec::new();

        for token in cap[2].split_whitespace() {
            let token = self.get_token(token)?;

            rule.push(token);
        }

        // Add the rule to our builder
        self.builder.transformation_rule(lhs, rule)?;

        Ok(self)
    }

    /// Consumes the builder, returning the generated `LSystem` and a `Renderer`
    /// which can associate tokens in the `LSystem` to turtle actions.
    pub fn finish(self) -> Result<(LSystem, TurtleRenderer<TurtleLSystemState>), LSystemError> {
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
                        // popping from an empty stack doesn't do anything
                        if let Some(angle) = state.angle_stack.pop() {
                            state.angle = angle;
                        }
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

        Ok((self.builder.finish()?, renderer))
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
