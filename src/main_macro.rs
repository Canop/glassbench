
/// Generates a benchmark with a consistent id
/// (using the benchmark file name), calling
/// the benchmarking function given in argument.
///
/// This generates the whole main function.
/// If you want to se the id yourself, you can
/// write the main yourself and call GlassBench::new
/// instead of using this macro.
#[macro_export]
macro_rules! glassbench {
    (
        $name: literal,
        $( $fun: path, )+
    ) => {
        pub fn main() {
            use glassbench::*;
            let id = env!("CARGO_CRATE_NAME");
            let cmd = Command::read();
            if cmd.include_group(&id) {
                let mut gb = glassbench::GlassBench::new(id, $name);
                $(
                    $fun(&mut gb);
                )+
                gb.print_report();
                if let Some(graph_arg) = cmd.graph.as_ref() {
                    gb.graph(graph_arg);
                } else {
                    gb.save();
                }
            } else {
                println!("skipping bench {:?}", &id);
            }
        }
    }
}

