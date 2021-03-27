use {
    crate::*,
    std::{
        convert::TryInto,
        time::{Duration, Instant},
    },
};

pub const WARMUP_ITERATIONS: usize = 2;
pub const ESTIMATE_ITERATIONS: u32 = 5;
pub const OPTIMAL_DURATION_NS: u128 = Duration::from_secs(2).as_nanos();
pub const MINIMAL_ITERATIONS: u32 = 50;


/// benching of one task
#[derive(Debug, Clone)]
pub struct TaskBench {
    pub name: String,
    pub measure: Option<TaskMeasure>,
}

impl TaskBench {

    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            measure: None,
        }
    }

    pub(crate) fn diff_with(&self, old_bench: &Bench) -> Option<TaskBenchDiff> {
        old_bench.tasks.iter()
            .find(|tb| tb.name == self.name)
            .and_then(|old_tb| old_tb.measure)
            .and_then(|old_mes| {
                self.measure.map(|new_mes| TaskBenchDiff::new(old_mes, new_mes))
            })
    }

    /// call the function to measure.
    ///
    /// There will be an initial warmup, after which
    /// the function will be called enought times to
    /// get a reliable estimation of its duration.
    pub fn iter<M, R>(&mut self, mut measured: M)
    where
        M: FnMut() -> R,
    {
        if self.measure.is_some() {
            eprintln!("bench already used - please fix your benchmark");
            return;
        }
        // just a warmup
        for _ in 0..WARMUP_ITERATIONS {
            measured();
        }
        // first estimation, to compute the number of iterations later
        let start = Instant::now();
        for _ in 0..ESTIMATE_ITERATIONS {
            measured();
        }
        let estimate_ns = start.elapsed().as_nanos();
        let iterations = ((OPTIMAL_DURATION_NS * ESTIMATE_ITERATIONS as u128) / estimate_ns)
            .try_into()
            .unwrap_or(MINIMAL_ITERATIONS)
            .max(MINIMAL_ITERATIONS);
        // now we do the real measure
        let start = Instant::now();
        for _ in 0..iterations {
            measured();
        }
        let total_duration = start.elapsed();
        self.measure = Some(TaskMeasure {
            iterations,
            total_duration,
        });
    }
}
