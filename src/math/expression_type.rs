use std::fmt::Display;

use crate::math::function_type::FunctionType;

use super::{expression::Expression, func_traits::VariableFunction, polynomial::Polynomial};

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    Functions(Vec<Expression>),
    Constant(f64),
    Polynomial(Polynomial<f64>),
    MultipliedFunction(Vec<Expression>),
    DivFunction(Box<Expression>, Box<Expression>),
}

impl Display for ExpressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match *self {
            Self::Functions(ref value) => value
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" + "),
            Self::Constant(val) => val.to_string(),
            Self::Polynomial(ref val) => format!("({})", val.to_string()),
            Self::MultipliedFunction(ref value) => value
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" * "),
            Self::DivFunction(ref num, ref den) => {
                format!("({}) / ({})", num.to_string(), den.to_string())
            }
        };
        f.write_str(out.as_str())
    }
}

impl ExpressionType {
    pub fn is_constant(&self) -> bool {
        match &self {
            Self::Constant(val) => true,
            Self::DivFunction(num, den) => num.is_constant() && den.is_constant(),
            Self::MultipliedFunction(ref value) => value.iter().all(|c| c.is_constant()),
            Self::Functions(ref value) => value.iter().all(|c| c.is_constant()),
            Self::Polynomial(ref value) => value.deg == 0,
        }
    }
}

impl VariableFunction for ExpressionType {
    fn evaluate(&self, x: f64) -> f64 {
        match &self {
            Self::Constant(val) => *val,
            Self::DivFunction(num, den) => num.evaluate(x) / den.evaluate(x),
            Self::Polynomial(ref value) => value.evaluate(x),
            Self::MultipliedFunction(ref value) => value
                .iter()
                .map(|c| c.evaluate(x))
                .reduce(|p, c| p * c)
                .unwrap(),
            Self::Functions(ref value) => value
                .iter()
                .map(|c| c.evaluate(x))
                .reduce(|p, c| p + c)
                .unwrap(),
        }
    }

    fn derivative(&self) -> Self {
        match &self {
            Self::Constant(ref value) => Self::Constant(0.0),
            Self::Functions(ref value) => Self::Functions(
                value
                    .iter()
                    .filter(|c| !c.is_constant())
                    .map(|c| c.derivative())
                    .collect::<Vec<Expression>>(),
            ),
            Self::MultipliedFunction(ref value) => Self::Functions(
                value
                    .iter()
                    .filter(|c| !c.is_constant())
                    .map(|c| {
                        Self::MultipliedFunction(
                            [
                                value
                                    .iter()
                                    .filter(|p| **p != *c)
                                    .map(|p| p.clone())
                                    .collect::<Vec<Expression>>(),
                                vec![c.derivative()],
                            ]
                            .concat(),
                        )
                    })
                    .map(|der| Expression {
                        function: FunctionType::Constant,
                        input: der,
                    })
                    .collect::<Vec<Expression>>(),
            ),
            Self::Polynomial(ref value) => Self::Polynomial(value.derivative()),
            // Self::DivFunction(ref num, ref den) => Self::DivFunction(d, ())
            _ => Self::Constant(0.0),
        }
    }
}
