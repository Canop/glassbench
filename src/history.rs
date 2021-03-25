use {
    crate::*,
    csv2svg::*,
    std::{
        fs::{self, File},
        path::PathBuf,
    },
};

#[derive(Debug)]
pub struct History {
    pub bench_id: String,
    pub task_name: String,
    pub measures: Vec<HistoricMeasure>,
}

impl History {
    /// get the directory in which all benchmark records for the
    /// given id are stored.
    pub fn top_dir(id: &str) -> PathBuf {
        std::env::current_dir()
            .unwrap()
            .join(".glassbench")
            .join(id)
    }
    pub fn is_graphable(&self) -> bool {
        self.measures.len() > 1
    }
    pub fn of_task(id: &str, task_name: &str) -> Result<History, GlassBenchError> {
        let mut measures = Vec::new();
        let top_dir = Self::top_dir(id);
        if top_dir.exists() {
            // we load everything and we sort at the end
            let month_dirs = fs::read_dir(top_dir)?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.is_dir());
            for month_dir in month_dirs {
                let dgbs = fs::read_dir(month_dir)?
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| !p.is_dir())
                    .filter_map(|p| File::open(p).ok())
                    .filter_map(|f| {
                        let dgb: Option<DatedGlassbench> = serde_json::from_reader(f).ok();
                        dgb
                    });
                for dgb in dgbs {
                    let mes = dgb.glassbench.tasks
                        .iter()
                        .find(|t| t.name == task_name)
                        .and_then(|t| t.measure);
                    if let Some(measure) = mes {
                        measures.push(HistoricMeasure {
                            date: dgb.date,
                            measure,
                        });
                    }
                }
            }
            measures.sort_by_key(|hm| hm.date);
        }
        Ok(History {
            bench_id: id.into(),
            task_name: task_name.into(),
            measures,
        })
    }

    /// makes a csv2svg table from the measures
    pub fn to_tbl(&self) -> Result<Tbl, GlassBenchError> {
        let mut times = Vec::new();
        let mut durations = Vec::new();
        for mes in &self.measures {
            times.push(mes.date.clone());
            // here we fairlessly convert u128 to i64
            // This is of course quite dangerous
            // -> TODO try if it all fits and if not use ms instead
            // of nanos
            let value = mes.measure.mean_duration().as_nanos() as i64;
            durations.push(Some(value));
        }
        let name = format!("{} / {} (ns)", &self.bench_id, &self.task_name);
        // TODO make better error types in csv2svg then remove unwraps here
        Ok(Tbl::from_seqs(vec![
            Seq::from_increasing_times("time".to_string(), times).unwrap(),
            Seq::from_integers(name, durations).unwrap(),
        ]).unwrap())
    }

    /// open the history as a SVG graph in the browser (hopefully)
    pub fn open_graph(&self) -> Result<(), GlassBenchError> {
        let tbl = self.to_tbl()?;
        let graph = Graph::new(tbl);
        let svg = graph.build_svg();
        let (mut w, path) = temp_file()?;
        csv2svg::write_embedded(&mut w, &svg).unwrap();
        open::that(path)?;
        Ok(())
    }
}
