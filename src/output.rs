use crate::grep::Hit;
use crate::util::relative_path;
use colored::Colorize;

pub fn display(hits: Vec<Hit>) {
    for hit in hits {
        let rel_path = relative_path(&crate::ARGS.path, &hit.file).unwrap();

        let file = format!("{}", rel_path.display()).purple();
        let line = format!("{}", hit.line_number).green();

        // Don't highlight match if inverted
        if crate::ARGS.invert {
            println!("[{}:{}] {}", file, line, hit.line_content);
        } else {
            let mut before = hit.line_content;
            let mut mid = before.split_off(hit.start_byte_idx);
            let after = mid.split_off(crate::ARGS.query.len());

            println!("[{}:{}] {}{}{}", file, line, before, mid.red(), after);
        }
    }
}
