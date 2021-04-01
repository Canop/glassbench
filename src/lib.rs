/*!

A micro-benchmarking crate with memory.

See [usage and example in README](https://github.com/Canop/glassbench#usage).

In a standard setup you'll only use

* the [glassbench!] macro which let you title your bench and add functions defining tasks
* the [Bench] struct, as argument of your global bench function, with its [Bench::task] function to define a task
* the [TaskBench] struct that you receive as argument when defining a task. You'll call
[TaskBench::iter] with the callback to benchmark
* [pretend_used] as an opaque sinkhole, which can receive the values you produce in your tests and
prevent the optimizer to remove their construction

*/


mod black_box;
mod bench;
mod command;
mod db;
mod error;
mod git_info;
mod history_graph;
mod history_tbl;
mod html_viewer;
mod main_macro;
mod printer;
mod report;
mod skin;
mod task_bench;
mod task_bench_diff;
mod task_history;
mod task_measure;

pub use {
    db::*,
    bench::*,
    black_box::*,
    command::*,
    error::*,
    git_info::*,
    history_graph::*,
    history_tbl::*,
    html_viewer::*,
    main_macro::*,
    printer::*,
    report::*,
    task_bench::*,
    task_bench_diff::*,
    task_history::*,
    task_measure::*,
};
