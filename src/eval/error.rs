use failure::Error;
use super::scope::ScopeError;
use crate::reader::Expression;

#[derive(Debug, Fail)]
pub enum EvalError {
    #[fail(display = "{:?} is not a function", _0)]
    NotAFunction(Expression),

    #[fail(display = "Scope error: {}", _0)]
    ScopeError(ScopeError),

    #[fail(display = "Custom error: {}", _0)]
    CustomError(Error),
}

impl From<Error> for EvalError {
    fn from(err: Error) -> Self {
        EvalError::CustomError(err)
    }
}

impl From<ScopeError> for EvalError {
    fn from(err: ScopeError) -> Self {
        EvalError::ScopeError(err)
    }
}

