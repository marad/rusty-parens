use crate::tokenizer::TokenizerError::{
    InvalidNumberCharacter, NotAnEscapableCharacter, UnexpectedEndOfInput,
};
use failure::Error;

#[derive(Debug, Fail)]
pub enum TokenizerError {
    #[fail(display = "Unexpected end of input")]
    UnexpectedEndOfInput,

    #[fail(display = "This is not an escapable character: {}", _0)]
    NotAnEscapableCharacter(char),

    #[fail(display = "Invalid character in number: {}", _0)]
    InvalidNumberCharacter(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    String,
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    LeftParen,
    RightParen,
    Value(String, ValueType),
}

#[derive(Debug, Clone)]
pub struct Tokenizer {
    to_read: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn from_string(s: &str) -> Self {
        Self {
            to_read: s.chars().collect(),
            position: 0,
        }
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        loop {
            if self.can_read() {
                match self.peek_char() {
                    '(' => {
                        self.consume_char();
                        return Ok(Token::LeftParen);
                    }
                    ')' => {
                        self.consume_char();
                        return Ok(Token::RightParen);
                    }
                    ' ' | '\n' | '\t' => {
                        self.consume_char();
                        continue;
                    }
                    c if c.is_digit(10) => return self.read_number(),
                    '"' => return self.read_string(),
                    _ => return self.read_identifier(),
                }
            } else {
                return Err(UnexpectedEndOfInput.into());
            }
        }
    }

    fn can_read(&self) -> bool {
        self.position < self.to_read.len()
    }
    fn peek_char(&self) -> char {
        self.to_read[self.position]
    }

    fn consume_char(&mut self) -> char {
        let ch = self.peek_char();
        self.position += 1;
        ch
    }

    fn read_identifier(&mut self) -> Result<Token, Error> {
        let mut current_token = String::new();
        loop {
            if self.can_read() {
                match self.peek_char() {
                    '[' | ']' | '{' | '}' | '(' | ')' | ' ' => break,
                    _ => current_token.push(self.consume_char()),
                }
            } else {
                break;
            }
        }

        Ok(Token::Identifier(current_token))
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        let mut current_token = String::new();
        self.consume_char(); // consume starting quote
        loop {
            if self.can_read() {
                match self.peek_char() {
                    '"' => {
                        self.consume_char();
                        break;
                    }
                    '\\' => {
                        self.consume_char(); // consume '/'
                        let to_escape = self.consume_char();
                        let escaped = self.get_escaped_char(to_escape)?;
                        current_token.push(escaped);
                    }
                    _ => current_token.push(self.consume_char()),
                }
            }
        }

        Ok(Token::Value(current_token, ValueType::String))
    }

    fn get_escaped_char(&self, c: char) -> Result<char, Error> {
        Ok(match c {
            'n' => '\n',
            't' => '\t',
            '\\' => '\\',
            _ => return Err(NotAnEscapableCharacter(c).into()),
        })
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        let mut current_token = String::new();
        loop {
            if self.can_read() {
                match self.peek_char() {
                    '.' => current_token.push(self.consume_char()),
                    c if c.is_digit(10) => current_token.push(self.consume_char()),
                    ' ' | ',' | ')' | ']' | '}' | '\n' | '\t' => break,
                    _ => return Err(InvalidNumberCharacter(self.consume_char()).into()),
                }
            } else {
                break;
            }
        }

        Ok(Token::Value(current_token, ValueType::Number))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod basic {
        use super::*;
        #[test]
        fn should_read_identifier() {
            // given
            let code = "identifier";
            let mut tokenizer = Tokenizer::from_string(code);

            // when
            let token = tokenizer.next().unwrap();

            // then
            assert_eq!(token, Token::Identifier(code.to_owned()));
        }

        #[test]
        fn should_read_string() {
            // given
            let code = "\"some string\"";
            let mut tokenizer = Tokenizer::from_string(code);

            // expect
            assert_eq!(
                Token::Value("some string".to_owned(), ValueType::String),
                tokenizer.next().unwrap()
            )
        }

        #[test]
        fn should_read_integer() {
            // given
            let code = "1234";
            let mut tokenizer = Tokenizer::from_string(code);

            // expect
            assert_eq!(
                Token::Value("1234".to_owned(), ValueType::Number),
                tokenizer.next().unwrap()
            )
        }

        #[test]
        fn should_read_float() {
            // given
            let code = "12.34";
            let mut tokenizer = Tokenizer::from_string(code);

            // expect
            assert_eq!(
                Token::Value("12.34".to_owned(), ValueType::Number),
                tokenizer.next().unwrap()
            )
        }

        #[test]
        fn should_ignore_leading_whitespace() {
            // given
            let code = " \n\t  123";
            let mut tokenizer = Tokenizer::from_string(code);

            // expect
            assert_eq!(
                Token::Value("123".to_owned(), ValueType::Number),
                tokenizer.next().unwrap()
            )
        }

        #[test]
        fn should_ignore_trailing_whitespace_when_reading_numbers() {
            // given
            let code = "123\t\n ";
            let mut tokenizer = Tokenizer::from_string(code);

            // expect
            assert_eq!(
                Token::Value("123".to_owned(), ValueType::Number),
                tokenizer.next().unwrap()
            )
        }
    }

    #[test]
    fn should_escape_string_characters() {
        for (to_escape, escaped) in &[('n', '\n'), ('t', '\t'), ('\\', '\\')] {
            // given
            let code = format!("\"some\\{}string\"", to_escape);
            let mut tokenizer = Tokenizer::from_string(&code);

            // expect
            assert_eq!(
                Token::Value(format!("some{}string", escaped), ValueType::String),
                tokenizer.next().unwrap()
            )
        }
    }

    #[test]
    fn should_read_simple_function_call() {
        // given
        let code = "(some-func)";
        let mut tokenizer = Tokenizer::from_string(code);

        // expect
        assert_eq!(Token::LeftParen, tokenizer.next().unwrap());
        assert_eq!(
            Token::Identifier("some-func".to_owned()),
            tokenizer.next().unwrap()
        );
        assert_eq!(Token::RightParen, tokenizer.next().unwrap());
    }

    #[test]
    fn should_read_function_call_with_arguments() {
        // given
        let code = "(some-func ident \"string\" 10 12.6)";
        let mut tokenizer = Tokenizer::from_string(code);

        // expect
        assert_eq!(Token::LeftParen, tokenizer.next().unwrap());
        assert_eq!(
            Token::Identifier("some-func".to_owned()),
            tokenizer.next().unwrap()
        );
        assert_eq!(
            Token::Identifier("ident".to_owned()),
            tokenizer.next().unwrap()
        );
        assert_eq!(
            Token::Value("string".to_owned(), ValueType::String),
            tokenizer.next().unwrap()
        );
        assert_eq!(
            Token::Value("10".to_owned(), ValueType::Number),
            tokenizer.next().unwrap()
        );
        assert_eq!(
            Token::Value("12.6".to_owned(), ValueType::Number),
            tokenizer.next().unwrap()
        );
        assert_eq!(Token::RightParen, tokenizer.next().unwrap());
    }
}
