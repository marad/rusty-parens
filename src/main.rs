#[macro_use]
extern crate failure;

mod reader;
mod tokenizer;
mod eval;

use std::io;
use crate::reader::{Expression, Reader, Function};
use failure::Error;
use eval::eval;
use crate::eval::Scope;
use std::io::Write;

fn main() -> Result<(), Error> {
    println!("Rusty Parens");
    let mut scope = Scope::new();

    scope.put(&"identity", Expression::Fn(Function::Identity));
    scope.put(&"+", Expression::Fn(Function::IntegerAdd));

    loop {
        let expr = read()?;
        let result = eval(&mut scope, &expr)?;
        print(result);
    }
}

fn read() -> Result<Expression, Error> {
    print!("> "); io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Reader::from_string(&buffer).read()
}

fn print(expr: Expression) {
    println!("{:?}", expr)
}
