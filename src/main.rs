mod math;

use crate::math::{
    expression::Expression, expression::ExpressionType, func_traits::VariableFunction,
    function_type::FunctionType, polynomial::Polynomial, trigonometric::TrigonometricFunction,
};

fn main() {
    let v = std::f64::consts::PI / 2.0;
    let c = 4.0;
    let expr = Expression::new_from(&vec![
        Expression {
            function: FunctionType::Trigonometric(TrigonometricFunction::Sine),
            input: ExpressionType::Constant(v),
        },
        Expression {
            function: FunctionType::Trigonometric(TrigonometricFunction::Cosine),
            input: ExpressionType::Polynomial(2.5 * x!() + 2.5),
        },
    ]);
    println!("{} {}", expr.evaluate(c), v.sin() + (2.5 * c + 2.5).cos());
}
