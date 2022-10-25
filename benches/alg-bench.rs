use criterion::{criterion_group, criterion_main, Criterion};
use reveaal::tests::refinement::Helper::json_run_query;

static PATH: &str = "samples/json/EcdarUniversity";

fn bench_reachability(c: &mut Criterion, query: &str) {
  c.bench_function(query, |b| {
      b.iter(|| {
        json_run_query(PATH, query)
      })
  });
}

fn reachability_benchmarking(c: &mut Criterion){
  bench_reachability(c, "reachability: Machine || Researcher -> [L5, L6](); [L4, L9]()");
}

criterion_group!(reachability_benches, reachability_benchmarking);
criterion_main!(reachability_benches);