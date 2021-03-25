use {
    crate::*,
    chrono::prelude::*,
};

/// an old measure read from history
#[derive(Debug, Clone)]
pub struct HistoricMeasure {
    pub date: DateTime<Utc>,
    pub measure: TaskMeasure,
}
