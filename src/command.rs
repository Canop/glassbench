

#[derive(Debug, Clone)]
pub struct Command {
    pub benches: Vec<String>,
    pub graph: Option<String>,
    pub history: Option<String>,
    pub tag: Option<String>,
    pub no_save: bool,
    pub verbose: bool,
}

impl Command {

    pub fn read() -> Self {
        let mut args = std::env::args()
            .skip(1); // it's the path to the compiled bench in target
        let mut benches = Vec::new();
        let mut graph = None;
        let mut history = None;
        let mut tag = None;
        let mut before_sep = true;
        let mut no_save = false;
        let mut verbose = false;
        while let Some(arg) = args.next() {
            if arg == "--" {
                before_sep = false;
            } else if before_sep {
                if !arg.starts_with("--") {
                    benches.push(arg);
                }
            } else {
                match arg.as_str() {
                    "--no-save" => {
                        no_save = true;
                    }
                    "--verbose" => {
                        verbose = true;
                    }
                    "--graph" => {
                        if let Some(val) = args.next() {
                            graph = Some(val);
                        }
                    }
                    "--history" => {
                        if let Some(val) = args.next() {
                            history = Some(val);
                        }
                    }
                    "--tag" => {
                        if let Some(val) = args.next() {
                            tag = Some(val);
                        }
                    }
                    "--bench" => {
                        // that's how the command given by cargo bench always ends
                    }
                    _ => {
                        println!("ignored bench argument: {:?}", arg);
                    }
                }
            }
        }
        Self {
            benches,
            graph,
            history,
            tag,
            no_save,
            verbose,
        }
    }

    pub fn include_bench(&self, id: &str) -> bool {
        self.benches.is_empty() || self.benches.iter().any(|g| g==id)
    }

}
