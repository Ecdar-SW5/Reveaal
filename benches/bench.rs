use std::vec;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reveaal::{
    tests::refinement::Helper::json_refinement_check,
    ProtobufServer::{
        services::{
            component::Rep, ecdar_backend_server::EcdarBackend, Component, ComponentsInfo,
            QueryRequest,
        },
        ConcreteEcdarBackend,
    },
};
use tonic::Request;

use criterion::async_executor::FuturesExecutor;
use futures::stream::FuturesUnordered;
use futures::StreamExt;

mod flamegraph_profiler;
use flamegraph_profiler::FlamegraphProfiler;

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

fn send_expensive_query_same_components(c: &mut Criterion) {
    let json = vec![
        std::fs::read_to_string(format!("{}/Components/Administration.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Researcher.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Machine.json", PATH)).unwrap(),
    ];
    c.bench_function("send_expensive_query_same_components", |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let backend = ConcreteEcdarBackend::default();
            let responses = (0..64)
                .map(|_| {
                    let request = create_query_request(
                        &json,
                        "determinism: Administration || Researcher || Machine",
                        0,
                    );
                    backend.send_query(request)
                })
                .collect::<FuturesUnordered<_>>();

            _ = black_box(responses.collect::<Vec<_>>().await);
        });
    });
}

fn send_expensive_query_different_components(c: &mut Criterion) {
    let json = vec![
        std::fs::read_to_string(format!("{}/Components/Administration.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Researcher.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Machine.json", PATH)).unwrap(),
    ];
    c.bench_function("send_expensive_query_different_components", |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let backend = ConcreteEcdarBackend::default();
            let responses = (0..64)
                .map(|hash| {
                    let request = create_query_request(
                        &json,
                        "determinism: Administration || Researcher || Machine",
                        hash,
                    );
                    backend.send_query(request)
                })
                .collect::<FuturesUnordered<_>>();

            _ = black_box(responses.collect::<Vec<_>>().await);
        });
    });
}

fn send_query_same_components(c: &mut Criterion) {
    let json = vec![std::fs::read_to_string(format!("{}/Components/Machine.json", PATH)).unwrap()];
    c.bench_function("send_query_same_components", |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let backend = ConcreteEcdarBackend::default();
            let responses = (0..64)
                .map(|_| {
                    let request = create_query_request(&json, "determinism: Machine", 0);
                    backend.send_query(request)
                })
                .collect::<FuturesUnordered<_>>();

            _ = black_box(responses.collect::<Vec<_>>().await);
        });
    });
}

fn send_query_different_components(c: &mut Criterion) {
    let json = vec![std::fs::read_to_string(format!("{}/Components/Machine.json", PATH)).unwrap()];
    c.bench_function("send_query_different_components", |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let backend = ConcreteEcdarBackend::default();
            let responses = (0..64)
                .map(|hash| {
                    let request = create_query_request(&json, "determinism: Machine", hash);
                    backend.send_query(request)
                })
                .collect::<FuturesUnordered<_>>();

            _ = black_box(responses.collect::<Vec<_>>().await);
        });
    });
}

fn create_query_request(json: &Vec<String>, query: &str, hash: u32) -> Request<QueryRequest> {
    Request::new(QueryRequest {
        user_id: 0,
        query_id: 0,
        query: String::from(query),
        components_info: Some(ComponentsInfo {
            components: create_components(json),
            components_hash: hash,
        }),
        ignored_input_outputs: None,
    })
}

fn create_components(json: &Vec<String>) -> Vec<Component> {
    json.into_iter()
        .map(|json| Component {
            rep: Some(Rep::Json(json.clone())),
        })
        .collect()
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(FlamegraphProfiler::new(100));
    targets = self_refinement, refinement, not_refinement,
}

criterion_group! {
    name = backend_bench;
    config = Criterion::default().with_profiler(FlamegraphProfiler::new(100));
    targets = send_query_same_components, send_query_different_components, send_expensive_query_same_components, send_expensive_query_different_components
}

criterion_main!(benches, backend_bench);
