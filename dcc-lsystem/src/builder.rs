use std::collections::HashMap;

use crate::arena::{Arena, ArenaId};
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
    pub fn token<S: Into<String>>(&mut self, name: S) -> ArenaId {
        self.arena.push(Token::new(name))
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
        for (id, _token) in self.arena.enumerate() {
            // no rule associated to this token, so its a constant token
            rules_map.entry(id).or_insert_with(|| vec![id]);
        }

        // If we set our system up correctly, it should be that each token
        // contributes exactly one rule, so we check for that here.
        assert_eq!(self.arena.len(), rules_map.len());

        LSystem::new(self.arena, axiom, rules_map)
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
