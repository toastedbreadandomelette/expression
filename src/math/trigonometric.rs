use crate::math::func_traits::VariableFunction;

/// Function involving normal trigonometry
#[derive(Debug, Clone, PartialEq)]
pub enum TrigonometricFunction {
    Sine,
    Cosine,
    Tangent,
    Cotangent,
    Secant,
    Cosecant,
    Composite(Vec<TrigonometricFunction>),
    Negative(Box<TrigonometricFunction>)
}

macro_rules! composite {
    ($($e:expr),*) => {{
        TrigonometricFunction::Composite(vec![$($e),*])
    }}
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

macro_rules! neg {
    ($e:expr) => {
        TrigonometricFunction::Negative(Box::new($e))
    };
}

impl TrigonometricFunction {
    pub fn to_string(&self) -> String {
        match self {
            Self::Sine => "sin".to_string(),
            Self::Cosine => "cos".to_string(),
            Self::Tangent => "tan".to_string(),
            Self::Cotangent => "cot".to_string(),
            Self::Secant => "sec".to_string(),
            Self::Cosecant => "cosec".to_string(),
            Self::Composite(_) => "".to_string(),
            Self::Negative(ref value) => format!("-{}", value.to_string())
        }
    }

    fn flatten_composite(&self) -> Self {
        match self {
            Self::Composite(ref comp) => {
                let mut new_composite: Vec<TrigonometricFunction> = Vec::new();
                comp.iter().for_each(|item| {
                    item.flatten_composite();
                    match item {
                        Self::Composite(another_comp) => {
                            for val in another_comp {
                                new_composite.push(val.clone());
                            }
                        }
                        _ => { }
                    }
                });
                Self::Composite(new_composite)
            }
            _ => { self.clone() }
        }
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
            Self::Composite(ref value) => value
                .iter()
                .map(|f| f.evaluate(x))
                .reduce(|v, c| v * c)
                .unwrap(),
            Self::Negative(ref value) => -value.evaluate(x)
        }
    }

    fn derivative(&self) -> Self {
        match self {
            Self::Sine => cos!(),
            Self::Cosine => neg!(sin!()),
            Self::Tangent => composite!(sec!(), sec!()),
            Self::Cotangent => neg!(composite!(cosec!(), cosec!())),
            Self::Secant => composite!(sec!(), tan!()),
            Self::Cosecant => neg!(composite!(cosec!(), cot!())),
            Self::Negative(ref value) => neg!(value.derivative()),
            _ => cos!()
        }
    }
}
