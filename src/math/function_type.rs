use crate::math::func_traits::VariableFunction;
use crate::math::polynomial::Polynomial;
use crate::math::trigonometric::TrigonometricFunction;

#[derive(Clone)]
pub enum FunctionType {
    Trigonometric(TrigonometricFunction),
    Constant,
    Polynomial(Polynomial<f64>),
}

impl VariableFunction for FunctionType {
    fn evaluate(&self, x: f64) -> f64 {
        match *self {
            FunctionType::Trigonometric(ref trig) => trig.evaluate(x),
            FunctionType::Polynomial(ref poly) => poly.evaluate(x),
            _ => x,
        }
    }

    fn derivative(&self) -> Self {
        match *self {
            FunctionType::Trigonometric(ref trig) => FunctionType::Trigonometric(trig.derivative()),
            FunctionType::Polynomial(ref poly) => FunctionType::Polynomial(poly.derivative()),
            _ => FunctionType::Constant,
        }
    }
}
