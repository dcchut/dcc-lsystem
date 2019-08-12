use crate::arena::{Arena, ArenaId};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenType {
    Variable,
    Constant,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
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
