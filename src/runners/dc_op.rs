use nalgebra::{DMatrix, DVector, Dyn, VecStorage, Vector, U1};

use crate::Circuit;

/// DC Operating Point to calculate the steady state of a circuit.
pub fn dc_op(circuit: &Circuit) -> Vector<f32, Dyn, VecStorage<f32, Dyn, U1>> {
    let n = circuit.node_count();
    let m = circuit
        .elements()
        .iter()
        .filter(|x| x.is_b_c_element())
        .count();
    let z_size = n - 1 + m;
    let a_size = z_size * z_size;

    let mut a_matrix: Vec<f32> = vec![0.0; a_size];
    let mut z_vector: Vec<f32> = vec![0.0; z_size];

    for element in circuit.elements().iter() {
        element.stamp(&mut a_matrix, &mut z_vector, n, m);
    }

    let a_matrix = DMatrix::from_vec(z_size, z_size, a_matrix);
    let z_vector = DVector::from_vec(z_vector);

    a_matrix.try_inverse().unwrap() * z_vector
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::{
        elements::{
            capacitor::Capacitor, dc_current_source::DCCurrentSource,
            dc_voltage_source::DCVoltageSource, inductor::Inductor, resistor::Resistor,
        },
        runners::dc_op::dc_op,
        Circuit,
    };

    /// One node circuit with a 10V voltage source connected to a 2Ω resistor.
    #[test]
    fn voltage_source() {
        let mut circuit = Circuit::default();
        let v0 = circuit.add_node();
        let v1 = circuit.add_node();
        circuit.add_element(Box::new(Resistor::new(2.0, v1, v0)));
        circuit.add_element(Box::new(DCVoltageSource::new(10.0, v1, v0, 0)));

        let matrix = dc_op(&circuit);

        assert_eq!(matrix.len(), 2);
        assert_relative_eq!(matrix[0], 10.0, epsilon = 0.01); // v1
        assert_relative_eq!(matrix[1], -5.0, epsilon = 0.01); // i_v_source
    }

    /// One node circuit with a 10A current source connected to a 2Ω resistor.
    #[test]
    fn current_source() {
        let mut circuit = Circuit::default();
        let v0 = circuit.add_node();
        let v1 = circuit.add_node();
        circuit.add_element(Box::new(Resistor::new(2.0, v1, v0)));
        circuit.add_element(Box::new(DCCurrentSource::new(10.0, v0, v1)));

        let matrix = dc_op(&circuit);

        assert_eq!(matrix.len(), 1);
        assert_relative_eq!(matrix[0], -20.0, epsilon = 0.01); // v1
    }

    /// Both current and voltage sources in a 2 node circuit.
    #[test]
    fn mixed_sources() {
        let mut circuit = Circuit::default();
        let v0 = circuit.add_node();
        let v1 = circuit.add_node();
        let v2 = circuit.add_node();
        circuit.add_element(Box::new(DCVoltageSource::new(10.0, v1, v2, 0)));
        circuit.add_element(Box::new(Resistor::new(2.0, v1, v0)));
        circuit.add_element(Box::new(Resistor::new(4.0, v1, v2)));
        circuit.add_element(Box::new(Resistor::new(2.0, v2, v0)));
        circuit.add_element(Box::new(DCCurrentSource::new(3.0, v1, v0)));

        let matrix = dc_op(&circuit);

        assert_eq!(matrix.len(), 3);
        assert_relative_eq!(matrix[0], 8.0, epsilon = 0.01); // v1
        assert_relative_eq!(matrix[1], -2.0, epsilon = 0.01); // v2
        assert_relative_eq!(matrix[2], -3.5, epsilon = 0.01); // i_v_source
    }

    /// A voltage source, resistor, and capacitor connected in series.
    #[test]
    fn capacitor() {
        let mut circuit = Circuit::default();
        let v0 = circuit.add_node();
        let v1 = circuit.add_node();
        let v2 = circuit.add_node();
        circuit.add_element(Box::new(DCVoltageSource::new(10.0, v1, v0, 0)));
        circuit.add_element(Box::new(Resistor::new(10.0, v1, v2)));
        circuit.add_element(Box::new(Capacitor::new(1.0, v2, v0)));

        let matrix = dc_op(&circuit);

        assert_eq!(matrix.len(), 3);
        assert_relative_eq!(matrix[0], 10.0, epsilon = 0.01); // v1
        assert_relative_eq!(matrix[1], 10.0, epsilon = 0.01); // v2
        assert_relative_eq!(matrix[2], 0.0, epsilon = 0.01); // i_v_source
    }

    /// A voltage source, resistor, and inductor connected in series.
    #[test]
    fn inductor() {
        let mut circuit = Circuit::default();
        let v0 = circuit.add_node();
        let v1 = circuit.add_node();
        let v2 = circuit.add_node();
        circuit.add_element(Box::new(DCVoltageSource::new(10.0, v1, v0, 0)));
        circuit.add_element(Box::new(Resistor::new(10.0, v1, v2)));
        circuit.add_element(Box::new(Inductor::new(1.0, v2, v0, 1)));

        let matrix = dc_op(&circuit);

        assert_eq!(matrix.len(), 4);
        assert_relative_eq!(matrix[0], 10.0, epsilon = 0.01); // v1
        assert_relative_eq!(matrix[1], 0.0, epsilon = 0.01); // v2
        assert_relative_eq!(matrix[2], -1.0, epsilon = 0.01); // i_v_source
        assert_relative_eq!(matrix[3], 1.0, epsilon = 0.01); // i_inductor
    }
}
