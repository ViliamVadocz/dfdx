use super::traits::Optimizer;
use crate::gradients::GradientTape;
use crate::prelude::{Tensor0D, TensorWithTape, WithTape};

#[derive(Debug)]
pub struct Sgd {
    pub lr: f32,
}

impl Default for Sgd {
    fn default() -> Self {
        Self { lr: 1e-2 }
    }
}

impl Optimizer for Sgd {
    fn compute_gradients(&mut self, loss: Tensor0D<WithTape>) -> Box<GradientTape> {
        let mut gradients = loss.backward();
        gradients.scale(self.lr);
        gradients
    }
}