use crate::tokenizer::{Tokenizer, Token, ValueType};
use failure::Error;
use std::cell::RefCell;
use crate::reader::Expression::*;
use std::string::String as StdString;

//trait Function: PartialEq {
//    fn call(&self, args: Vec<Expression>) -> Expression;
//}
#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Identity,
}

impl Function {
    pub fn call(&self, args: &[Expression]) -> Expression {
        match self {
            Function::Identity => args.first().unwrap().clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(StdString),
    String(StdString),
    Integer(i32),
    Float(f32),
//    Fn(Box<Function>),
    Fn(Function),
    List(Vec<Expression>),
}

pub struct Reader {
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
        self.read_form(token)
    }

    fn read_form(&self, token: Token) -> Result<Expression, Error> {
        Ok(match token {
            Token::Identifier(ident) =>
                Expression::Identifier(ident),
            Token::Value(value, ValueType::String) =>
                Expression::String(value),
            Token::Value(value, ValueType::Number) =>
                self.read_number(&value)?,
            Token::LeftParen =>
                self.read_list()?,
            _ => Expression::Identifier("--".to_owned()), // todo error
        })
    }

    fn read_number(&self, value: &StdString) -> Result<Expression, Error> {
        Ok(if value.contains(".") {
            let val = value.parse::<f32>()?;
            Expression::Float(val)
        }
        else {
            let val = value.parse::<i32>()?;
            Expression::Integer(val)
        })
    }

    fn read_list(&self) -> Result<Expression, Error> {
        let mut contents: Vec<Expression> = vec![];
        loop {
            let token = self.tokenizer.borrow_mut().next()?;
            match token {
                Token::RightParen => break,
                other_token => {
                    let expr = self.read_form(other_token)?;
                    contents.push(expr);
                }
            }
        }
        Ok(List(contents))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod basic {
        use super::*;

        #[test]
        fn should_read_identifier() -> Result<(), Error> {
            // given
            let code = "ident";
            let reader = Reader::from_string(code);

            // expect
            assert_eq!(Identifier("ident".to_owned()), reader.read()?);
            Ok(())
        }

        #[test]
        fn should_read_string() -> Result<(), Error> {
            // given
            let code = "\"some string\"";
            let reader = Reader::from_string(code);

            // expect
            assert_eq!(String("some string".to_owned()), reader.read()?);
            Ok(())
        }

        #[test]
        fn should_read_integer() -> Result<(), Error> {
            // given
            let code = "231";
            let reader = Reader::from_string(code);

            // expect
            assert_eq!(Integer(231), reader.read()?);
            Ok(())
        }

        #[test]
        fn should_read_float() -> Result<(), Error> {
            // given
            let code = "3.14";
            let reader = Reader::from_string(code);

            // expect
            assert_eq!(Float(3.14), reader.read()?);
            Ok(())
        }

        // TODO: test for reading invalid number like '3.2xc'

        #[test]
        fn should_rad_list() -> Result<(), Error> {
            // given
            let code = "(say-hello \"John\")";
            let reader = Reader::from_string(code);

            // expect
            assert_eq!(List(vec![
                Identifier("say-hello".to_owned()),
                String("John".to_owned()),
            ]), reader.read()?);
            Ok(())
        }
    }

    #[test]
    fn should_read_nested_lists() -> Result<(), Error> {
        // given
        let code = "(say-hello (str \"John\" surname))";
        let reader = Reader::from_string(code);

        // expect
        assert_eq!(List(vec![
            Identifier("say-hello".to_owned()),
            List(vec![
                Identifier("str".to_owned()),
                String("John".to_owned()),
                Identifier("surname".to_owned())
                ]),
        ]), reader.read()?);
        Ok(())
    }
}