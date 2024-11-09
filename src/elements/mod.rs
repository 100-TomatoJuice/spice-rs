use std::{any::Any, fmt::Debug};

use dyn_clone::DynClone;
use nalgebra::Complex;

use crate::NodeId;

pub mod ac_volatage_source;
pub mod capacitor;
pub mod dc_current_source;
pub mod dc_voltage_source;
pub mod inductor;
pub mod resistor;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    Positive,
    #[default]
    Neutral,
    Negative,
}

impl Polarity {
    pub fn sign(&self) -> f32 {
        match self {
            Self::Positive | Self::Neutral => 1.0,
            Self::Negative => -1.0,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Terminal {
    pub node: NodeId,
    pub polarity: Polarity,
}

impl Terminal {
    pub fn new(node: NodeId, polarity: Polarity) -> Self {
        Self { node, polarity }
    }

    pub fn new_positive(node: NodeId) -> Self {
        Self::new(node, Polarity::Positive)
    }

    pub fn new_neutral(node: NodeId) -> Self {
        Self::new(node, Polarity::Neutral)
    }

    pub fn new_negative(node: NodeId) -> Self {
        Self::new(node, Polarity::Negative)
    }

    pub fn sign(&self) -> f32 {
        self.polarity.sign()
    }
}

pub trait Element: Any + DynClone + Debug {
    fn terminals(&self) -> &[Terminal];

    /// "Stamp" the circuit elements' influence onto the
    /// `a_matrix` and `z_vector`.
    ///
    /// The `a_matrix` consists of 4 different matrices: G, B, C, D.
    /// These are ordered in like so:
    ///
    /// `[G B]`\
    /// `[C D]`
    ///
    /// * `a_matrix` - Consists of all the passive elements in the circuit.
    /// * `z_vector` - Consists of all the active elements in the circuit.
    /// * `n` - Number of nodes in the circuit.
    /// * `m` - Number of independent voltage sources.
    fn stamp(&self, a_matrix: &mut Vec<f32>, z_vector: &mut Vec<f32>, n: usize, m: usize);

    /// Does this element stamp itself onto the B or C matrices?
    fn is_b_c_element(&self) -> bool {
        false
    }

    fn dc_voltage(&self) -> f32;
    fn ac_voltage(&self) -> Complex<f32>;

    fn dc_current(&self) -> f32;
    fn ac_current(&self) -> Complex<f32>;

    fn resistance(&self) -> f32;
    fn conductance(&self) -> f32 {
        let resistance = self.resistance();
        if resistance == 0.0 {
            return 0.0;
        }

        resistance.recip()
    }

    fn impedance(&self, frequency: f32) -> Complex<f32>;
    fn admittance(&self, frequency: f32) -> Complex<f32> {
        let impedance = self.impedance(frequency);
        if impedance == Complex::ZERO {
            return Complex::ZERO;
        }

        impedance.inv()
    }
}

dyn_clone::clone_trait_object!(Element);
