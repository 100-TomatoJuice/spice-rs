use spice_rs::{
    elements::{
        dc_current_source::DCCurrentSource, dc_voltage_source::DCVoltageSource, resistor::Resistor,
        Element,
    },
    runners::dc::dc_run,
    Circuit,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use petgraph::graph::UnGraph;

fn criterion_benchmark(c: &mut Criterion) {
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

    c.bench_function("dc_run", |b| b.iter(|| dc_run(black_box(&graph))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
