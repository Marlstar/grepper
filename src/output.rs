use crate::grep::Hit;
use crate::util::relative_path;

pub fn display(hits: Vec<Hit>) {
    for hit in hits {
        let rel_path = relative_path(&*crate::CWD, &hit.file).unwrap();

        println!("[{}:{}] {}", rel_path.display(), hit.line_number, hit.line_content)
    }
}
