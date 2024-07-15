use nalgebra::Complex;
use petgraph::graph::NodeIndex;

use super::{Element, Terminal};

#[derive(Default, Clone)]
pub struct DCCurrentSource {
    current: f32,
    terminals: [Terminal; 2],
    resistance: f32,
}

impl DCCurrentSource {
    pub fn new(current: f32, positive_node: NodeIndex, negative_node: NodeIndex) -> Self {
        Self {
            current,
            terminals: [
                Terminal::new(positive_node, super::Polarity::Positive),
                Terminal::new(negative_node, super::Polarity::Negative),
            ],
            resistance: 0.0,
        }
    }

    #[must_use]
    pub fn with_resistance(mut self, resistance: f32) -> Self {
        self.resistance = resistance;
        self
    }
}

impl Element for DCCurrentSource {
    fn terminals(&self) -> &[Terminal] {
        &self.terminals
    }

    fn dc_voltage(&self) -> f32 {
        0.0
    }

    fn ac_voltage(&self) -> Complex<f32> {
        Complex::ZERO
    }

    fn dc_current(&self) -> f32 {
        self.current
    }

    fn ac_current(&self) -> Complex<f32> {
        Complex::ZERO
    }

    fn resistance(&self) -> f32 {
        self.resistance
    }

    fn impedance(&self, _frequency: f32) -> Complex<f32> {
        Complex::ZERO
    }
}
