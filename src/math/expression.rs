use std::fmt::Display;

use super::{
    expression_type::ExpressionType, func_traits::VariableFunction,
};
use crate::math::function_type::FunctionType;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub function: FunctionType,
    pub input: ExpressionType,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.function {
            FunctionType::None => f.write_str(format!("{}", self.input.to_string()).as_str()),
            _ => f.write_str(
                format!("{}({})", self.function.to_string(), self.input.to_string()).as_str(),
            ),
        }
    }
}

impl Expression {
    pub fn new() -> Expression {
        Expression {
            function: FunctionType::None,
            input: ExpressionType::Constant(1.0),
        }
    }

    pub fn new_from(expr: &Vec<Expression>) -> Expression {
        Expression {
            function: FunctionType::None,
            input: ExpressionType::MultipliedExpressions(expr.clone()),
        }
    }

    pub fn is_constant(&self) -> bool {
        self.input.is_constant()
    }

    pub fn simplify(&self) {
        
    }
}

impl VariableFunction for Expression {
    fn evaluate(&self, x: f64) -> f64 {
        let input_value = x;

        match &self.input {
            ExpressionType::Constant(value) => self.function.evaluate(*value),
            ExpressionType::Expressions(value) => self.function.evaluate(
                value
                    .into_iter()
                    .map(|expr| expr.evaluate(input_value))
                    .reduce(|prev, curr| prev + curr)
                    .unwrap(),
            ),
            ExpressionType::MultipliedExpressions(value) => self.function.evaluate(
                value
                    .into_iter()
                    .map(|expr| expr.evaluate(input_value))
                    .reduce(|prev, curr| prev * curr)
                    .unwrap(),
            ),
            ExpressionType::DividedExpressions(num, den) => {
                self.function.evaluate(num.evaluate(x) / den.evaluate(x))
            }
            ExpressionType::Polynomial(value) => {
                self.function.evaluate(value.evaluate(input_value))
            }
        }
    }

    fn derivative(&self) -> Self {
        // Derivative depends on what the input for the function is:
        // If the input is constucted that contains
        // independent expression,
        // we convert it into a multiplied function
        // else, we return as constant
        match &self.input {
            ExpressionType::Constant(_value) => Expression {
                function: self.function.derivative(),
                input: self.input.derivative(),
            },
            ExpressionType::MultipliedExpressions(ref value) => Expression {
                function: FunctionType::None,
                input: ExpressionType::Expressions(
                    value
                        .iter()
                        .filter(|c| !c.is_constant())
                        .map(|c| {
                            ExpressionType::MultipliedExpressions(
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
            },
            ExpressionType::Expressions(ref value) => Expression {
                function: FunctionType::None,
                input: ExpressionType::Expressions(
                    value
                        .iter()
                        .filter(|c| !c.is_constant())
                        .map(|c| c.derivative())
                        .collect::<Vec<Expression>>(),
                ),
            },
            ExpressionType::DividedExpressions(ref num, ref den) => Expression {
                function: FunctionType::None,
                input: ExpressionType::DividedExpressions(
                    Box::new(Expression {
                        function: FunctionType::None,
                        input: ExpressionType::Expressions(vec![
                            Expression {
                                function: FunctionType::None,
                                input: ExpressionType::MultipliedExpressions(vec![
                                    num.derivative(),
                                    *den.clone(),
                                ]),
                            },
                            Expression {
                                function: FunctionType::None,
                                input: ExpressionType::MultipliedExpressions(vec![
                                    den.derivative(),
                                    *num.clone(),
                                    Expression {
                                        function: FunctionType::None,
                                        input: ExpressionType::Constant(-1.0f64),
                                    },
                                ]),
                            },
                        ]),
                    }),
                    Box::new(Expression {
                        function: FunctionType::None,
                        input: ExpressionType::MultipliedExpressions(vec![
                            *den.clone(),
                            *den.clone(),
                        ]),
                    }),
                ),
            },
            ExpressionType::Polynomial(_) => Expression {
                function: FunctionType::None,
                input: ExpressionType::MultipliedExpressions(vec![
                    Expression {
                        function: FunctionType::None,
                        input: self.input.derivative(),
                    },
                    Expression {
                        function: self.function.derivative(),
                        input: self.input.clone(),
                    },
                ]),
            },
        }
        // Expression {
        //     function: FunctionType::Constant,
        //     input: ExpressionType::Constant(1.0),
        // }
    }
}
