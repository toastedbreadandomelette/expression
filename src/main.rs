mod math;

use crate::math::{
    expression::Expression, expression_type::ExpressionType, func_traits::VariableFunction,
    function_type::FunctionType, polynomial::Polynomial, trigonometric::TrigonometricFunction,
};

fn main() {
    let c = 4.0;
    let expr = Expression::new_from(&vec![
        Expression {
            function: FunctionType::Trigonometric(TrigonometricFunction::Sine),
            input: ExpressionType::Polynomial(x!()),
        },
        Expression {
            function: FunctionType::Trigonometric(TrigonometricFunction::Cosine),
            input: ExpressionType::Polynomial(2.5 * x!(3) + (x!(2) * 5.0) - 2.5),
        },
    ]);
    println!(
        "{}\n{}\n{}\n{} {}",
        expr.evaluate(c),
        expr,
        expr.derivative().derivative(),
        expr.derivative().evaluate(c),
        (((-2.5 + 2.5*16.0) as f64).cos() * (4_f64).cos()) + ((4_f64).sin() * 20.0_f64 * -((2.5 * 16.0 - 2.5) as f64).sin())
    );
}
