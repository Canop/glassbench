use {
    serde::{Serialize, Deserialize},
    std::time::Duration,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TaskMeasure {
    pub iterations: u32,
    pub total_duration: Duration,
}

impl TaskMeasure {
    pub fn mean_duration(&self) -> Duration {
        self.total_duration / self.iterations
    }
}
