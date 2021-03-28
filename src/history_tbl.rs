use {
    crate::*,
    minimad::OwningTemplateExpander,
};

static MD: &str = r#"
## History of ${bench-name} / ${task-name}
|:-:|:-:|:-:|:-:
|**time**|**commit**|**tag**|**mean duration**
|:-|-:|-:|-:
${records
|${time}|${commit}|${tag}|**${mean-duration}**
}
|-:
"#;

/// A temporary structure for printing as table a
/// history in standard output
pub struct HistoryTbl<'b> {
    history: &'b TaskHistory,
}

impl<'b> HistoryTbl<'b> {

    pub fn new(
        history: &'b TaskHistory,
    ) -> Self {
        Self {
            history,
        }
    }

    /// Print the history to the console
    pub fn print(&self, printer: &Printer) {
        let h = &self.history;
        let mut expander = OwningTemplateExpander::new();
        expander
            .set("bench-name", &h.bench_name)
            .set("task-name", &h.task_name);
        for record in &h.records {
            let sub = expander.sub("records");
            sub
                .set("time", record.time)
                .set(
                    "commit",
                    if let Some(gi) = record.git_info.as_ref() {
                        gi.commit_id.chars().take(8).collect::<String>()
                    } else {
                        " ".to_string()
                    },
                )
                .set(
                    "tag",
                    if let Some(tag) = record.tag.as_ref() {
                        tag.to_string()
                    } else {
                        " ".to_string()
                    },
                )
                .set("mean-duration", format!("{:?}", record.measure.mean_duration()));
        }
        printer.print(expander, MD);
    }
}
