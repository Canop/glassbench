use {
    git2::{
        Repository,
    },
};

/// Git related information regarding the execution context
///
/// Right now it just contains the id of the head commit.
#[derive(Debug, Clone)]
pub struct GitInfo {
    pub commit_id: String,
}

impl GitInfo {

    /// Read the current git state (if any)
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

    /// Build a readable abstract of the diff of two [GitInfo]
    pub(crate) fn diff(old_gi: &Option<GitInfo>, new_gi: &Option<GitInfo>) -> String {
        match (old_gi, new_gi) {
            (Some(old_gi), Some(new_gi)) => {
                if old_gi.commit_id == new_gi.commit_id {
                    "(same commit)".to_string()
                } else {
                    format!(
                        "(last commit: {})",
                        // I'm sure there's a less stupid way to print the first 8 chars
                        old_gi.commit_id.chars().take(8).collect::<String>(),
                    )
                }
            }
            _ => {
                "".to_string()
            }
        }
    }

}
