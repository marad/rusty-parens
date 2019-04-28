use std::collections::HashMap;
use crate::reader::Expression;

pub struct Scope {
    names: HashMap<String, Expression>
}

impl Scope {
    pub fn new() -> Self {
        Self {
            names: HashMap::new()
        }
    }

    pub fn put(&mut self, name: &ToString, value: Expression) {
        self.names.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Result<Expression, ScopeError> {
        self.names.get(name).ok_or_else(||ScopeError::IdentifierNotFound(name.to_string()))
            .map(Clone::clone)
    }
}

#[derive(Debug, Fail)]
pub enum ScopeError {
    #[fail(display = "Identifier not found in scope: {}", _0)]
    IdentifierNotFound(String)
}
