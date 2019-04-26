use std::collections::HashMap;
use crate::reader::{Expression as Expr, Function};
use failure::Error;
use crate::eval::EvalError::NotAFunction;

#[derive(Debug, Fail)]
pub enum EvalError {
    #[fail(display = "{:?} is not a function", _0)]
    NotAFunction(Expr),
}


pub struct Scope {
    names: HashMap<String, Expr>
}

pub fn eval(scope: &mut Scope, expr: &Expr) -> Result<Expr, EvalError> {
    match expr {
        Expr::Identifier(ident) =>
            Ok(scope.names[ident].clone()),
        Expr::List(data) =>
            eval_list(scope, &data),
        c => Ok(c.clone())
    }
}

fn eval_list(scope: &mut Scope, list: &Vec<Expr>) -> Result<Expr, EvalError> {
    let func = eval(scope, &list[0])?;
    match func {
        Expr::Fn(func) => Ok(func.call(&list[1..])),
        expr => Err(NotAFunction(expr))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod basic {
        use super::*;
        use crate::reader::Reader;
        use crate::reader::Expression::Integer;

        #[test]
        fn should_eval_values_to_themselves() -> Result<(), Error> {
            // given
            let mut scope = Scope {
                names: HashMap::new()
            };
            let integer_expr = Expr::Integer(42);
            let float_expr = Expr::Float(3.14);
            let string_expr = Expr::String("hello".to_owned());

            // expect
            assert_eq!(integer_expr, eval(&mut scope, &integer_expr)?);
            assert_eq!(float_expr, eval(&mut scope, &float_expr)?);
            assert_eq!(string_expr, eval(&mut scope, &string_expr)?);
            Ok(())
        }

        #[test]
        fn should_evaluate_identifiers() -> Result<(), Error> {
            // given
            let integer_expr = Expr::Integer(42);
            let float_expr = Expr::Float(3.14);
            let string_expr = Expr::String("hello".to_owned());
            let mut scope = Scope {
                names: HashMap::new()
            };
            scope.names.insert("integer".to_owned(), integer_expr.clone());
            scope.names.insert("float".to_owned(), float_expr.clone());
            scope.names.insert("string".to_owned(), string_expr.clone());

            // expect
            assert_eq!(integer_expr, eval(&mut scope, &Expr::Identifier("integer".to_owned()))?);
            assert_eq!(float_expr, eval(&mut scope, &Expr::Identifier("float".to_owned()))?);
            assert_eq!(string_expr, eval(&mut scope, &Expr::Identifier("string".to_owned()))?);

            Ok(())
        }

        // TODO: test eval empty list
        // TODO: test eval list with "not" a function as a first arg
        #[test]
        fn should_evaluate_functions() -> Result<(), Error>{
            // given
            let func = Expr::Fn(Function::Identity);
            let mut scope = Scope {
                names: HashMap::new()
            };
            scope.names.insert("identity".to_owned(), func.clone());
            let expr = Reader::from_string("(identity 5)").read()?;

            // when
            let result = eval(&mut scope, &expr)?;

            // then
            assert_eq!(Integer(5), result);

            Ok(())
        }
    }
}