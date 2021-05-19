use crate::LSystemError;

/// A token for use in an L-system.  In general, the `LSystem` owns the token,
/// while the user can refer to the token via an `ArenaId`.  This means
/// we don't have to deal with any tricky ownership issues.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    name: String,
}

impl Token {
    /// Create a new token with the given name.  Note that `name` must *not* contain
    /// any spaces.
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::token::Token;
    ///
    /// // A token can be whatever you want
    /// let token = Token::new("daisy").unwrap();
    ///
    /// // Except it can't have spaces!
    /// assert!(Token::new("geddy lee").is_err());
    /// ```
    pub fn new<T: Into<String>>(name: T) -> Result<Self, LSystemError> {
        let name = name.into();

        if name.contains(' ') {
            Err(LSystemError::InvalidToken(name))
        } else {
            Ok(Self { name })
        }
    }

    /// Get the name of this token
    ///
    /// # Example
    /// ```rust
    /// use dcc_lsystem::token::Token;
    ///
    /// let token = Token::new("cow").unwrap();
    /// assert_eq!(token.name(), "cow");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name())
    }
}
