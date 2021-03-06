/// glassbench error type
#[derive(thiserror::Error, Debug)]
pub enum GlassBenchError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("iter already called")]
    IterAlreadyCalled,
    #[error("SQLite error: {0}")]
    SQLite(#[from] rusqlite::Error),
    #[error("User query not understood")]
    ClientError,
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
