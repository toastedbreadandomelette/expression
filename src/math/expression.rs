use std::fmt::Display;

use super::{
    expression_type::ExpressionType, func_traits::VariableFunction, polynomial::Polynomial,
};
use crate::math::function_type::FunctionType;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub function: FunctionType,
    pub input: ExpressionType,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}({})", self.function.to_string(), self.input.to_string()).as_str())
    }
}

impl Expression {
    pub fn new() -> Expression {
        Expression {
            function: FunctionType::Constant,
            input: ExpressionType::Constant(1.0),
        }
    }

    pub fn new_from(expr: &Vec<Expression>) -> Expression {
        Expression {
            function: FunctionType::Constant,
            input: ExpressionType::MultipliedFunction(expr.clone()),
        }
    }

    pub fn is_constant(&self) -> bool {
        self.input.is_constant()
    }
}

impl VariableFunction for Expression {
    fn evaluate(&self, x: f64) -> f64 {
        let input_value = x;

        match &self.input {
            ExpressionType::Constant(value) => self.function.evaluate(*value),
            ExpressionType::Functions(value) => self.function.evaluate(
                value
                    .into_iter()
                    .map(|expr| expr.evaluate(input_value))
                    .reduce(|prev, curr| prev + curr)
                    .unwrap(),
            ),
            ExpressionType::MultipliedFunction(value) => self.function.evaluate(
                value
                    .into_iter()
                    .map(|expr| expr.evaluate(input_value))
                    .reduce(|prev, curr| prev * curr)
                    .unwrap(),
            ),
            ExpressionType::DivFunction(num, den) => {
                self.function.evaluate(num.evaluate(x) / den.evaluate(x))
            }
            ExpressionType::Polynomial(value) => {
                self.function.evaluate(value.evaluate(input_value))
            }
        }
    }

    fn derivative(&self) -> Self {
        // Derivative depends on what the input for the function is:
        // If the input is constucted independent expression on it's own,
        // we convert it into a multiplied function
        // else, we return as constant
        match &self.input {
            ExpressionType::Constant(_value) => Expression {
                function: self.function.derivative(),
                input: self.input.derivative(),
            },
            ExpressionType::MultipliedFunction(ref value) => Expression {
                function: FunctionType::Constant,
                input: ExpressionType::Functions(
                    value
                        .iter()
                        .filter(|c| !c.is_constant())
                        .map(|c| {
                            ExpressionType::MultipliedFunction(
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
            },
            ExpressionType::Functions(ref value) => Expression {
                function: FunctionType::Constant,
                input: ExpressionType::Functions(
                    value
                        .iter()
                        .filter(|c| !c.is_constant())
                        .map(|c| c.derivative())
                        .collect::<Vec<Expression>>(),
                ),
            },
            ExpressionType::DivFunction(num, den) => Expression {
                function: self.function.derivative(),
                input: ExpressionType::Constant(1.0),
            },
            ExpressionType::Polynomial(ref value) => Expression {
                function: FunctionType::Constant,
                input: ExpressionType::MultipliedFunction(vec![
                    Expression {
                        function: FunctionType::Constant,
                        input: self.input.derivative(),
                    },
                    Expression {
                        function: self.function.derivative(),
                        input: self.input.clone(),
                    },
                ]),
            },
            _ => Expression {
                function: FunctionType::Constant,
                input: ExpressionType::Constant(1.0),
            },
        }
        // Expression {
        //     function: FunctionType::Constant,
        //     input: ExpressionType::Constant(1.0),
        // }
    }
}
