use std::{fs::File, os::raw::c_int, path::Path};

use criterion::profiler::Profiler;
use pprof::ProfilerGuard;

pub struct FlamegraphProfiler<'a> {
    frequency: c_int,
    active_profiler: Option<ProfilerGuard<'a>>,
}

impl<'a> FlamegraphProfiler<'a> {
    pub fn new(frequency: c_int) -> Self {
        FlamegraphProfiler {
            frequency,
            active_profiler: None,
        }
    }
}

impl<'a> Profiler for FlamegraphProfiler<'a> {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        self.active_profiler = Some(ProfilerGuard::new(self.frequency).unwrap());
    }

    fn stop_profiling(&mut self, _benchmark_id: &str, benchmark_dir: &Path) {
        std::fs::create_dir_all(benchmark_dir).expect("Could not create benchmark directory");
        let flamegraph_path = benchmark_dir.join("flamegraph.svg");
        let flamegraph_file =
            File::create(&flamegraph_path).expect("Could not create the flamegraph file");
        self.active_profiler
            .take()
            .unwrap()
            .report()
            .build()
            .unwrap()
            .flamegraph(flamegraph_file)
            .expect("Could not save the flamegraph")
    }
}
