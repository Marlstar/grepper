use std::path::{Path, PathBuf};
use crate::Error;

fn all_files() -> Vec<PathBuf> {
    todo!()
}

fn dir_entries(dir: &Path) -> Result<DirEntries, Error> {
    if !dir.is_dir() { return Err(Error::NotDirectory); }

    let mut files: Vec<PathBuf> = vec![];
    let mut dirs: Vec<PathBuf> = vec![];

    for entry in dir.read_dir()? {
        let path = entry?.path();
        if path.is_dir() { dirs.push(path); }
        else if path.is_file() { files.push(path); }
    }

    return Ok(DirEntries { files, dirs });
}

pub struct DirEntries {
    pub files: Vec<PathBuf>,
    pub dirs: Vec<PathBuf>,
}

fn recurse_directories(base: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut dirs: Vec<PathBuf> = vec![];
    recurse_directories_inner(base, &mut dirs);
    return Ok(dirs);
}

fn recurse_directories_inner(p: &Path, v: &mut Vec<PathBuf>) {
    let entries = match dir_entries(p) {
        Ok(a) => a.dirs,
        Err(_) => return,
    };
    for entry in entries {
        v.push(entry.clone());
        recurse_directories_inner(&entry, v);
    }
}

mod tests {
    #[test]
    fn recurse_directories() {
        let testfs = crate::CWD.join("testfs/");
        let comp = [
            testfs.join("dir1"),
            testfs.join("dir1/subdir1.1"),
            testfs.join("dir1/subdir1.2"),

            testfs.join("dir2"),
            testfs.join("dir2/subdir2.1"),
            testfs.join("dir2/subdir2.2")
        ];

        let dirs = super::recurse_directories(&crate::CWD.join("testfs")).unwrap();
        let comps = comp.iter().map(|a| dirs.contains(a)).collect::<Vec<bool>>();
        dbg!(&dirs);
        dbg!(&comps);

        assert!(!comps.contains(&false));
    }
}
