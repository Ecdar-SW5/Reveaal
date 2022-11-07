use std::{os::raw::c_int, path::Path};

use criterion::profiler::Profiler;

pub struct FlamegraphProfiler {}

impl FlamegraphProfiler {
    pub fn new(_frequency: c_int) -> Self {
        FlamegraphProfiler {}
    }
}

impl Profiler for FlamegraphProfiler {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        unimplemented!("pprof does not work on windows");
    }

    fn stop_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        unimplemented!("pprof does not work on windows");
    }
}
