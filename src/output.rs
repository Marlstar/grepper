use crate::grep::Hit;
use crate::util::relative_path;
use colored::Colorize;

pub fn display(hits: Vec<Hit>) {
    for hit in hits {
        let rel_path = relative_path(&*crate::CWD, &hit.file).unwrap();

        let file = format!("{}", rel_path.display()).purple();
        let line = format!("{}", hit.line_number).green();
        let content = hit.line_content;
        println!("[{}:{}] {}", file, line, content);
    }
}
