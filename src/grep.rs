use crate::files;
use crate::Error;
use crate::ARGS;
use std::path::PathBuf;

pub fn recursive_grep() -> Result<Vec<Hit>, Error> {
    let files = files::all_files(&crate::CWD)?;
    // TODO: warn about files that failed to open?
    let file_contents = files.into_iter().map(|a| (a.clone(), files::file_content_string(&a).unwrap_or_default())).collect::<Vec<(PathBuf,String)>>();

    let mut hits: Vec<Hit> = vec![];
    for (path, file) in file_contents {
        for (i, line) in file.lines().enumerate() {
            let pos = find(&line);

            match (pos, ARGS.invert) {
                (Some(p), false) => {
                    hits.push(Hit {
                        file: path.clone(),
                        line_number: i,
                        line_content: line.to_string(),
                        start_byte_idx: p,
                    });
                },

                // Inverted query
                (None, true) => {
                    hits.push(Hit {
                        file: path.clone(),
                        line_number: i,
                        line_content: line.to_string(),
                        start_byte_idx: 0,
                    });
                },
                _ => {},
            };
        }
    }

    return Ok(hits);
}

fn find(line: &str) -> Option<usize> {
    let mut line = line.to_string();
    let mut query = ARGS.query.to_string();

    if ARGS.case_insensitive {
        line = line.to_lowercase();
        query = query.to_lowercase();
    }

    return line.find(&query);
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub file: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub start_byte_idx: usize,
}
