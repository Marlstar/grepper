use crate::files;
use crate::Error;
use std::path::PathBuf;

pub fn recursive_grep() -> Result<Vec<Hit>, Error> {
    let files = files::all_files(&crate::CWD)?;
    // TODO: warn about files that failed to open?
    let file_contents = files.into_iter().map(|a| (a.clone(), files::file_content_string(&a).unwrap_or_default())).collect::<Vec<(PathBuf,String)>>();

    let mut hits: Vec<Hit> = vec![];
    for (path, file) in file_contents {
        for (i, line) in file.lines().enumerate() {
            if line.contains(&crate::ARGS.query) {
                hits.push(Hit {
                    file: path.clone(),
                    line_number: i,
                    line_content: line.to_string(),
                });
            }
        }
    }

    return Ok(hits);
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub file: PathBuf,
    pub line_number: usize,
    pub line_content: String,
}
