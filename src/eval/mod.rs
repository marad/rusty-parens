mod scope;
mod error;

use crate::reader::Expression as Expr;
pub use scope::{Scope, ScopeError};
use error::EvalError;

pub fn eval(scope: &Scope, expr: &Expr) -> Result<Expr, EvalError> {
    match expr {
        Expr::Identifier(ident) =>
            Ok(scope.get(ident)?.clone()),
        Expr::List(data) =>
            eval_list(scope, &data),
        c => Ok(c.clone())
    }
}

fn eval_list(scope: &Scope, list: &Vec<Expr>) -> Result<Expr, EvalError> {
    let func = eval(scope, &list[0])?;
    match func {
        Expr::Fn(func) => Ok(func.call(&list[1..])?),
        expr => Err(EvalError::NotAFunction(expr))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod basic {
        use super::*;
        use crate::reader::Reader;
        use crate::reader::Expression::Integer;
        use failure::Error;
        use crate::reader::Function;

        #[test]
        fn should_eval_values_to_themselves() -> Result<(), Error> {
            // given
            let mut scope = Scope::new();
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
            let mut scope = Scope::new();
            scope.put(&"integer", integer_expr.clone());
            scope.put(&"float", float_expr.clone());
            scope.put(&"string", string_expr.clone());

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
            let native_func : fn(&[Expression]) -> Result<Expression, Error> =
                |exprs| Ok(exprs.first().unwrap().clone());
            let func = Expr::Fn(Function::Native(native_func));
            let mut scope = Scope::new();
            scope.put(&"identity", func.clone());
            let expr = Reader::from_string("(identity 5)").read()?;

            // when
            let result = eval(&mut scope, &expr)?;

            // then
            assert_eq!(Integer(5), result);
            Ok(())
        }
    }
}