use std::fmt::Display;

use crate::math::func_traits::VariableFunction;
use crate::math::polynomial::Polynomial;
use crate::math::trigonometric::TrigonometricFunction;

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
    Trigonometric(TrigonometricFunction),
    None,
    Polynomial(Polynomial<f64>),
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self {
            Self::None => "".to_string(),
            Self::Polynomial(ref value) => value.to_string(),
            Self::Trigonometric(ref value) => value.to_string(),
        };
        f.write_str(output.as_str())
    }
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
            _ => FunctionType::None,
        }
    }
}
