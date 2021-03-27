use {
    crate::*,
    chrono::prelude::*,
};

#[derive(Debug)]
pub struct TaskRecord {
    pub time: DateTime<Utc>,
    pub git_info: Option<GitInfo>,
    pub tag: Option<String>,
    pub measure: TaskMeasure,
}

#[derive(Debug)]
pub struct TaskHistory {
    pub bench_name: String,
    pub task_name: String,
    pub records: Vec<TaskRecord>,
}
