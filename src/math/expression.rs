use crate::math::function_type::FunctionType;

use super::{func_traits::VariableFunction, polynomial::Polynomial};

#[derive(Clone)]
pub enum ExpressionType {
    Functions(Vec<Expression>),
    Constant(f64),
    Polynomial(Polynomial<f64>),
    MultipliedFunction(Vec<Expression>),
    DivFunction(Box<Expression>, Box<Expression>),
}

#[derive(Clone)]
pub struct Expression {
    pub function: FunctionType,
    pub input: ExpressionType,
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
            input: ExpressionType::Functions(expr.clone()),
        }
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
                    .collect::<Vec<f64>>()
                    .into_iter()
                    .reduce(|prev, curr| prev + curr)
                    .unwrap(),
            ),
            ExpressionType::MultipliedFunction(value) => self.function.evaluate(
                value
                    .into_iter()
                    .map(|expr| expr.evaluate(input_value))
                    .collect::<Vec<f64>>()
                    .into_iter()
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
        match &self.input {
            ExpressionType::Constant(_value) => Expression {
                function: FunctionType::Constant,
                input: ExpressionType::Constant(0.0),
            },
            ExpressionType::MultipliedFunction(ref value) => {
                let mut derivative: Vec<Expression> = Vec::new();
                for index in 0..value.len() {
                    let mut der: Vec<Expression> = Vec::new();
                    for i in 0..value.len() {
                        if index == i {
                            der.push(value[index].derivative());
                        } else {
                            der.push(value[index].clone());
                        }
                    }

                    derivative.push({
                        Expression {
                            function: value[index].function.clone(),
                            input: ExpressionType::MultipliedFunction(der),
                        }
                    })
                }

                return Expression {
                    function: self.function.clone(),
                    input: ExpressionType::Functions(derivative),
                };
            }
            ExpressionType::Functions(ref value) => Expression {
                function: FunctionType::Constant,
                input: ExpressionType::Constant(1.0),
            },
            ExpressionType::DivFunction(num, den) => Expression {
                function: FunctionType::Constant,
                input: ExpressionType::Constant(1.0),
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
