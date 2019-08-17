use crate::arena::{Arena, ArenaId};

/// A token for use in an L-system.  In general, the `LSystem` owns the token,
/// while the user can refer to the token via an `ArenaId`.  This means
/// we don't have to deal with any tricky ownership issues.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    name: String,
}

impl Token {
    /// Create a new token with the given name.
    ///
    /// # Panics
    /// This function will panic if `name` contains any spaces
    pub fn new<T: Into<String>>(name: T) -> Self {
        let name = name.into();

        if name.contains(' ') {
            panic!("Token name may not contain whitespace");
        }

        Self { name }
    }

    /// Get the name of this token
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name())
    }
}

impl Arena<Token> {
    /// Returns a string representation of the given slice of ArenaId's in terms
    /// of the contents of this arena.
    pub fn render(&self, tokens: &[ArenaId]) -> String {
        assert!(self.is_valid_slice(tokens));

        let mut st = String::new();

        for token in tokens {
            st.push_str(&format!("{}", self.get(*token).unwrap()));
        }

        st
    }
}
