

#[derive(Debug, Clone)]
pub struct Command {
    pub groups: Vec<String>,
    pub graph: Option<String>
}

impl Command {

    pub fn read() -> Self {
        let mut args = std::env::args()
            .skip(1); // it's the path to the compiled bench in target
        let mut groups = Vec::new();
        let mut graph = None;
        let mut before_sep = true;
        while let Some(arg) = args.next() {
            if arg == "--" {
                before_sep = false;
            } else if before_sep {
                if !arg.starts_with("--") {
                    groups.push(arg);
                }
            } else {
                match arg.as_str() {
                    "--graph" => {
                        if let Some(val) = args.next() {
                            graph = Some(val);
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
            groups,
            graph,
        }
    }

    pub fn include_group(&self, id: &str) -> bool {
        self.groups.is_empty() || self.groups.iter().any(|g| g==id)
    }

}
