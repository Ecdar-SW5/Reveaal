use criterion::{black_box, Criterion, criterion_group, criterion_main};
use reveaal::System::local_consistency::is_fully_consistent;
use reveaal::tests;
use reveaal::TransitionSystems::{CompiledComponent, TransitionSystem};
use reveaal::TransitionSystems::transition_system::{ClockReductionInstruction, Heights};
use tests::ClockReduction::helper::test::read_json_component_and_process;

/// This benchmark loads a massive component and performs a consistency check.
/// The component is then clock reduced and the consistency check is run again.

pub(crate) fn bench_clock_reduction(c: &mut Criterion){
    let component = read_json_component_and_process("samples/json/ClockReductionTest/RedundantClocks", "CombinedComponent");
    let dim = component.declarations.clocks.len() + 1;
    let compiled_component = CompiledComponent::compile(component.clone(), dim).unwrap();
    let clock_reduction_instructions = compiled_component.find_redundant_clocks(Heights::empty());
    let mut clock_reduced_component = component.clone();
    clock_reduced_component.reduce_clocks(clock_reduction_instructions);
    let reduced_dim = clock_reduced_component.declarations.clocks.len() + 1;
    let clock_reduced_compiled_component = CompiledComponent::compile(clock_reduced_component, reduced_dim).unwrap();
    let mut group = c.benchmark_group("Clock Reduction");
    group.bench_function("Consistency check - No reduction", |b|{
        b.iter(|| black_box(compiled_component.is_locally_consistent()))
    });
    group.bench_function("Consistency check - With reduction", |b| {
        b.iter(|| black_box(clock_reduced_compiled_component.is_locally_consistent()))
    });
    group.finish();
}

criterion_group!(benches, bench_clock_reduction);
criterion_main!(benches);
