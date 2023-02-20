mod math;

use crate::math::{
    expression::Expression, expression_type::ExpressionType, func_traits::VariableFunction,
    function_type::FunctionType, polynomial::Polynomial, trigonometric::TrigonometricFunction,
};

fn main() {
    let v = std::f64::consts::PI / 2.0;
    let c = 4.0;
    let expr = Expression::new_from(&vec![
        Expression {
            function: FunctionType::Trigonometric(TrigonometricFunction::Sine),
            input: ExpressionType::Polynomial(x!()),
        },
        Expression {
            function: FunctionType::Trigonometric(TrigonometricFunction::Cosine),
            input: ExpressionType::Polynomial(2.5 * (x!() * &x!()) + 2.5),
        },
    ]);
    println!(
        "{}\n{}\n{}\n{} {}",
        expr.evaluate(c),
        expr,
        expr.derivative(),
        expr.derivative().evaluate(c),
        ((20.0_f64).cos() * ((2.5 * 16.0 + 2.5) as f64).sin())
    );
}
