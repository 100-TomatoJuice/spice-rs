use nalgebra::Complex;

use crate::NodeId;

use super::{dc_voltage_source::DCVoltageSource, Element, Terminal};

#[derive(Clone, Copy)]
pub struct Inductor {
    inductance: f32,
    terminals: [Terminal; 2],
    index: usize,
}

impl Inductor {
    pub fn new(
        inductance: f32,
        positive_node: NodeId,
        negative_node: NodeId,
        index: usize,
    ) -> Self {
        Self {
            inductance,
            terminals: [
                Terminal::new(positive_node, super::Polarity::Positive),
                Terminal::new(negative_node, super::Polarity::Negative),
            ],
            index,
        }
    }
}

impl Element for Inductor {
    fn terminals(&self) -> &[Terminal] {
        &self.terminals
    }

    fn stamp(&self, a_matrix: &mut Vec<f32>, z_vector: &mut Vec<f32>, n: usize, m: usize) {
        let nodes: Vec<NodeId> = self.terminals().iter().map(|x| x.node).collect();
        DCVoltageSource::new(0.0, nodes[0], nodes[1], self.index).stamp(a_matrix, z_vector, n, m);
    }

    fn is_b_c_element(&self) -> bool {
        true
    }

    fn dc_voltage(&self) -> f32 {
        0.0
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
        0.0
    }

    /// The rectangular impedence of an inductor is equal to `0 + jωL`,
    /// where `L` is the inductance in Henries and `ω` is the frequency in Hertz.
    fn impedance(&self, frequency: f32) -> Complex<f32> {
        Complex::new(0.0, self.inductance * frequency)
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Complex;

    use crate::{elements::Element, NodeId};

    use super::Inductor;

    /// Test if the impedance for the inductor is correctly calculated.
    #[test]
    fn impedance() {
        let inductor = Inductor::new(10.0, NodeId(0), NodeId(1), 0);
        assert_eq!(
            inductor.impedance(1000.0),
            Complex::<f32>::new(0.0, 10000.0)
        )
    }
}
