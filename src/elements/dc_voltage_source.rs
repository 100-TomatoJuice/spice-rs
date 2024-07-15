use nalgebra::Complex;
use petgraph::graph::NodeIndex;

use super::{Element, Terminal, MIN_RESISTANCE};

#[derive(Default, Clone)]
pub struct DCVoltageSource {
    voltage: f32,
    terminals: [Terminal; 2],
    resistance: f32,
}

impl DCVoltageSource {
    pub fn new(voltage: f32, positive_node: NodeIndex, negative_node: NodeIndex) -> Self {
        Self {
            voltage,
            terminals: [
                Terminal::new(positive_node, super::Polarity::Positive),
                Terminal::new(negative_node, super::Polarity::Negative),
            ],
            resistance: MIN_RESISTANCE,
        }
    }

    #[must_use]
    pub fn with_resistance(mut self, resistance: f32) -> Self {
        self.resistance = resistance;
        self
    }
}

impl Element for DCVoltageSource {
    fn terminals(&self) -> &[Terminal] {
        &self.terminals
    }

    fn dc_voltage(&self) -> f32 {
        self.voltage
    }

    fn ac_voltage(&self) -> Complex<f32> {
        Complex::ZERO
    }

    fn dc_current(&self) -> f32 {
        0.0
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
