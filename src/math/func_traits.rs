pub trait VariableFunction {
    fn evaluate(&self, x: f64) -> f64;

    fn derivative(&self) -> Self;
}
