use {
    std::time::Duration,
};

/// The result of the measure of a task: number of iterations
/// and total duration
#[derive(Debug, Clone, Copy)]
pub struct TaskMeasure {
    pub iterations: u32,
    pub total_duration: Duration,
}

impl TaskMeasure {
    /// compute the only value you're normally interested into:
    /// the mean duration
    pub fn mean_duration(&self) -> Duration {
        self.total_duration / self.iterations
    }
}
