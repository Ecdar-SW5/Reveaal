use criterion::{criterion_group, criterion_main};
use reveaal::TransitionSystems::CompiledComponent;
use tests::ClockReduction::helper::test::read_json_component_and_process;



pub(crate) fn bench_heap_clock_reduction(){
    let component = read_json_component_and_process("samples/json/ClockReductionTests/RedundantClocks");
    let dim = component.declarations.clocks.len() + 1;
    let compiled_component = CompiledComponent::compile(component.clone(), dim).unwrap();




}

criterion_group!(benches, bench_heap_clock_reduction);
criterion_main!(benches);