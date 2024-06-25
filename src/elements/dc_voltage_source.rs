use nalgebra::Complex;
use petgraph::graph::NodeIndex;

use super::{Element, Terminal};

// f16 epsilon
const DEFAULT_DC_SOURCE_RESISTANCE: f32 = 9.7656E-4_f32;

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
            resistance: DEFAULT_DC_SOURCE_RESISTANCE,
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

    fn impedance(&self) -> Complex<f32> {
        Complex::ZERO
    }
}
