use crate::tokenizer::{Tokenizer, Token};
use failure::Error;
use std::cell::RefCell;

trait Function {
    fn call(&self);
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    String(String),
    Integer(i32),
    Float(f32),
    List(Vec<Expression>),
    Vector(Vec<Expression>),
}

struct Reader {
    tokenizer: RefCell<Tokenizer>,
}

impl Reader {
    pub fn from_string(code: &str) -> Self {
        Self {
            tokenizer: RefCell::new(Tokenizer::from_string(code))
        }
    }

    pub fn read(&self) -> Result<Expression, Error> {
        let token = self.tokenizer.borrow_mut().next()?;
        let form = match token {
            Token::Identifier(ident) => Expression::Identifier(ident),
            _ => Expression::Identifier("--".to_owned()),
        };
        Ok(form)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_read_identifier() -> Result<(), Error> {
        // given
        let code = "ident";
        let reader = Reader::from_string(code);

        // expect
        assert_eq!(Expression::Identifier("ident".to_owned()), reader.read()?);
        Ok(())
    }
}