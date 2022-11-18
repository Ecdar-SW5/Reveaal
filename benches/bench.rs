use std::intrinsics::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use reveaal::tests::refinement::Helper::json_refinement_check;

pub mod flamegraph;
use flamegraph::flamegraph_profiler::FlamegraphProfiler;
use reveaal::tests::ClockReduction::benchmark::{with_clock_reduction, without_clock_reduction};

static PATH: &str = "samples/json/EcdarUniversity";

fn bench_refinement(c: &mut Criterion, query: &str) {
    c.bench_function(query, |b| {
        b.iter(|| {
            assert!(json_refinement_check(PATH, &format!("refinement: {query}")));
        })
    });
}

fn bench_non_refinement(c: &mut Criterion, query: &str) {
    c.bench_function(&format!("NOT {query}"), |b| {
        b.iter(|| {
            assert!(!json_refinement_check(
                PATH,
                &format!("refinement: {query}")
            ));
        })
    });
}

fn bench_self_refinement(c: &mut Criterion, query: &str) {
    bench_refinement(c, &format!("{query} <= {query}"));
}

fn self_refinement(c: &mut Criterion) {
    bench_self_refinement(c, "Adm2");
    bench_self_refinement(c, "Administration");
    bench_self_refinement(c, "HalfAdm1");
    bench_self_refinement(c, "HalfAdm2");
    bench_self_refinement(c, "Machine");
    bench_self_refinement(c, "Machine3");
    bench_self_refinement(c, "Researcher");
    bench_self_refinement(c, "Spec");

    bench_self_refinement(c, "Administration || Researcher || Machine");
}

fn refinement(c: &mut Criterion) {
    bench_refinement(c, "Researcher <= Spec // Administration // Machine");
    bench_refinement(c, "Machine <= Spec // Administration // Researcher");
    bench_refinement(c, "Administration <= Spec // Researcher // Machine");
    bench_refinement(c, "Administration || Researcher <= Spec // Machine");
    bench_refinement(c, "Researcher || Machine <= Spec // Administration");
    bench_refinement(c, "Machine || Administration <= Spec // Researcher");
}

fn not_refinement(c: &mut Criterion) {
    bench_non_refinement(c, "Adm2 <= Spec // Researcher // Machine");
    bench_non_refinement(c, "Machine <= Spec // Adm2 // Researcher");
    bench_non_refinement(c, "Adm2 || Researcher <= Spec // Machine");
}

fn clock_reduction(c: &mut Criterion){
    c.bench_function("With Clock Reduction", |b| b.iter(|| with_clock_reduction(black_box(4))));
    c.bench_function("Without Clock Reduction", |b| b.iter(|| without_clock_reduction(black_box(4))));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(FlamegraphProfiler::new(100));
    targets = self_refinement, refinement, not_refinement,
}

criterion_main!(benches);
