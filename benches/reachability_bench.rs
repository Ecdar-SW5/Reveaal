use criterion::{criterion_group, criterion_main, Criterion};
use reveaal::tests::refinement::Helper::json_run_query;
pub mod flamegraph;
use flamegraph::flamegraph_profiler::FlamegraphProfiler;

static PATH: &str = "samples/json/EcdarUniversity";

fn bench_reachability(c: &mut Criterion, query: &str) {
    c.bench_function(query, |b| b.iter(|| json_run_query(PATH, query)));
}

fn reachability_benchmarking(c: &mut Criterion) {
    bench_reachability(
        c,
        "reachability: Machine || Researcher -> [L5, L6](); [L4, L9]()",
    );
    bench_reachability(
        c,
        "reachability: Administration || Machine || Researcher -> [L3, L5, L9](); [L0, L5, U0]()",
    );
    bench_reachability(
        c,
        "reachability: Administration || Machine || Researcher -> [L0, L5, U0](); [L3, L5, L9]()",
    );
    bench_reachability(c, "reachability: Machine -> [L5](y<6); [L4](y<=6)");
    bench_reachability(c, "reachability: Machine -> [L5](); [L4](y>7)");
    bench_reachability(c, "reachability: Machine -> [L4](y<=6); [L5](y>=4)");
    bench_reachability(c, "reachability: Machine -> [L5](y<1); [L5](y<2)");
    bench_reachability(c, "reachability: Machine -> [L5](); [L5]()");
    bench_reachability(
        c,
        "reachability: Machine || Researcher -> [L5, U0](); [L5, L7]()",
    );
    bench_reachability(c, "reachability: Researcher -> [U0](); [L7]()");
}

criterion_group! {
  name = reachability_benches;
  config = Criterion::default().with_profiler(FlamegraphProfiler::new(100));
  targets = reachability_benchmarking
}

criterion_main!(reachability_benches);
