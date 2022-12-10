use crate::*;

/// Generates a benchmark with a consistent id
/// (using the benchmark file title), calling
/// the benchmarking functions given in argument.
///
/// ```no-test
/// glassbench!(
///     "Sortings",
///     bench_number_sorting,
///     bench_alpha_sorting,
/// );
/// ```
///
/// This generates the whole main function.
/// If you want to set the bench name yourself
/// (not recommanded), or change the way the launch
/// arguments are used, you can write the main
/// yourself and call [create_bench] and [after_bench]
/// instead of using this macro.
#[macro_export]
macro_rules! glassbench {
    (
        $title: literal,
        $( $fun: path, )+
    ) => {
        pub fn main() {
            use glassbench::*;
            let name = env!("CARGO_CRATE_NAME");
            let cmd = Command::read();
            if cmd.include_bench(&name) {
                let mut bench = create_bench(name, $title, &cmd);
                $(
                    $fun(&mut bench);
                )+
                if let Err(e) = after_bench(&mut bench, &cmd) {
                    eprintln!("{}", e);
                }
            } else {
                println!("skipping bench {:?}", &name);
            }
        }
    }
}

/// Create a bench with a user defined name (instead of
/// the file name) and command (instead of the one read in
/// arguments)
///
/// Unless you have special reasons, you should not
/// use this function but the [glassbench!] function.
pub fn create_bench<S1, S2>(name: S1, title: S2, cmd: &Command) -> Bench
where
    S1: Into<String>,
    S2: Into<String>,
{
    let mut bench = Bench::new(name, title);
    bench.tag = cmd.tag.clone();
    bench
}

/// Print the tabular report for the executed benchmark
/// then graph, list history, and or save according to
/// command
///
/// Unless you have special reasons, you should not
/// use this function but the [glassbench!] function.
pub fn after_bench(bench: &mut Bench, cmd: &Command) -> Result<(), GlassBenchError> {
    let printer = Printer::new();
    let mut db = Db::open()?;
    let previous = db.last_bench_named(&bench.name)?;
    let report = Report::new(bench, &previous);
    report.print(&printer);
    let mut no_save = cmd.no_save;
    if let Some(graph_arg) = cmd.graph.as_ref() {
        let task_name = bench.task_name_from_arg(graph_arg);
        let viewer = HtmlViewer::new(&bench.name, task_name);
        viewer.open_in_browser()?;
        no_save = true;
    }
    if let Some(tbl_arg) = cmd.history.as_ref() {
        let history = bench.task_history(&mut db, tbl_arg)?;
        let tbl = HistoryTbl::new(&history);
        tbl.print(&printer);
        no_save = true;
    }
    if !no_save {
        db.save_bench(bench)?;
    }
    Ok(())
}
