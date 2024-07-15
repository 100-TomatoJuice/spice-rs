use nalgebra::Complex;
use petgraph::graph::NodeIndex;

use super::{Element, Terminal};

// f16 epsilon
const DEFAULT_AC_SOURCE_RESISTANCE: Complex<f32> = Complex::new(9.7656E-4_f32, 0.0);

#[derive(Default, Clone)]
pub struct AcVoltageSource {
    voltage: Complex<f32>,
    terminals: [Terminal; 2],
    impedance: Complex<f32>,
}

impl AcVoltageSource {
    pub fn new(voltage: Complex<f32>, positive_node: NodeIndex, negative_node: NodeIndex) -> Self {
        Self {
            voltage,
            terminals: [
                Terminal::new(positive_node, super::Polarity::Positive),
                Terminal::new(negative_node, super::Polarity::Negative),
            ],
            impedance: DEFAULT_AC_SOURCE_RESISTANCE,
        }
    }

    #[must_use]
    pub fn with_impedance(mut self, impedance: Complex<f32>) -> Self {
        self.impedance = impedance;
        self
    }
}

impl Element for AcVoltageSource {
    fn terminals(&self) -> &[Terminal] {
        &self.terminals
    }

    fn dc_voltage(&self) -> f32 {
        0.0
    }

    fn ac_voltage(&self) -> Complex<f32> {
        self.voltage
    }

    fn dc_current(&self) -> f32 {
        0.0
    }

    fn ac_current(&self) -> Complex<f32> {
        Complex::ZERO
    }

    fn resistance(&self) -> f32 {
        0.0
    }

    fn impedance(&self, _frequency: f32) -> Complex<f32> {
        self.impedance
    }
}
