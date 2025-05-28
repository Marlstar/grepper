use ignore::gitignore::Gitignore;
use std::sync::LazyLock;
use std::path::{Path, PathBuf};

pub static GITIGNORE: LazyLock<Gitignore> = LazyLock::new(find_gitignore);

fn find_gitignore() -> Gitignore {
    use crate::CWD;

    let mut current_path: &Path = &CWD;
    let gitignore_path = loop {
        if current_path.join(".gitignore").exists() && current_path.join(".gitignore").is_file() {
            // .gitignore has been located
            break current_path.join(".gitignore");
        }
        else if let Some(parent) = current_path.parent() {
            // Walk upward
            current_path = parent;
            continue;
        }
        else {
            // There is no parent left to check, and a .gitignore has not been found
            break PathBuf::from("/");
        }
    };

    Gitignore::new(gitignore_path).0
}

/// Check if a path should be ignored (if it matches an entry in `.gitignore`)
pub fn matches(path: impl AsRef<Path>) -> bool {
    matches!(GITIGNORE.matched(&path, path.as_ref().is_dir()), ignore::Match::Ignore(_))
}
