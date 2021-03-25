use {
    crate::*,
    minimad::OwningTemplateExpander,
};

static MD: &str = r#"
# ${bench-name}
|-:|:-:|:-:|:-:|:-:
|#|**task**|**iterations**|**total duration**|**mean duration**|**change**
|-:|:-|-:|-:|-:|-:
${tasks
|**${task-num}**|${task-name}|${iterations}|${total-duration}|**${mean-duration}**|${change}
}
|-:
"#;

pub(crate) struct Report<'b> {
    gb: &'b GlassBench,
    previous: &'b Option<DatedGlassbench>,
}

impl<'b> Report<'b> {
    pub fn new(
        gb: &'b GlassBench,
        previous: &'b Option<DatedGlassbench>,
    ) -> Self {
        Self {
            gb,
            previous,
        }
    }
    pub fn print(&self) {
        let printer = Printer::new();
        let mut expander = OwningTemplateExpander::new();
        expander
            .set("bench-id", &self.gb.id)
            .set("bench-name", &self.gb.name);
        for (idx, task) in self.gb.tasks.iter().enumerate() {
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
                    .map(|dg| &dg.glassbench)
                    .and_then(|ogb| task.diff_with(ogb));
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
