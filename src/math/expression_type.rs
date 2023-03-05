use std::fmt::Display;

use crate::math::function_type::FunctionType;

use super::{expression::Expression, func_traits::VariableFunction, polynomial::Polynomial};

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    Expressions(Vec<Expression>),
    Constant(f64),
    Polynomial(Polynomial<f64>),
    MultipliedExpressions(Vec<Expression>),
    DividedExpressions(Box<Expression>, Box<Expression>),
}

impl Display for ExpressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match *self {
            Self::Expressions(ref value) => value
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" + "),
            Self::Constant(val) => val.to_string(),
            Self::Polynomial(ref val) => format!("({})", val.to_string()),
            Self::MultipliedExpressions(ref value) => value
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" * "),
            Self::DividedExpressions(ref num, ref den) => {
                format!("({}) / ({})", num.to_string(), den.to_string())
            }
        };
        f.write_str(out.as_str())
    }
}

impl ExpressionType {
    pub fn is_constant(&self) -> bool {
        match &self {
            Self::Constant(_val) => true,
            Self::DividedExpressions(num, den) => num.is_constant() && den.is_constant(),
            Self::MultipliedExpressions(ref value) => value.iter().all(|c| c.is_constant()),
            Self::Expressions(ref value) => value.iter().all(|c| c.is_constant()),
            Self::Polynomial(ref value) => value.deg == 0,
        }
    }
}

impl VariableFunction for ExpressionType {
    fn evaluate(&self, x: f64) -> f64 {
        match &self {
            Self::Constant(val) => *val,
            Self::DividedExpressions(num, den) => num.evaluate(x) / den.evaluate(x),
            Self::Polynomial(ref value) => value.evaluate(x),
            Self::MultipliedExpressions(ref value) => value
                .iter()
                .map(|c| c.evaluate(x))
                .reduce(|p, c| p * c)
                .unwrap(),
            Self::Expressions(ref value) => value
                .iter()
                .map(|c| c.evaluate(x))
                .reduce(|p, c| p + c)
                .unwrap(),
        }
    }

    fn derivative(&self) -> Self {
        match &self {
            Self::Constant(ref _value) => Self::Constant(0.0),
            Self::Expressions(ref value) => Self::Expressions(
                value
                    .iter()
                    .filter(|c| !c.is_constant())
                    .map(|c| c.derivative())
                    .collect::<Vec<Expression>>(),
            ),
            Self::MultipliedExpressions(ref value) => Self::Expressions(
                value
                    .iter()
                    .filter(|c| !c.is_constant())
                    .map(|c| {
                        Self::MultipliedExpressions(
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
                        function: FunctionType::None,
                        input: der,
                    })
                    .collect::<Vec<Expression>>(),
            ),
            Self::Polynomial(ref value) => Self::Polynomial(value.derivative()),
            // Self::DividedExpressions(ref num, ref den) => Self::DividedExpressions(d, ())
            _ => Self::Constant(0.0),
        }
    }
}
