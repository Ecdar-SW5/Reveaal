use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reveaal::extract_system_rep::create_executable_query;
use reveaal::parse_queries::parse_to_query;
use reveaal::{JsonProjectLoader, DEFAULT_SETTINGS, TEST_SETTINGS};

const QUERY: &str = "refinement: (((((Adm2 && HalfAdm1 && HalfAdm2) || Machine || Researcher) && ((Adm2 && HalfAdm1) || Machine || Researcher) && ((Adm2 && HalfAdm2) || Machine || Researcher) && ((HalfAdm1 && HalfAdm2) || Machine || Researcher) && (Adm2 || Machine || Researcher)) // (Adm2 && HalfAdm1 && HalfAdm2)) // Researcher) <= (((((Adm2 && HalfAdm1 && HalfAdm2) || Machine || Researcher) && ((Adm2 && HalfAdm1) || Machine || Researcher) && ((Adm2 && HalfAdm2) || Machine || Researcher) && ((HalfAdm1 && HalfAdm2) || Machine || Researcher) && (Adm2 || Machine || Researcher)) // (Adm2 && HalfAdm1 && HalfAdm2)) // Researcher)";

fn bench_refinement(c: &mut Criterion) {
    // Set up the bench.
    let mut group = c.benchmark_group("Clock Reduction");
    group.bench_function("Refinement check - No reduction", |b| {
        b.iter(|| black_box(normal_reachability()))
    });
    group.bench_function("Refinement check - With reduction", |b| {
        b.iter(|| black_box(clock_reduced_reachability()))
    });
    group.finish();
}

fn clock_reduced_reachability() {
    let query = parse_to_query(QUERY);
    let mut loader =
        JsonProjectLoader::new("samples/json/EcdarUniversity".to_string(), DEFAULT_SETTINGS)
            .to_comp_loader();
    let executor = create_executable_query(query.get(0).unwrap(), &mut *loader).unwrap();
    executor.execute();
}

fn normal_reachability() {
    let query = parse_to_query(QUERY);
    let mut loader =
        JsonProjectLoader::new("samples/json/EcdarUniversity".to_string(), TEST_SETTINGS)
            .to_comp_loader();
    let executor = create_executable_query(query.get(0).unwrap(), &mut *loader).unwrap();
    executor.execute();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_refinement
}
criterion_main!(benches);
