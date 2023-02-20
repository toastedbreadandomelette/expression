use crate::math::func_traits::VariableFunction;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl TrigonometricFunction {
    pub fn to_string(&self) -> String {
        match self {
            Self::Sine => "sin",
            Self::Cosine => "cos",
            Self::Tangent => "tan",
            Self::Cotangent => "cot",
            Self::Secant => "sec",
            Self::Cosecant => "cosec",
        }
        .to_string()
    }
}

impl VariableFunction for TrigonometricFunction {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Self::Sine => x.sin(),
            Self::Cosine => x.cos(),
            Self::Tangent => x.tan(),
            Self::Cotangent => 1.0 / x.tan(),
            Self::Secant => 1.0 / x.cos(),
            Self::Cosecant => 1.0 / x.sin(),
        }
    }

    fn derivative(&self) -> Self {
        match self {
            Self::Sine => cos!(),
            Self::Cosine => sin!(),
            Self::Tangent => cot!(),
            Self::Cotangent => tan!(),
            Self::Secant => sec!(),
            Self::Cosecant => cosec!(),
        }
    }
}
