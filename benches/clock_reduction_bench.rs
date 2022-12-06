use criterion::{criterion_group, criterion_main, Criterion};
use reveaal::extract_system_rep::create_executable_query;
use reveaal::parse_queries::parse_to_query;
use reveaal::{tests::TEST_SETTINGS, JsonProjectLoader, DEFAULT_SETTINGS};

const QUERY: &str = "refinement: (((((Adm2 && HalfAdm1 && HalfAdm2) || Machine || Researcher) && ((Adm2 && HalfAdm1) || Machine || Researcher) && ((Adm2 && HalfAdm2) || Machine || Researcher) && ((HalfAdm1 && HalfAdm2) || Machine || Researcher) && (Adm2 || Machine || Researcher)) // (Adm2 && HalfAdm1 && HalfAdm2)) // Researcher) <= (((((Adm2 && HalfAdm1 && HalfAdm2) || Machine || Researcher) && ((Adm2 && HalfAdm1) || Machine || Researcher) && ((Adm2 && HalfAdm2) || Machine || Researcher) && ((HalfAdm1 && HalfAdm2) || Machine || Researcher) && (Adm2 || Machine || Researcher)) // (Adm2 && HalfAdm1 && HalfAdm2)) // Researcher)";

/// This bench runs `QUERY` with and without clock reduction such that you can compare the results.
/// The bench takes about 40 min on my machine, so grab some coffee.
fn bench_clock_reduced_refinement(c: &mut Criterion) {
    // Set up the bench.
    let mut group = c.benchmark_group("Clock Reduction");
    group.bench_function("Refinement check - No reduction", |b| {
        b.iter(normal_refinement);
    });
    group.bench_function("Refinement check - With reduction", |b| {
        b.iter(clock_reduced_refinement);
    });
    group.finish();
}

fn clock_reduced_refinement() {
    let query = parse_to_query(QUERY);
    let mut loader =
        JsonProjectLoader::new("samples/json/EcdarUniversity".to_string(), DEFAULT_SETTINGS)
            .to_comp_loader();
    let executor = create_executable_query(query.get(0).unwrap(), &mut *loader).unwrap();
    executor.execute();
}

fn normal_refinement() {
    let query = parse_to_query(QUERY);
    let mut loader =
        JsonProjectLoader::new("samples/json/EcdarUniversity".to_string(), TEST_SETTINGS)
            .to_comp_loader();
    let executor = create_executable_query(query.get(0).unwrap(), &mut *loader).unwrap();
    executor.execute();
}

criterion_group! {
    name = clock_reduction_bench;
    config = Criterion::default().sample_size(10);
    targets = bench_clock_reduced_refinement
}
criterion_main!(clock_reduction_bench);
