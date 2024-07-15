use nalgebra::Complex;
use petgraph::graph::NodeIndex;

pub mod ac_volatage_source;
pub mod capacitor;
pub mod dc_current_source;
pub mod dc_voltage_source;
pub mod inductor;
pub mod resistor;

// f16 epsilon
pub const MIN_RESISTANCE: f32 = 9.7656E-4_f32;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    Positive,
    #[default]
    Neutral,
    Negative,
}

impl Polarity {
    pub fn sign(&self) -> f32 {
        match self {
            Self::Positive => 1.0,
            Self::Neutral => 1.0,
            Self::Negative => -1.0,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Terminal {
    pub node: NodeIndex,
    pub polarity: Polarity,
}

impl Terminal {
    pub fn new(node: NodeIndex, polarity: Polarity) -> Self {
        Self { node, polarity }
    }
}

pub trait Element {
    fn terminals(&self) -> &[Terminal];

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
