use criterion::{black_box, criterion_group, criterion_main, Criterion};
use spice_rs::{
    elements::{
        dc_current_source::DCCurrentSource, dc_voltage_source::DCVoltageSource, resistor::Resistor,
    },
    runners::dc_op::dc_op,
    Circuit,
};

fn criterion_benchmark(c: &mut Criterion) {
    let mut graph = Circuit::default();
    let v0 = graph.push_node();
    let v1 = graph.push_node();
    let v2 = graph.push_node();
    let v3 = graph.push_node();
    graph.add_element(Box::new(Resistor::new(2.0, v1, v0)));
    graph.add_element(Box::new(Resistor::new(4.0, v1, v0)));
    graph.add_element(Box::new(DCVoltageSource::new(10.0, v2, v1, 0)));
    graph.add_element(Box::new(Resistor::new(6.0, v2, v0)));
    graph.add_element(Box::new(Resistor::new(2.0, v2, v3)));
    graph.add_element(Box::new(DCCurrentSource::new(3.0, v3, v0)));

    c.bench_function("dc_run", |b| b.iter(|| dc_op(black_box(&graph))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
