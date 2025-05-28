use std::{io::Read, path::{Path, PathBuf}};
use crate::Error;

pub fn all_files(dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut files: Vec<PathBuf> = vec![];

    let mut dirs = recurse_directories(dir)?;
    dirs.push(dir.to_owned());
    for dir in dirs {
        files.append(&mut dir_entries(&dir)?.files);
    }

    return Ok(files);
}

fn dir_entries(dir: &Path) -> Result<DirEntries, Error> {
    if !dir.is_dir() { return Err(Error::NotDirectory); }

    let mut files: Vec<PathBuf> = vec![];
    let mut dirs: Vec<PathBuf> = vec![];

    for entry in dir.read_dir()? {
        let path = entry?.path();

        // Check against .gitignore
        use crate::gitignore::matches;
        if crate::ARGS.gitignore && matches(&path) { continue; }

        if path.is_dir() { dirs.push(path); }
        else if path.is_file() { files.push(path); }
    }

    return Ok(DirEntries { files, dirs });
}

pub struct DirEntries {
    pub files: Vec<PathBuf>,
    pub dirs: Vec<PathBuf>,
}

pub fn recurse_directories(base: &Path) -> Result<Vec<PathBuf>, Error> {
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

pub fn file_content_string(p: &Path) -> Result<String, Error> {
    let mut file = std::fs::File::open(p)?;
    let size = file.metadata()?.len();
    let mut buf = String::with_capacity(size as usize);
    file.read_to_string(&mut buf)?;
    return Ok(buf);
}

pub fn file_content_bytes(p: &Path) -> Result<Vec<u8>, Error> {
    let mut file = std::fs::File::open(p)?;
    let size = file.metadata()?.len();
    let mut buf = Vec::with_capacity(size as usize);
    file.read_to_end(&mut buf)?;
    return Ok(buf);
}

mod tests {
    #[test]
    fn all_files() {
        use crate::CWD;
        let files = super::all_files(&CWD.join("testfs")).unwrap();
        let comp = vec![
            CWD.join("testfs/base.txt"),
            CWD.join("testfs/dir1/c1.md"),
            CWD.join("testfs/dir1/subdir1.2/c1.2.txt"),
            CWD.join("testfs/dir2/d2.txt"),
            CWD.join("testfs/dir2/important_content.txt"),
            CWD.join("testfs/dir2/subdir2.1/c2.1.txt"),
        ];

        dbg!(&files);
        dbg!(&comp);

        let comps = comp.iter().map(|a| files.contains(a)).collect::<Vec<bool>>();

        assert!(!comps.contains(&false))
    }

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
