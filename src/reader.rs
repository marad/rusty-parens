use crate::reader::Expression::*;
use crate::tokenizer::{Token, Tokenizer, ValueType};
use failure::Error;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::{Debug, Formatter};
use std::string::String as StdString;

#[derive(Clone)]
pub enum Function {
    Native(fn(&[Expression]) -> Result<Expression, Error>),
    Regular(Vec<Expression>),
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        f.write_str("<function>")?;
        Ok(())
    }
}

impl PartialEq<Function> for Function {
    fn eq(&self, other: &Function) -> bool {
        self.type_id() == other.type_id()
    }
}

impl Function {
    pub fn call(&self, args: &[Expression]) -> Result<Expression, Error> {
        match self {
            Function::Native(f) => f(args),
            Function::Regular(_exprs) => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(StdString),
    String(StdString),
    Integer(i32),
    Float(f32),
    Fn(Function),
    List(Vec<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Expression::Float(value) => f.write_fmt(format_args!("{}", value))?,
            Expression::Integer(value) => f.write_fmt(format_args!("{}", value))?,
            Expression::Fn(_) => f.write_str("<function>")?,
            Expression::Identifier(value) => f.write_fmt(format_args!("{}", value))?,
            Expression::String(value) => f.write_fmt(format_args!("{}", value))?,
            Expression::List(values) => {
                f.write_str("(")?;
                values
                    .iter()
                    .map(|s| Display::fmt(s, f))
                    .collect::<Result<Vec<_>, _>>()?;
                f.write_str(")")?;
            }
        }
        Ok(())
    }
}

pub struct Reader {
    tokenizer: RefCell<Tokenizer>,
}

impl Reader {
    pub fn from_string(code: &str) -> Self {
        Self {
            tokenizer: RefCell::new(Tokenizer::from_string(code)),
        }
    }

    pub fn read(&self) -> Result<Expression, Error> {
        let token = self.tokenizer.borrow_mut().next()?;
        self.read_form(token)
    }

    fn read_form(&self, token: Token) -> Result<Expression, Error> {
        Ok(match token {
            Token::Identifier(ident) => Expression::Identifier(ident),
            Token::Value(value, ValueType::String) => Expression::String(value),
            Token::Value(value, ValueType::Number) => self.read_number(&value)?,
            Token::LeftParen => self.read_list()?,
            _ => Expression::Identifier("--".to_owned()), // todo error
        })
    }

    fn read_number(&self, value: &str) -> Result<Expression, Error> {
        Ok(if value.contains('.') {
            let val = value.parse::<f32>()?;
            Expression::Float(val)
        } else {
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
            assert_eq!(
                List(vec![
                    Identifier("say-hello".to_owned()),
                    String("John".to_owned()),
                ]),
                reader.read()?
            );
            Ok(())
        }
    }

    #[test]
    fn should_read_nested_lists() -> Result<(), Error> {
        // given
        let code = "(say-hello (str \"John\" surname))";
        let reader = Reader::from_string(code);

        // expect
        assert_eq!(
            List(vec![
                Identifier("say-hello".to_owned()),
                List(vec![
                    Identifier("str".to_owned()),
                    String("John".to_owned()),
                    Identifier("surname".to_owned())
                ]),
            ]),
            reader.read()?
        );
        Ok(())
    }
}
