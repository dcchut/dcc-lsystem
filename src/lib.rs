/*!
# dcc-lsystem

A crate for working with [Lindenmayer systems](https://en.wikipedia.org/wiki/L-system).

## Background

An L-System consists of an alphabet of symbols that can be used to make strings,
a collection of production rules that expand each symbol into a larger string of symbols,
an initial axiom string from which to begin construction, and a mechanism for transforming
the generated strings into geometric structures.

## Algae example
Lindenmayer's original L-System for modelling the growth of Algae had
variables `A` and `B`, axiom `A`, and production rules `A -> AB`, `B -> A`.  Iterating
this system produces the following output:

0. `A`
1. `AB`
2. `ABA`
3. `ABAAB`

## Basic usage

Put the following in your `Cargo.toml`:

```toml
dcc-lsystem = "0.1.0"
```

An L-system is represented in this crate by an instance of `LSystem`.  The suggested method for constructing an `LSystem`
is to use a `LSystemBuilder`, as shown in this implementation of Lindenmayer's Algae system:

```rust
use dcc_lsystem::{LSystemBuilder, TokenType};

let mut builder = LSystemBuilder::new();

// Set up our two tokens
let a = builder.token("A", TokenType::Variable);
let b = builder.token("B", TokenType::Variable);

// Set up our axiom
builder.axiom(vec![a]);

// Set the transformation rules
builder.transformation_rule(a, vec![a,b]); // A -> AB
builder.transformation_rule(b, vec![a]);   // B -> A

// Build our LSystem, which should have initial state A
let mut system = builder.finish();
assert_eq!(system.render(), "A");

// system.step() applies our production rules a single time
system.step();
assert_eq!(system.render(), "AB");

system.step();
assert_eq!(system.render(), "ABA");

// system.step_by() applies our production rule a number of times
system.step_by(5);
assert_eq!(system.render(), "ABAABABAABAABABAABABAABAABABAABAAB");
```

### License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
*/

use std::collections::HashMap;

pub mod arena;

pub use arena::{Arena, ArenaId};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenType {
    Variable,
    Constant,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Token {
    name: String,
    token_type: TokenType,
}

impl Token {
    pub fn new<T: Into<String>>(name: T, token_type: TokenType) -> Self {
        Self {
            name: name.into(),
            token_type,
        }
    }

    pub fn is_variable(&self) -> bool {
        match &self.token_type {
            TokenType::Variable => true,
            _ => false,
        }
    }

    pub fn is_constant(&self) -> bool {
        match &self.token_type {
            TokenType::Constant => true,
            _ => false,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone)]
struct TransformationRule {
    predecessor: ArenaId,
    successor: Vec<ArenaId>,
}

impl TransformationRule {
    pub fn new(predecessor: ArenaId, successor: Vec<ArenaId>) -> Self {
        Self {
            predecessor,
            successor,
        }
    }
}

impl Arena<Token> {
    /// Returns a string representation of the given slice of ArenaId's in terms
    /// of the contents of this arena.
    fn render(&self, tokens: &[ArenaId]) -> String {
        assert!(self.is_valid_slice(tokens));

        let mut st = String::new();

        for token in tokens {
            st.push_str(&format!("{}", self.get(*token).unwrap()));
        }

        st
    }
}

#[derive(Default, Clone)]
pub struct LSystemBuilder {
    arena: Arena<Token>,
    axiom: Option<Vec<ArenaId>>,
    rules: Vec<TransformationRule>,
}

impl LSystemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new token.
    ///
    /// Returns a TokenId which can be used (in this LSystem) to refer to the registered token.
    pub fn token<S: Into<String>>(&mut self, name: S, token_type: TokenType) -> ArenaId {
        let token = Token::new(name.into(), token_type);

        self.arena.push(token)
    }

    /// Register a new transformation rule in this LSystem.
    ///
    /// If any of the provided TokenId are invalid, this function will panic.
    pub fn transformation_rule(&mut self, predecessor: ArenaId, successor: Vec<ArenaId>) {
        // Verify that the TokenId corresponds to a token in this LSystem
        if !self.arena.is_valid(predecessor) || !self.arena.is_valid_slice(&successor) {
            panic!("Invalid token id provided to Lsystem::transformation_rule");
        }

        // Add the rule to this system
        self.rules
            .push(TransformationRule::new(predecessor, successor));
    }

    /// Set the axiom for this LSystem.
    pub fn axiom(&mut self, axiom: Vec<ArenaId>) {
        self.axiom = Some(axiom);
    }

    /// Consumes the builder, returning an LSystem instance.
    ///
    /// This function will panic if you have not set an axiom before proceeding.
    pub fn finish(self) -> LSystem {
        let axiom = self.axiom.expect("finish called before axiom set");

        // Construct a HashMap associating each variable with its corresponding transformation rule
        let mut rules_map = HashMap::new();

        for rule in self.rules.into_iter() {
            rules_map.insert(rule.predecessor, rule.successor);
        }

        // We also add constant production rules of the form P => P.
        for (id, token) in self.arena.enumerate() {
            if token.is_constant() {
                rules_map.insert(id, vec![id]);
            }
        }

        // If we set our system up correctly, it should be that each token
        // contributes exactly one rule, so we check for that here.
        assert_eq!(self.arena.len(), rules_map.len());

        LSystem {
            arena: self.arena,
            axiom: axiom.clone(),
            rules_map,
            state: axiom,
        }
    }
}

fn build_rules_string(rules: &[TransformationRule], arena: &Arena<Token>) -> String {
    let mut st = Vec::new();

    for rule in rules {
        st.push(format!(
            "{} => {}",
            arena.render(&[rule.predecessor]),
            arena.render(&rule.successor)
        ));
    }

    st.join(",")
}

impl std::fmt::Debug for LSystemBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.debug_struct("LSystemBuilder")
            .field("arena", &self.arena)
            .field("axiom", &self.axiom)
            .field("rules", &build_rules_string(&self.rules, &self.arena))
            .finish()
    }
}

