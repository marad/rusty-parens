#[macro_use]
extern crate failure;

mod eval;
mod reader;
mod tokenizer;

use crate::eval::Scope;
use crate::reader::{Expression, Function, Reader};
use eval::eval;
use failure::Error;
use std::io;
use std::io::Write;

fn main() -> Result<(), Error> {
    println!("Rusty Parens");
    let mut scope = Scope::new();

    scope.put(&"+", Expression::Fn(Function::Native(integer_add)));

    loop {
        let expr = read()?;
        let result = eval(&mut scope, &expr);
        match result {
            Ok(result) => print(result),
            Err(error) => eprintln!("{}", error),
        }
    }
}

fn read() -> Result<Expression, Error> {
    print!("> ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Reader::from_string(&buffer).read()
}

fn print(expr: Expression) {
    println!("{}", expr)
}

fn integer_add(exprs: &[Expression]) -> Result<Expression, Error> {
    match exprs {
        [Expression::Integer(a), Expression::Integer(b)] => Ok(Expression::Integer(a + b)),
        _ => Ok(Expression::String("Incompatible types".to_owned())),
    }
}
