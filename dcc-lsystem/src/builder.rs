use std::collections::HashMap;

use crate::arena::{Arena, ArenaId};
use crate::errors::LSystemError;
use crate::system::LSystem;
use crate::token::Token;

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

/// A struct for constructing [`LSystem`]s.
///
/// # Example
/// In the following example we build an LSystem with axiom `a`, variables `a` and `b`,
/// and transformation rule `a -> ab`.
///
/// ```rust
/// use dcc_lsystem::{LSystemBuilder, LSystemError};
///
/// fn main() -> Result<(), LSystemError> {
///     let mut builder = LSystemBuilder::new();
///
///     // Register our variables
///     let a = builder.token("a")?;
///     let b = builder.token("b")?;
///
///     // Add a transformation rule
///     builder.transformation_rule(a, vec![a, b])?;
///
///     // Set the axiom.
///     builder.axiom(vec![a])?;
///
///     // Build the LSystem and iterate it.
///     let mut system = builder.finish()?;
///     system.step_by(2);
///
///     assert_eq!(system.render(), "abb");
///     Ok(())
/// }
/// ```
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
    ///
    /// ```rust
    /// # use dcc_lsystem::LSystemError;
    /// # fn main() -> Result<(), LSystemError> {
    /// use dcc_lsystem::LSystemBuilder;
    ///
    /// // Register a few tokens
    /// let mut builder = LSystemBuilder::new();
    /// let a = builder.token("a")?;
    /// let b = builder.token("b")?;
    /// let c = builder.token("c")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn token<S: Into<String>>(&mut self, name: S) -> Result<ArenaId, LSystemError> {
        Ok(self.arena.push(Token::new(name)?))
    }

    fn validate_ids(&self, ids: &[ArenaId]) -> Result<(), LSystemError> {
        for &id in ids {
            if !self.arena.is_valid(id) {
                return Err(LSystemError::InvalidArenaId(id));
            }
        }

        Ok(())
    }

    /// Register a new transformation rule in this LSystem.
    ///
    /// This function will return an error if any of the provided TokenId are invalid.
    ///
    /// ```rust
    /// # use dcc_lsystem::LSystemError;
    /// # fn main() -> Result<(), LSystemError> {
    /// use dcc_lsystem::LSystemBuilder;
    ///
    /// // Build an LSystem with axiom `a` and transformation rule `a -> aab`.
    /// let mut builder = LSystemBuilder::new();
    /// let a = builder.token("a")?;
    /// let b = builder.token("b")?;
    /// builder.transformation_rule(a, vec![a, a, b])?;
    /// builder.axiom(vec![a, b])?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn transformation_rule(
        &mut self,
        predecessor: ArenaId,
        successor: Vec<ArenaId>,
    ) -> Result<(), LSystemError> {
        // Verify that all provided TokenId's correspond to a token in this LSystem.
        self.validate_ids(&[predecessor])?;
        self.validate_ids(&successor)?;

        // Add the rule to this system
        self.rules
            .push(TransformationRule::new(predecessor, successor));

        Ok(())
    }

    /// Set the axiom for this LSystem.
    ///
    /// # Example
    /// ```rust
    /// # use dcc_lsystem::LSystemError;
    /// # fn main() -> Result<(), LSystemError> {
    /// use dcc_lsystem::LSystemBuilder;
    ///
    /// let mut builder = LSystemBuilder::new();
    /// let x = builder.token("x")?;
    /// let y = builder.token("y")?;
    /// let z = builder.token("z")?;
    ///
    /// // Our [`LSystem`] will start from the state "yyz".
    /// builder.axiom(vec![y, y, z])?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn axiom(&mut self, axiom: Vec<ArenaId>) -> Result<(), LSystemError> {
        self.validate_ids(axiom.as_slice())?;
        self.axiom = Some(axiom);

        Ok(())
    }

    /// Consumes the builder, returning an LSystem instance.  If an axiom has not been
    /// set then this function will return an [`LSystemError::MissingAxiom`] error.
    ///
    /// # Example
    /// ```rust
    /// # use dcc_lsystem::LSystemError;
    /// # fn main() -> Result<(), LSystemError> {
    /// use dcc_lsystem::LSystemBuilder;
    ///
    /// let mut builder = LSystemBuilder::new();
    /// let k = builder.token("k")?;
    /// let g = builder.token("g")?;
    ///
    /// builder.axiom(vec![k, g]);
    /// builder.transformation_rule(k, vec![k, g]);
    ///
    /// let system = builder.finish()?;
    /// assert_eq!(system.render(), "kg");
    /// # Ok(())
    /// # }
    /// ```
    pub fn finish(self) -> Result<LSystem, LSystemError> {
        let axiom = self.axiom.ok_or(LSystemError::MissingAxiom)?;

        // Construct a HashMap associating each variable with its corresponding transformation rule
        let mut rules_map = HashMap::new();

        for rule in self.rules.into_iter() {
            rules_map.insert(rule.predecessor, rule.successor);
        }

        // We also add constant production rules of the form P => P.
        for (id, _token) in self.arena.enumerate() {
            // no rule associated to this token, so its a constant token
            rules_map.entry(id).or_insert_with(|| vec![id]);
        }

        // If we set our system up correctly, it should be that each token
        // contributes exactly one rule, so we check for that here.
        assert_eq!(self.arena.len(), rules_map.len());

        Ok(LSystem::new(self.arena, axiom, rules_map))
    }
}

/// Returns a string representation of the given slice of ArenaId's in terms
/// of the contents of this arena.
fn render_tokens(arena: &[Token], tokens: &[ArenaId]) -> String {
    let mut st = String::new();

    for token in tokens {
        st.push_str(&format!("{}", arena[token.0]));
    }

    st
}

fn build_rules_string(rules: &[TransformationRule], arena: &Arena<Token>) -> String {
    let mut st = Vec::new();

    for rule in rules {
        st.push(format!(
            "{} => {}",
            render_tokens(arena.as_slice(), &[rule.predecessor]),
            render_tokens(arena.as_slice(), &rule.successor),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_invalid_token() -> Result<(), LSystemError> {
        let mut builder = LSystemBuilder::new();

        let _daisy = builder.token("daisy")?;

        // make sure we can't add a token with a space in it
        assert!(builder.token("space cadet").is_err());

        Ok(())
    }

    #[test]
    fn test_builder_axiom_and_transformation_rule_errors() -> Result<(), LSystemError> {
        let mut builder = LSystemBuilder::new();

        let x = builder.token("x")?;
        let y = builder.token("y")?;

        let mut some_other_builder = LSystemBuilder::new();

        // `x` won't be valid for an empty builder
        assert!(some_other_builder.axiom(vec![x]).is_err());

        let q = some_other_builder.token("q")?;

        // make sure `y` still isn't valid
        assert!(some_other_builder.axiom(vec![y]).is_err());

        // similarly trying to add a transformation rule should go badly.
        assert!(some_other_builder
            .transformation_rule(q, vec![x, y, q])
            .is_err());
        assert!(some_other_builder
            .transformation_rule(y, vec![q, q])
            .is_err());

        Ok(())
    }
}
