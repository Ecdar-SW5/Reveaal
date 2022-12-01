use std::vec;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reveaal::ProtobufServer::{
    services::{
        component::Rep, ecdar_backend_server::EcdarBackend, Component, ComponentsInfo, QueryRequest,
    },
    ConcreteEcdarBackend,
};
use tonic::Request;

use criterion::async_executor::FuturesExecutor;
use futures::stream::FuturesUnordered;
use futures::StreamExt;

pub mod flamegraph;
use flamegraph::flamegraph_profiler::FlamegraphProfiler;

static PATH: &str = "samples/json/EcdarUniversity";

const NUM_OF_REQUESTS: u32 = 512;

fn send_query_with_components(
    id: String,
    c: &mut Criterion,
    components: &[String],
    query: &str,
    active_cache: bool,
) {
    c.bench_function(&id, |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let backend = ConcreteEcdarBackend::default();
            let responses = (0..NUM_OF_REQUESTS)
                .map(|hash| {
                    let request = create_query_request(
                        components,
                        query,
                        if active_cache { 0 } else { hash },
                    );
                    backend.send_query(request)
                })
                .collect::<FuturesUnordered<_>>();

            _ = black_box(responses.collect::<Vec<_>>().await);
        });
    });
}

fn create_query_request(json: &[String], query: &str, hash: u32) -> Request<QueryRequest> {
    Request::new(QueryRequest {
        user_id: 0,
        query_id: 0,
        query: String::from(query),
        components_info: Some(ComponentsInfo {
            components: create_components(json),
            components_hash: hash,
        }),
        ignored_input_outputs: None,
        settings: None,
    })
}

fn create_components(json: &[String]) -> Vec<Component> {
    json.iter()
        .map(|json| Component {
            rep: Some(Rep::Json(json.clone())),
        })
        .collect()
}

fn threadpool_cache(c: &mut Criterion) {
    let json = vec![
        std::fs::read_to_string(format!("{}/Components/Administration.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Researcher.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Machine.json", PATH)).unwrap(),
    ];
    let expensive_query = String::from("determinism: Administration || Researcher || Machine");
    let cheap_query = String::from("determinism: Machine");

    send_query_with_components(
        String::from("Expensive queries with identical models"),
        c,
        &json,
        &expensive_query,
        true,
    );
    send_query_with_components(
        String::from("Expensive queries with different models"),
        c,
        &json,
        &expensive_query,
        false,
    );
    send_query_with_components(
        String::from("Cheap queries with identical models"),
        c,
        &json,
        &cheap_query,
        true,
    );
    send_query_with_components(
        String::from("Cheap queries with different models"),
        c,
        &json,
        &cheap_query,
        false,
    );

    let bigJson = vec![
        std::fs::read_to_string(format!("{}/Components/Administration.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Researcher.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Machine.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Adm2.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Machine2.json", PATH)).unwrap(),
        std::fs::read_to_string(format!("{}/Components/Machine3.json", PATH)).unwrap(),
    ];

    let very_expensive = String::from("determinism: Administration || Researcher || Machine || Adm2 || Machine2 || Machine3");

    send_query_with_components(
        String::from("Determinism multithread bench"),
        c,
        &bigJson,
        &very_expensive,
        true,
    );

}

criterion_group! {
    name = backend_bench;
    config = Criterion::default().with_profiler(FlamegraphProfiler::new(100));
    targets = threadpool_cache
}

criterion_main!(backend_bench);
