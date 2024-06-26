use nalgebra::{DMatrix, Dyn, Matrix, VecStorage};
use petgraph::visit::{EdgeRef, IntoNodeReferences};

use crate::Circuit;

pub fn dc_run(circuit: &Circuit) -> Matrix<f32, Dyn, Dyn, VecStorage<f32, Dyn, Dyn>> {
    let size = circuit.node_count() - circuit.node_weights().filter(|x| **x == true).count();
    let mut resistances: Vec<f32> = vec![0.0; size * size];
    let mut currents: Vec<f32> = vec![0.0; size];

    for (node, is_ground) in circuit.node_references() {
        if *is_ground {
            continue;
        }

        for edge in circuit.edges(node) {
            let element = edge.weight();
            let source_index = edge.source().index();
            let target_index = edge.target().index();

            let source_polarity = element
                .terminals()
                .iter()
                .find(|x| x.node == edge.source())
                .unwrap()
                .polarity
                .sign();

            currents[source_index] += source_polarity * element.dc_current();
            currents[source_index] +=
                source_polarity * element.dc_voltage() * element.conductance();

            resistances[source_index * size + source_index] += element.conductance();

            if *circuit.node_weight(edge.target()).unwrap() == false {
                resistances[target_index + source_index * size] -= element.conductance();
            }
        }
    }

    let resistance_matrix = DMatrix::from_vec(size, size, resistances);
    let current_matrix = DMatrix::from_vec(size, 1, currents);

    resistance_matrix.try_inverse().unwrap() * current_matrix
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use petgraph::graph::UnGraph;

    use crate::{
        elements::{
            dc_current_source::DCCurrentSource, dc_voltage_source::DCVoltageSource,
            resistor::Resistor, Element,
        },
        runners::dc::dc_run,
        Circuit,
    };

    /// One node circuit with a 10V voltage source connected to a 2Ω resistor.
    #[test]
    fn voltage_source() {
        let mut graph = Circuit(UnGraph::<bool, Box<dyn Element>>::new_undirected());
        let v1 = graph.add_node(false);
        let v2 = graph.add_node(true);
        graph.add_edge(v1, v2, Box::new(Resistor::new(2.0, v1, v2)));
        graph.add_edge(v1, v2, Box::new(DCVoltageSource::new(10.0, v1, v2)));

        let matrix = dc_run(&graph);

        assert_relative_eq!(matrix[0], 10.0, epsilon = 0.01); // v1
    }

    /// One node circuit with a 10A current source connected to a 2Ω resistor.
    #[test]
    fn current_source() {
        let mut graph = Circuit(UnGraph::<bool, Box<dyn Element>>::new_undirected());
        let v1 = graph.add_node(false);
        let v2 = graph.add_node(true);
        graph.add_edge(v1, v2, Box::new(Resistor::new(2.0, v1, v2)));
        graph.add_edge(v1, v2, Box::new(DCCurrentSource::new(10.0, v1, v2)));

        let matrix = dc_run(&graph);

        assert_relative_eq!(matrix[0], 20.0, epsilon = 0.01); // v1
    }

    /// Both current and voltage sources in a 3 node circuit.
    #[test]
    fn mixed_sources() {
        let mut graph = Circuit(UnGraph::<bool, Box<dyn Element>>::new_undirected());
        let v1 = graph.add_node(false);
        let v2 = graph.add_node(false);
        let v3 = graph.add_node(false);
        let v4 = graph.add_node(true);
        graph.add_edge(v1, v4, Box::new(Resistor::new(2.0, v1, v4)));
        graph.add_edge(v1, v4, Box::new(Resistor::new(4.0, v1, v4)));
        graph.add_edge(v1, v2, Box::new(DCVoltageSource::new(10.0, v2, v1)));
        graph.add_edge(v2, v4, Box::new(Resistor::new(6.0, v2, v4)));
        graph.add_edge(v2, v3, Box::new(Resistor::new(2.0, v2, v3)));
        graph.add_edge(v3, v4, Box::new(DCCurrentSource::new(3.0, v3, v4)));

        let matrix = dc_run(&graph);

        assert_relative_eq!(matrix[0], 1.45, epsilon = 0.01); // v1
        assert_relative_eq!(matrix[1], 11.46, epsilon = 0.01); // v2
        assert_relative_eq!(matrix[2], 17.46, epsilon = 0.01); // v3
    }
}
