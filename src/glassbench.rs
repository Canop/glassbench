use {
    crate::*,
    serde::{Serialize, Deserialize},
};


/// A whole benchmark.
///
/// You normally create it with the `glassbench!`
/// macro which will manage table rendering, saving
/// and graphing if required by arguments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlassBench {
    pub id: String,
    pub name: String,
    pub tasks: Vec<TaskBench>,
}

impl GlassBench {

    /// Create a benchmark with a specific id
    /// and name.
    ///
    /// You normally create it with the `glassbench!`
    /// macro which will fetch the id in the name of
    /// the executed benchmark.
    pub fn new<S1, S2>(id: S1, name: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            name: name.into(),
            tasks: Vec::new(),
        }
    }

    /// Specify a task to benchmark.
    pub fn task<S: Into<String>, F>(&mut self, name: S, mut f: F)
    where
        F: FnMut(&mut TaskBench),
    {
        let mut b = TaskBench::new(name.into());
        f(&mut b);
        self.tasks.push(b);
    }

    /// print the report to the console.
    ///
    /// You don't have to call this yourself if you use
    /// the `glassbench!` macro.
    pub fn print_report(&self) {
        let previous = match DatedGlassbench::last(&self.id) {
            Err(e) => {
                eprintln!("failed loading previous report: {}", e);
                None
            }
            Ok(previous) => {
                previous
            }
        };
        let report = Report::new(&self, &previous);
        report.print();
    }

    /// graph some history.
    ///
    /// You don't have to call this yourself if you use
    /// the `glassbench!` macro.
    pub fn graph(&self, graph_arg: &str) {
        if let Ok(num) = graph_arg.parse::<usize>() {
            if num == 0 {
                eprintln!("graph argument 0 not yet implemented");
            } else if let Some(task) = self.tasks.get(num-1) {
                match History::of_task(&self.id, &task.name) {
                    Ok(history) if history.is_graphable() => {
                        match history.open_graph() {
                            Err(e) => {
                                eprintln!("Error opening history graph: {}", e);
                            }
                            _ => {
                                println!("History graph open in your browser");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("reading task history failed: {}", e);
                    }
                    _ => {
                        eprintln!("not enough points in history");
                    }
                }
            } else {
                eprintln!("no task with number {} found", num);
            }
        } else {
            eprintln!("graph argument not understood: {:?}", graph_arg);
        }
    }

    /// save the measurements into the .glassbench directory
    ///
    /// You don't have to call this yourself if you use
    /// the `glassbench!` macro.
    pub fn save(self) {
        let dgb = DatedGlassbench::new(self);
        dgb.save();
    }

}


