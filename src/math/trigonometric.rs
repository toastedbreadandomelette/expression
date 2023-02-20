use crate::math::func_traits::VariableFunction;

#[derive(Copy, Clone)]
pub enum TrigonometricFunction {
    Sine,
    Cosine,
    Tangent,
    Cotangent,
    Secant,
    Cosecant,
}

macro_rules! sin {
    () => {
        TrigonometricFunction::Sine
    };
}

macro_rules! cos {
    () => {
        TrigonometricFunction::Cosine
    };
}

macro_rules! tan {
    () => {
        TrigonometricFunction::Tangent
    };
}

macro_rules! cot {
    () => {
        TrigonometricFunction::Cotangent
    };
}

macro_rules! sec {
    () => {
        TrigonometricFunction::Secant
    };
}

macro_rules! cosec {
    () => {
        TrigonometricFunction::Cosecant
    };
}

impl VariableFunction for TrigonometricFunction {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            TrigonometricFunction::Sine => x.sin(),
            TrigonometricFunction::Cosine => x.cos(),
            TrigonometricFunction::Tangent => x.tan(),
            TrigonometricFunction::Cotangent => 1.0 / x.tan(),
            TrigonometricFunction::Secant => 1.0 / x.cos(),
            TrigonometricFunction::Cosecant => 1.0 / x.sin(),
        }
    }

    fn derivative(&self) -> Self {
        match self {
            TrigonometricFunction::Sine => cos!(),
            TrigonometricFunction::Cosine => sin!(),
            TrigonometricFunction::Tangent => cot!(),
            TrigonometricFunction::Cotangent => tan!(),
            TrigonometricFunction::Secant => sec!(),
            TrigonometricFunction::Cosecant => cosec!(),
        }
    }
}
