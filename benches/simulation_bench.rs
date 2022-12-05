use criterion::{criterion_group, criterion_main, Criterion};
pub mod flamegraph;
use flamegraph::flamegraph_profiler::FlamegraphProfiler;
use reveaal::{
    tests::Simulation::helper,
    DataReader::component_loader::ModelCache,
    ProtobufServer::{
        services::{SimulationStartRequest, SimulationStepRequest},
        ConcreteEcdarBackend,
    },
};
use tonic::Response;

static PATH: &str = "samples/json/EcdarUniversity";

fn create_step_request(
    component_names: &[&str],
    components_path: &str,
    composition: &str,
    last_response: &SimulationStartRequest,
) -> SimulationStepRequest {
    let cache = ModelCache::default();
    helper::create_step_request(
        component_names,
        components_path,
        composition,
        ConcreteEcdarBackend::handle_start_simulation(last_response.clone(), cache)
            .map(Response::new),
    )
}

fn start_simulation(c: &mut Criterion, id: &str, request: SimulationStartRequest) {
    let cache = ModelCache::default();
    c.bench_function(id, |b| {
        b.iter(|| ConcreteEcdarBackend::handle_start_simulation(request.to_owned(), cache.clone()))
    });
}

fn take_simulation_step(c: &mut Criterion, id: &str, request: SimulationStepRequest) {
    let cache = ModelCache::default();
    c.bench_function(id, |b| {
        b.iter(|| ConcreteEcdarBackend::handle_take_simulation_step(request.clone(), cache.clone()))
    });
}

fn simulation(c: &mut Criterion) {
    let start_request_1 = helper::create_start_request(&["Machine"], PATH, "(Machine)");
    let start_request_2 =
        helper::create_start_request(&["HalfAdm1", "HalfAdm2"], PATH, "(HalfAdm1 && HalfAdm2)");
    let start_request_3 = helper::create_start_request(
        &["Administration", "Machine", "Researcher"],
        PATH,
        "(Administration || Machine || Researcher)",
    );
    let start_request_4 = helper::create_start_request(
        &["HalfAdm1", "HalfAdm2", "Machine", "Researcher"],
        "samples/json/EcdarUniversity",
        "((HalfAdm1 && HalfAdm2) || Machine || Researcher)",
    );

    let step_request_1 = create_step_request(&["Machine"], PATH, "(Machine)", &start_request_1);
    let step_request_2 = create_step_request(
        &["HalfAdm1", "HalfAdm2"],
        PATH,
        "(HalfAdm1 && HalfAdm2)",
        &start_request_2,
    );
    let step_request_3 = create_step_request(
        &["Administration", "Machine", "Researcher"],
        PATH,
        "(Administration || Machine || Researcher)",
        &start_request_3,
    );
    let step_request_4 = create_step_request(
        &["HalfAdm1", "HalfAdm2", "Machine", "Researcher"],
        "samples/json/EcdarUniversity",
        "((HalfAdm1 && HalfAdm2) || Machine || Researcher)",
        &start_request_4,
    );

    start_simulation(c, "start simulation for (Machine)", start_request_1);
    start_simulation(
        c,
        "start simulation for (HalfAdm1 && HalfAdm2)",
        start_request_2,
    );
    start_simulation(
        c,
        "start simulation for (Administration || Machine || Researcher)",
        start_request_3,
    );
    start_simulation(
        c,
        "start simulation for ((HalfAdm1 && HalfAdm2) || Machine || Researcher)",
        start_request_4,
    );

    take_simulation_step(c, "take simulation step for (Machine)", step_request_1);
    take_simulation_step(
        c,
        "take simulation step for (HalfAdm1 && HalfAdm2)",
        step_request_2,
    );
    take_simulation_step(
        c,
        "take simulation step for (Administration || Machine || Researcher)",
        step_request_3,
    );
    take_simulation_step(
        c,
        "take simulation step for ((HalfAdm1 && HalfAdm2) || Machine || Researcher)",
        step_request_4,
    );
}

criterion_group! {
  name = simulation_benches;
  config = Criterion::default().with_profiler(FlamegraphProfiler::new(100));
  targets = simulation
}

criterion_main!(simulation_benches);
