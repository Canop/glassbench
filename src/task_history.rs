use {
    crate::*,
    chrono::prelude::*,
};


/// A measure of a task, with time, commit and tag
#[derive(Debug)]
pub struct TaskRecord {
    pub time: DateTime<Utc>,
    pub git_info: Option<GitInfo>,
    pub tag: Option<String>,
    pub measure: TaskMeasure,
}


/// The history of the measures of ta task as defined
/// by the bench name and task name
#[derive(Debug)]
pub struct TaskHistory {
    pub bench_name: String,
    pub task_name: String,
    pub records: Vec<TaskRecord>,
}
