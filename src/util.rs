use std::path::{Path, PathBuf};
use crate::Error;

pub fn relative_path(base: impl AsRef<Path>, sub: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let base = base.as_ref();
    let sub = sub.as_ref();

    return match sub.strip_prefix(base) {
        Ok(p) => Ok(p.to_owned()),
        Err(_) => Err(Error::NotSubdirectory),
    };
}
