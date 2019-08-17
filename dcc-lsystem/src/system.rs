use std::collections::HashMap;

use crate::arena::{Arena, ArenaId};
use crate::token::Token;

/// An L-system.
///
/// # Basic Usage
///
/// The suggested method for constructing an `LSystem` is to use a [`LSystemBuilder`], as shown in our implementation
/// of Lindenmayer's system below.
///
/// ```rust
/// use dcc_lsystem::LSystemBuilder;
///
/// let mut builder = LSystemBuilder::new();
///
/// // Set up our two tokens
/// let a = builder.token("A");
/// let b = builder.token("B");
///
/// // Set the axiom
/// builder.axiom(vec![a]);
///
/// // Set the transformation rules
/// builder.transformation_rule(a, vec![a, b]);
/// builder.transformation_rule(b, vec![a]);
///
/// // Build our L-system
/// let mut system = builder.finish();
/// ```
///
/// Once the `LSystem` has been built, you can use the `step()` and `step_by()` methods
/// to iterate the system.
///
/// ```rust,no_run
/// use dcc_lsystem::LSystemBuilder;
///
/// let mut builder = LSystemBuilder::new();
///
/// /* <---- snip ----> */
///
/// let mut system = builder.finish();
///
/// // The initial state of our system
/// assert_eq!(system.render(), "A");
///
/// system.step();
///
/// assert_eq!(system.render(), "AB");
///
/// system.step();
///
/// assert_eq!(system.render(), "ABA");
///
/// system.step_by(5);
///
/// // The state after 7 iterations
/// assert_eq!(system.render(), "ABAABABAABAABABAABABAABAABABAABAAB");
/// ```
///
/// [`LSystemBuilder`]: builder/struct.LSystemBuilder.html
#[derive(Clone, Debug)]
pub struct LSystem {
    arena: Arena<Token>,
    axiom: Vec<ArenaId>,
    rules_map: HashMap<ArenaId, Vec<ArenaId>>,
    state: Vec<ArenaId>,
    steps: usize,
}

impl LSystem {
    /// Create a new instance of `LSystem`.  In general you should avoid using this
    /// and use an `LSystemBuilder` instead.
    pub fn new(
        arena: Arena<Token>,
        axiom: Vec<ArenaId>,
        rules_map: HashMap<ArenaId, Vec<ArenaId>>,
    ) -> Self {
        Self {
            arena,
            axiom: axiom.clone(),
            rules_map,
            state: axiom,
            steps: 0,
        }
    }

    /// Reset the system to its initial state.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::LSystemBuilder;
    ///
    /// let mut builder = LSystemBuilder::new();
    ///
    /// //  Create a simple L-System with one variable `A` and production rule `A -> AA`
    /// let a = builder.token("A");
    /// builder.axiom(vec![a]);
    /// builder.transformation_rule(a, vec![a, a]);
    /// let mut system = builder.finish();
    ///
    /// // Do some work with the system
    /// system.step_by(3);
    /// assert_eq!(system.render(), "AAAAAAAA");
    ///
    /// // Reset the system back to its axiom
    /// system.reset();
    /// assert_eq!(system.render(), "A");
    /// ```
    pub fn reset(&mut self) {
        self.state = self.axiom.clone();
        self.steps = 0;
    }

    /// Iterate the system a single step.
    pub fn step(&mut self) {
        let mut next_state = Vec::new();

        for id in self.state.iter() {
            next_state.extend(self.rules_map[id].clone());
        }

        self.state = next_state;
        self.steps += 1;
    }

    /// Iterate the system by `n` steps.
    pub fn step_by(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Returns the number of iterations the system has undergone so far
    pub fn steps(&self) -> usize {
        self.steps
    }

    /// Returns the current state of the system as a `String`.
    pub fn render(&self) -> String {
        self.state
            .iter()
            .map(|id| self.arena.get(*id).unwrap().name().clone())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Returns the current state of the system.
    pub fn get_state(&self) -> &[ArenaId] {
        &self.state
    }
}
