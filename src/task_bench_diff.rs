use {
    crate::*,
};

pub struct TaskBenchDiff {
    pub percents: f64,
}

impl TaskBenchDiff {
    pub fn new(old_mes: TaskMeasure, new_mes: TaskMeasure) -> Self {
        let old_ns = old_mes.mean_duration().as_nanos() as f64;
        let new_ns = new_mes.mean_duration().as_nanos() as f64;
        let diff_ns = new_ns - old_ns;
        let percents = 100f64 * diff_ns / old_ns;
        Self {
            percents,
        }
    }
}
