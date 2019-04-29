use super::reader::Expression;

use self::error::EvalError;
pub use self::scope::{Scope, ScopeError};

mod error;
mod scope;

pub fn eval(scope: &mut Scope, expr: &Expression) -> Result<Expression, EvalError> {
    match expr {
        Expression::Identifier(ident) => Ok(scope.get(ident)?),
        Expression::List(data) => eval_list(scope, &data),
        c => Ok(c.clone()),
    }
}

fn eval_list(scope: &mut Scope, list: &[Expression]) -> Result<Expression, EvalError> {
    if list.len() == 0 {
        return Err(EvalError::EmptyList)
    }

    let func = eval(scope, &list[0])?;
    match func {
        Expression::Fn(func) => {
            let args = list[1..]
                .iter()
                .map(|expr| eval(scope, expr))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(func.call(&args)?)
        }
        expr => Err(EvalError::NotAFunction(expr)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod basic {
        use failure::Error;

        use crate::reader::Expression as Expr;
        use crate::reader::Function;
        use crate::reader::Reader;

        use super::*;

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

        mod identifiers {
            use super::*;
            use crate::eval::scope::ScopeError::IdentifierNotFound;

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
                assert_eq!(
                    integer_expr,
                    eval(&mut scope, &Expr::Identifier("integer".to_owned()))?
                );
                assert_eq!(
                    float_expr,
                    eval(&mut scope, &Expr::Identifier("float".to_owned()))?
                );
                assert_eq!(
                    string_expr,
                    eval(&mut scope, &Expr::Identifier("string".to_owned()))?
                );

                Ok(())
            }

            #[test]
            fn should_return_proper_error_when_identifier_is_not_found() {
                // given
                let mut scope = Scope::new();

                // when
                let error = eval(&mut scope, &Expr::Identifier("identifier".to_owned())).err().unwrap();

                // expect
                match error {
                    EvalError::ScopeError(IdentifierNotFound(ident)) => assert_eq!(ident, "identifier"),
                    err => panic!("Invalid error returned: {}. Expected Identifier not found error.", err)
                }
            }
        }

        mod functions {
            use super::*;

            #[test]
            fn should_evaluate_functions() -> Result<(), Error> {
                // given
                let native_func: fn(&[Expression]) -> Result<Expression, Error> =
                    |exprs| Ok(exprs.first().unwrap().clone());
                let func = Expr::Fn(Function::Native(native_func));
                let mut scope = Scope::new();
                scope.put(&"identity", func.clone());
                let expr = Reader::from_string("(identity 5)").read()?;

                // when
                let result = eval(&mut scope, &expr)?;

                // then
                assert_eq!(Expr::Integer(5), result);
                Ok(())
            }

            #[test]
            fn should_eval_function_args() -> Result<(), Error> {
                // given
                let native_func: fn(&[Expression]) -> Result<Expression, Error> =
                    |exprs| Ok(exprs.first().unwrap().clone());
                let func = Expr::Fn(Function::Native(native_func));
                let mut scope = Scope::new();
                scope.put(&"identity", func.clone());
                let expr = Reader::from_string("(identity (identity 5))").read()?;

                // when
                let result = eval(&mut scope, &expr)?;

                // then
                assert_eq!(Expr::Integer(5), result);

                Ok(())
            }

            #[test]
            fn should_return_error_when_evaluating_empty_list() -> Result<(), Error> {
                // given
                let mut scope = Scope::new();
                let expr = Reader::from_string("()").read()?;

                // when
                let result = eval(&mut scope, &expr).err().unwrap();

                // then
                match result {
                    EvalError::EmptyList => true,
                    err => panic!("Wrong error returned: {}", err),
                };
                Ok(())
            }
        }
    }
}