/// An L-system.
///
/// # Basic Usage
///
/// The suggested method for constructing an `LSystem` is to use a `LSystemBuilder`, as shown in our implementation
/// of Lindenmayer's system below.
///
/// ```rust
/// use dcc_lsystem::{LSystemBuilder, TokenType};
///
/// let mut builder = LSystemBuilder::new();
///
/// // Set up our two tokens
/// let a = builder.token("A", TokenType::Variable);
/// let b = builder.token("B", TokenType::Variable);
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
/// Once the LSystem has been built, you can use the step() and step_by() methods
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
/// [`LSystemBuilder`]: struct.LSystemBuilder.html
#[derive(Clone, Debug)]
pub struct LSystem {
    arena: Arena<Token>,
    axiom: Vec<ArenaId>,
    rules_map: HashMap<ArenaId, Vec<ArenaId>>,
    state: Vec<ArenaId>,
}

impl LSystem {
    /// Reset the system to its initial state.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::{TokenType, LSystemBuilder};
    ///
    /// let mut builder = LSystemBuilder::new();
    ///
    /// //  Create a simple L-System with one variable `A` and production rule `A -> AA`
    /// let a = builder.token("A", TokenType::Variable);
    /// builder.axiom(vec![a]);
    /// builder.transformation_rule(a, vec![a,a]);
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
    }

    /// Iterate the system a single step.
    pub fn step(&mut self) {
        let mut next_state = Vec::new();

        for id in self.state.iter() {
            next_state.extend(self.rules_map[id].clone());
        }

        self.state = next_state;
    }

    /// Iterate the system by n steps.
    pub fn step_by(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Returns the current state of the system as a String.
    pub fn render(&self) -> String {
        self.state
            .iter()
            .map(|id| self.arena.get(*id).unwrap().name().clone())
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let zero = builder.token("0", TokenType::Variable);
        let one = builder.token("1", TokenType::Variable);
        let left_square_bracket = builder.token("[", TokenType::Constant);
        let right_square_bracket = builder.token("]", TokenType::Constant);

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
}
