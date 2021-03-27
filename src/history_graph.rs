use {
    crate::*,
    csv2svg::*,
};

pub struct HistoryGraph<'b> {
    history: &'b TaskHistory,
}

impl<'b> HistoryGraph<'b> {

    pub fn new(
        history: &'b TaskHistory,
    ) -> Self {
        Self {
            history,
        }
    }

    /// open the history as a SVG graph in the browser (hopefully)
    pub fn open_in_browser(&self) -> Result<(), GlassBenchError> {
        let h = &self.history;
        let mut times = Vec::new();
        let mut durations = Vec::new();
        for record in &h.records {
            times.push(record.time.clone());
            // here we fairlessly convert u128 to i64
            let value = record.measure.mean_duration().as_nanos() as i64;
            durations.push(Some(value));
        }
        let name = format!("{} / {} (ns)", &h.bench_name, &h.task_name);
        let tbl = Tbl::from_seqs(vec![
            Seq::from_increasing_times("time".to_string(), times).unwrap(),
            Seq::from_integers(name, durations).unwrap(),
        ]).unwrap();
        let graph = Graph::new(tbl);
        let svg = graph.build_svg();
        let (mut w, path) = temp_file()?;
        csv2svg::write_embedded(&mut w, &svg).unwrap();
        open::that(path)?;
        Ok(())
    }
}
