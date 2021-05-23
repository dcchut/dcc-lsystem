//! Contains the main [`LSystem`] struct for working with Lindenmayer systems.
//!
//! # Basic Usage
//!
//! The suggested method for constructing an [`LSystem`] is to use a [`LSystemBuilder`](dcc_lsystem::LSystemBuilder), as shown in our implementation
//! of Lindenmayer's system below.
//!
//! ```rust
//! # use dcc_lsystem::LSystemError;
//! # fn main() -> Result<(), LSystemError> {
//! use dcc_lsystem::LSystemBuilder;
//!
//! let mut builder = LSystemBuilder::new();
//!
//! // Set up our two tokens
//! let a = builder.token("A")?;
//! let b = builder.token("B")?;
//!
//! // Set the axiom
//! builder.axiom(vec![a])?;
//!
//! // Set the transformation rules
//! builder.transformation_rule(a, vec![a, b])?;
//! builder.transformation_rule(b, vec![a])?;
//!
//! // Build our L-system
//! let mut system = builder.finish()?;
//! # Ok(())
//! # }
//! ```
//!
//! Once the [`LSystem`] has been built, you can use the [`LSystem::step()`] and [`LSystem::step_by()`] methods
//! to iterate the system.
//!
//! ```rust,no_run
//! # use dcc_lsystem::LSystemError;
//! # fn main() -> Result<(), LSystemError> {
//! use dcc_lsystem::LSystemBuilder;
//!
//! let mut builder = LSystemBuilder::new();
//!
//! /* <---- snip ----> */
//!
//! let mut system = builder.finish()?;
//!
//! // The initial state of our system
//! assert_eq!(system.render(), "A");
//!
//! system.step();
//!
//! assert_eq!(system.render(), "AB");
//!
//! system.step();
//!
//! assert_eq!(system.render(), "ABA");
//!
//! system.step_by(5);
//!
//! // The state after 7 iterations
//! assert_eq!(system.render(), "ABAABABAABAABABAABABAABAABABAABAAB");
//! # Ok(())
//! # }
//! ```
use std::collections::HashMap;

use crate::arena::{Arena, ArenaId};
use crate::token::Token;

/// Main struct for working with Lindenmayer systems.
#[derive(Clone, Debug)]
pub struct LSystem {
    arena: Arena<Token>,
    axiom: Vec<ArenaId>,
    rules_map: HashMap<ArenaId, Vec<ArenaId>>,
    state: Vec<ArenaId>,
    steps: usize,
}

