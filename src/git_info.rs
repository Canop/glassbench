use {
    git2::{
        Repository,
    },
    serde::{Serialize, Deserialize},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitInfo {
    commit_id: String,
}

impl GitInfo {
    pub fn read() -> Option<Self> {
        std::env::current_dir().ok()
            .and_then(|dir| Repository::discover(dir).ok())
            .and_then(|repo| {
                repo.head().ok()
                    .and_then(|head| {
                        //println!("head: {:?} {:#?} ", &head.name(), &head.kind());
                        head.peel_to_commit().ok()
                            .map(|commit| {
                                GitInfo {
                                    commit_id: commit.id().to_string(),
                                }
                            })
                    })
            })
    }
}
