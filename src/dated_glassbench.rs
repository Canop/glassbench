use {
    crate::*,
    chrono::prelude::*,
    serde::{Serialize, Deserialize},
    std::{
        fs::{self, File},
        io::Write,
    },
};

pub const FILENAME_DATE_FORMAT: &'static str = "%Y-%m-%dT%H-%M-%SZ.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatedGlassbench {
    pub date: DateTime<Utc>,
    pub bench: GlassBench,
}

impl DatedGlassbench {

    pub(crate) fn new(bench: GlassBench) -> Self {
        Self {
            date: Utc::now(),
            bench,
        }
    }

    pub fn last(id: &str) -> Result<Option<Self>, GlassBenchError> {
        let top_dir = History::top_dir(id);
        if !top_dir.exists() {
            return Ok(None);
        }
        let last_month_dir = fs::read_dir(top_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter_map(|path| {
                path.file_name()
                    .and_then(|s| s.to_str())
                    .and_then(|s| s.parse::<u32>().ok())
                    .map(|d| (path, d))
            })
            .max_by_key(|t| t.1)
            .map(|t| t.0);
        if let Some(month_dir) = last_month_dir {
            let last = fs::read_dir(month_dir)?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter_map(|path| {
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .and_then(parse_date_from_filename)
                        .map(|dt| (path, dt))
                })
                .max_by_key(|t| t.1)
                .map(|t| t.0);
            if let Some(path) = last {
                let file = File::open(path)?;
                Ok(serde_json::from_reader(file)?)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    // TODO maybe better error handling to have just a message
    // instead of crashing ?
    pub fn save(self) {
        let now: DateTime<Utc> = Utc::now();
        let dir = History::top_dir(&self.bench.id)
            .join(now.format("%Y%m").to_string());
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join(now.format(FILENAME_DATE_FORMAT).to_string());
        let mut file = File::create(path).unwrap();
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        write!(file, "{}", serialized).unwrap();
    }
}

pub fn parse_date_from_filename(s: &str) -> Option<DateTime<Utc>> {
    Utc.datetime_from_str(s, FILENAME_DATE_FORMAT).ok()
}
