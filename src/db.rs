use {
    crate::*,
    chrono::prelude::*,
    rusqlite::{params, Connection, OptionalExtension, Row},
    std::{path::PathBuf, time::Duration},
};

/// version of the schema
pub const VERSION: &str = "1";

fn create_tables(con: &Connection) -> Result<(), GlassBenchError> {
    con.execute(
        "CREATE TABLE IF NOT EXISTS bench (
            id INTEGER PRIMARY KEY,
            time INTEGER NOT NULL,
            name TEXT NOT NULL,
            title TEXT NOT NULL,
            tag TEXT,
            commit_id TEXT
        )",
        params![],
    )?;
    con.execute(
        "CREATE TABLE IF NOT EXISTS task (
            bench INTEGER NOT NULL,
            name TEXT NOT NULL,
            iterations INTEGER NOT NULL,
            total_duration_ns INTEGER NOT NULL,
            mean_duration_ns INTEGER NOT NULL,
            FOREIGN KEY(bench) REFERENCES bench(id),
            PRIMARY KEY(bench, name)
        )",
        params![],
    )?;
    Ok(())
}

/// Storage interface for Glassbench, wrapping a SQLite connection
///
/// All durations are stored as nanoseconds in i64:
/// If the duration of a task exceeds a few centuries it can
/// be assumed benchmarking it isn't really necessary.
pub struct Db {
    pub con: Connection,
}

impl Db {
    /// return the name of the glassbench database file
    pub fn path() -> Result<PathBuf, GlassBenchError> {
        Ok(std::env::current_dir()?.join(format!("glassbench_v{}.db", VERSION)))
    }

    /// Create a new instance of DB, creating the sqlite file and
    /// the tables if necessary
    pub fn open() -> Result<Self, GlassBenchError> {
        let con = Connection::open(Self::path()?)?;
        create_tables(&con)?;
        Ok(Db { con })
    }

    /// Save a bench, with included tasks if any. Return the id of the bench
    pub fn save_bench(&mut self, bench: &Bench) -> Result<i64, GlassBenchError> {
        self.con.execute(
            "INSERT INTO bench (time, name, title, tag, commit_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                bench.time.timestamp(),
                &bench.name,
                &bench.title,
                &bench.tag,
                bench.git_info.as_ref().map(|gi| &gi.commit_id),
            ],
        )?;
        let bench_id = self.con.last_insert_rowid();
        let mut ps = self.con.prepare(
            "INSERT INTO task
                (bench, name, iterations, total_duration_ns, mean_duration_ns)
            VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        let tasks = bench
            .tasks
            .iter()
            .filter_map(|t| t.measure.as_ref().map(|mes| (&t.name, mes)));
        for (name, mes) in tasks {
            ps.execute(params![
                bench_id,
                name,
                mes.iterations,
                mes.total_duration.as_nanos() as i64,
                mes.mean_duration().as_nanos() as i64,
            ])?;
        }
        Ok(bench_id)
    }

    /// Load the last bench having this name
    pub fn last_bench_named(&mut self, name: &str) -> Result<Option<Bench>, GlassBenchError> {
        match self
            .con
            .query_row(
                "SELECT id, time, name, title, tag, commit_id
            fROM bench WHERE name=?1 ORDER BY id DESC LIMIT 1",
                params![name],
                parse_bench,
            )
            .optional()?
        {
            Some((bench_id, mut bench)) => {
                let mut ps = self.con.prepare(
                    "SELECT name, iterations, total_duration_ns FROM task WHERE bench=?1",
                )?;
                let iter = ps.query_map(params![bench_id], parse_task)?;
                for task in iter {
                    bench.tasks.push(task?);
                }
                Ok(Some(bench))
            }
            None => {
                // no bench found
                Ok(None)
            }
        }
    }

    /// Load a [TaskHistory] with all measure for a bench name and task name
    pub fn task_history(
        &mut self,
        bench_name: &str,
        task_name: &str,
    ) -> Result<TaskHistory, GlassBenchError> {
        let mut history = TaskHistory {
            bench_name: bench_name.into(),
            task_name: task_name.into(),
            records: Vec::new(),
        };
        let mut ps = self.con.prepare(
            "SELECT
                bench.time, bench.tag, bench.commit_id,
                task.iterations, task.total_duration_ns
            FROM task JOIN bench ON task.bench=bench.id
            WHERE bench.name=?1 AND task.name=?2
            ORDER BY bench.time",
        )?;
        let iter = ps.query_map(params![bench_name, task_name], parse_task_record)?;
        for record in iter {
            history.records.push(record?);
        }
        Ok(history)
    }
}

/// parse a bench from a row assuming this order:
/// id, time, name, title, tag, commit_id
fn parse_bench(row: &Row<'_>) -> Result<(i64, Bench), rusqlite::Error> {
    let bench_id: i64 = row.get(0)?;
    let commit_id: Option<String> = row.get(5)?;
    let bench = Bench {
        time: Utc.timestamp_opt(row.get(1)?, 0).unwrap(),
        name: row.get(2)?,
        title: row.get(3)?,
        tag: row.get(4)?,
        git_info: commit_id.map(|commit_id| GitInfo { commit_id }),
        tasks: Vec::new(),
    };
    Ok((bench_id, bench))
}

/// parse a task_bench from a row assuming this order:
/// name, iterations, total_duration_ns
fn parse_task(row: &Row<'_>) -> Result<TaskBench, rusqlite::Error> {
    let nanos: i64 = row.get(2)?;
    Ok(TaskBench {
        name: row.get(0)?,
        measure: Some(TaskMeasure {
            iterations: row.get(1)?,
            total_duration: Duration::from_nanos(nanos as u64),
        }),
    })
}

/// Parse a task_record from a row assuming those columns:
///    bench.time, bench.tag, bench.commit_id,
///    task.iterations, task.total_duration_ns
fn parse_task_record(row: &Row<'_>) -> Result<TaskRecord, rusqlite::Error> {
    let commit_id: Option<String> = row.get(2)?;
    let nanos: i64 = row.get(4)?;
    Ok(TaskRecord {
        time: Utc.timestamp_opt(row.get(0)?, 0).unwrap(),
        git_info: commit_id.map(|commit_id| GitInfo { commit_id }),
        tag: row.get(1)?,
        measure: TaskMeasure {
            iterations: row.get(3)?,
            total_duration: Duration::from_nanos(nanos as u64),
        },
    })
}
