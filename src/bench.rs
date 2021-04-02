use {
    crate::*,
    chrono::prelude::*,
};

/// A whole benchmark
///
/// You normally create it with the `glassbench!`
/// macro which will manage table rendering, saving
/// and graphing if required by arguments.
#[derive(Debug, Clone)]
pub struct Bench {
    pub time: DateTime<Utc>,
    pub name: String,
    pub title: String,
    pub git_info: Option<GitInfo>,
    pub tag: Option<String>,
    pub tasks: Vec<TaskBench>,
}

impl Bench {

    /// Create a benchmark with a specific name and title
    ///
    /// You normally create don't use this function but the `glassbench!`
    /// macro which will fetch the id in the name of the executed benchmark.
    pub fn new<S1, S2>(name: S1, title: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            time: Utc::now(),
            name: name.into(),
            title: title.into(),
            tasks: Vec::new(),
            tag: None,
            git_info: GitInfo::read(),
        }
    }

    /// Specify a task to benchmark
    ///
    /// Example:
    ///
    /// ```
    /// # use glassbench::*;
    /// # struct BigComputer {}
    /// # impl BigComputer {
    /// #     pub fn new() -> Self {
    /// #         Self {}
    /// #     }
    /// #     pub fn answer(&self, q: usize) -> usize {
    /// #         q + 2
    /// #     }
    /// # }
    /// # let mut bench = Bench::new("doc", "Doc Example");
    /// bench.task("answer 42", |task| {
    ///     let computer = BigComputer::new();
    ///     let question = 42;
    ///     task.iter(|| {
    ///         pretend_used(computer.answer(question));
    ///     });
    /// });
    /// ```
    pub fn task<S: Into<String>, F>(&mut self, name: S, mut f: F)
    where
        F: FnMut(&mut TaskBench),
    {
        let mut b = TaskBench::new(name.into());
        f(&mut b);
        self.tasks.push(b);
    }

    /// Warning: this API is considered unstable
    pub fn task_name_from_arg(&self, arg: &str) -> Option<&str> {
        arg.parse::<usize>().ok()
            .and_then(|num| {
                if num == 0 {
                    eprintln!("history argument 0 not yet implemented");
                    None
                } else {
                    self.tasks.get(num - 1)
                }
            })
            .map(|task| task.name.as_str())
    }

    /// load the history of a task from DB
    ///
    /// You don't have to call this yourself if you use the [glassbench!] macro.
    pub fn task_history(
        &self,
        db: &mut Db,
        tbl_arg: &str,
    ) -> Result<TaskHistory, GlassBenchError> {
        if let Some(task_name) = self.task_name_from_arg(tbl_arg) {
            db.task_history(&self.name, task_name)
        } else {
            Err(GlassBenchError::ClientError)
        }
    }

}


