use {
    crate::*,
    termimad::minimad::OwningTemplateExpander,
};

static MD: &str = r#"
# ${bench-title}
${comparison
comparison with ${previous-date} ${git-diff}
}
|-:|:-:|:-:|:-:|:-:
|#|**task**|**iterations**|**total duration**|**mean duration**|**change**
|-:|:-|-:|-:|-:|-:
${tasks
|**${task-num}**|${task-name}|${iterations}|${total-duration}|**${mean-duration}**|${change}
}
|-:
"#;

/// A temporary structure to print the result of a benchmark to the console
pub struct Report<'b> {
    bench: &'b Bench,
    previous: &'b Option<Bench>,
}

impl<'b> Report<'b> {

    pub fn new(
        bench: &'b Bench,
        previous: &'b Option<Bench>,
    ) -> Self {
        Self {
            bench,
            previous,
        }
    }

    /// Print the report to the console
    ///
    /// You don't have to call this yourself if you use
    /// the `glassbench!` macro.
    pub fn print(&self, printer: &Printer) {
        let mut expander = OwningTemplateExpander::new();
        expander
            .set("bench-title", &self.bench.title)
            .set("bench-name", &self.bench.name);
        if let Some(previous) = self.previous.as_ref() {
            expander.sub("comparison")
                .set("previous-date", previous.time)
                .set("git-diff", GitInfo::diff(&previous.git_info, &self.bench.git_info));
        }
        for (idx, task) in self.bench.tasks.iter().enumerate() {
            if let Some(mes) = &task.measure {
                let sub = expander.sub("tasks");
                sub
                    .set("task-num", idx+1)
                    .set("task-name", &task.name)
                    .set("iterations", &mes.iterations)
                    .set("total-duration", format!("{:?}", &mes.total_duration))
                    .set("mean-duration", format!("{:?}", mes.mean_duration()));
                let diff = self.previous
                    .as_ref()
                    .and_then(|obench| task.diff_with(obench));
                if let Some(diff) = diff {
                    sub.set_md(
                        "change",
                        if diff.percents < 0.0 {
                            if diff.percents < -2.0 {
                                format!("*{:.2}%*", diff.percents)
                            } else {
                                format!("{:.2}%", diff.percents)
                            }
                        } else {
                            if diff.percents > 2.0 {
                                format!("~~+{:.2}%~~", diff.percents)
                            } else {
                                format!("+{:.2}%", diff.percents)
                            }
                        },
                    );
                } else {
                    sub.set("change", " ");
                }
            }
        }
        printer.print(expander, MD);
    }
}
