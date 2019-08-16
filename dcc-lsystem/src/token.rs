use crate::arena::{Arena, ArenaId};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    name: String,
}

impl Token {
    pub fn new<T: Into<String>>(name: T) -> Self {
        let name = name.into();

        if name.contains(" ") {
            panic!("Token name may not contain whitespace");
        }

        Self { name }
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
