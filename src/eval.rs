use std::collections::HashMap;
use crate::reader::Expression;

pub struct Scope {
    names: HashMap<String, Expression>
}

pub fn eval(scope: &mut Scope, expr: &Expression) -> Expression {
    match expr {
        c => c.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_eval_values_to_themselves() {
        // given
        let mut scope = Scope {
            names: HashMap::new()
        };
        let integer_expr = Expression::Integer(42);
        let float_expr = Expression::Float(3.14);
        let string_expr = Expression::String("hello".to_owned());

        // expect
        assert_eq!(integer_expr, eval(&mut scope, &integer_expr));
        assert_eq!(float_expr, eval(&mut scope, &float_expr));
        assert_eq!(string_expr, eval(&mut scope, &string_expr))
    }
}