impl LSystem {
    /// Create a new instance of [`LSystem`].  In general you should avoid using this method directly
    /// and use a [`LSystemBuilder`](dcc_lsystem::LSystemBuilder) instead.
    pub(crate) fn new(
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
    /// # use dcc_lsystem::LSystemError;
    /// # fn main() -> Result<(), LSystemError> {
    /// use dcc_lsystem::LSystemBuilder;
    ///
    /// let mut builder = LSystemBuilder::new();
    ///
    /// //  Create a simple L-System with one variable `A` and production rule `A -> AA`
    /// let a = builder.token("A")?;
    /// builder.axiom(vec![a])?;
    /// builder.transformation_rule(a, vec![a, a])?;
    /// let mut system = builder.finish()?;
    ///
    /// // Do some work with the system
    /// system.step_by(3);
    /// assert_eq!(system.render(), "AAAAAAAA");
    ///
    /// // Reset the system back to its axiom
    /// system.reset();
    /// assert_eq!(system.render(), "A");
    /// # Ok(())
    /// # }
    /// ```
    pub fn reset(&mut self) {
        self.state = self.axiom.clone();
        self.steps = 0;
    }

    /// Iterate the system a single step.
    ///
    /// # Example
    /// ```rust
    /// # use dcc_lsystem::{LSystemError, LSystemBuilder};
    /// # fn main() -> Result<(), LSystemError> {
    /// # let mut builder = LSystemBuilder::new();
    /// # let a = builder.token("a")?;
    /// # let b = builder.token("b")?;
    /// # builder.axiom(vec![a])?;
    /// # builder.transformation_rule(a,vec![a, b, b, a])?;
    /// # let mut system = builder.finish()?;
    /// // `system` is an LSystem with axiom `a` and transformation rule `a -> abba`.
    /// assert_eq!(system.render(), "a");
    ///
    /// // Iterate the system a single time
    /// system.step();
    /// assert_eq!(system.render(), "abba");
    ///
    /// // Once more for good luck
    /// system.step();
    /// assert_eq!(system.render(), "abbabbabba");
    /// # Ok(())
    /// # }
    /// ```
    pub fn step(&mut self) {
        let mut next_state = Vec::new();

        for id in self.state.iter() {
            next_state.extend(self.rules_map[id].clone());
        }

        self.state = next_state;
        self.steps += 1;
    }

    /// Iterate the system by `n` steps.
    ///
    /// # Example
    /// ```rust
    /// # use dcc_lsystem::{LSystemError, LSystemBuilder};
    /// # fn main() -> Result<(), LSystemError> {
    /// # let mut builder = LSystemBuilder::new();
    /// # let a = builder.token("a")?;
    /// # let b = builder.token("b")?;
    /// # builder.axiom(vec![a])?;
    /// # builder.transformation_rule(a,vec![b, a])?;
    /// # let mut system = builder.finish()?;
    /// // `system` is an LSystem with axiom `a` and transformation rule `a -> ba`.
    /// assert_eq!(system.render(), "a");
    ///
    /// // Iterate the system three times
    /// system.step_by(3);
    /// assert_eq!(system.render(), "bbba");
    /// # Ok(())
    /// # }
    /// ```
    pub fn step_by(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Returns the number of iterations the system has undergone so far
    ///
    /// # Example
    /// ```rust
    /// # use dcc_lsystem::{LSystemError, LSystemBuilder};
    /// # fn main() -> Result<(), LSystemError> {
    /// # let mut builder = LSystemBuilder::new();
    /// # let a = builder.token("a")?;
    /// # let b = builder.token("b")?;
    /// # builder.axiom(vec![a])?;
    /// # builder.transformation_rule(a,vec![b, a, b])?;
    /// # let mut system = builder.finish()?;
    /// // `system` is an LSystem with axiom `a` and transformation rule `a -> bab`.
    /// assert_eq!(system.steps(), 0);
    /// assert_eq!(system.render(), "a");
    ///
    /// // Iterate the system 100 times
    /// system.step_by(100);
    /// assert_eq!(system.steps(), 100);
    /// assert_eq!(system.render().len(), 201);
    /// # Ok(())
    /// # }
    /// ```
    pub fn steps(&self) -> usize {
        self.steps
    }

    /// Returns the current state of the system as a [`String`].
    ///
    /// # Example
    /// ```rust
    /// # use dcc_lsystem::{LSystemError, LSystemBuilder};
    /// # fn main() -> Result<(), LSystemError> {
    /// # let mut builder = LSystemBuilder::new();
    /// # let a = builder.token("a")?;
    /// # let b = builder.token("b")?;
    /// # builder.axiom(vec![a])?;
    /// # builder.transformation_rule(a,vec![a, a])?;
    /// # let mut system = builder.finish()?;
    /// // `system` is an LSystem with axiom `a` and transformation rule `a -> aa`.
    /// assert_eq!(system.render(), "a");
    ///
    /// system.step();
    /// assert_eq!(system.render(), "aa");
    ///
    /// system.step();
    /// assert_eq!(system.render(), "aaaa");
    /// # Ok(())
    /// # }
    /// ```
    pub fn render(&self) -> String {
        self.state
            .iter()
            // unwrap: the only way to obtain an LSystem is through one of the builders,
            //         which verify that all indexes are valid.
            .map(|id| self.arena.get(*id).unwrap().name())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Returns a slice consisting of the [`ArenaId`]'s of the tokens currently in the system.
    ///
    /// # Example
    /// ```rust
    /// # use dcc_lsystem::{LSystemError, LSystemBuilder};
    /// # fn main() -> Result<(), LSystemError> {
    /// # let mut builder = LSystemBuilder::new();
    /// # let a = builder.token("a")?;
    /// # let b = builder.token("b")?;
    /// # builder.axiom(vec![a])?;
    /// # builder.transformation_rule(a,vec![a, a, b])?;
    /// # let mut system = builder.finish()?;
    /// // `system` is an LSystem with axiom `a` and transformation rule `a -> aab`.
    /// assert_eq!(system.get_state(), &[a]);
    /// system.step();
    /// assert_eq!(system.get_state(), &[a, a, b]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    pub fn get_state(&self) -> &[ArenaId] {
        &self.state
    }
}
