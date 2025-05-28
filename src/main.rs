fn main() {
    let hits = grepper::grep::recursive_grep().unwrap();
    for hit in hits {
        let rel_path = format!("{}", hit.file.display());
        let rel_path = rel_path.trim_start_matches(&format!("{}/", grepper::CWD.display()));

        println!("[{}:{}] {}", rel_path, hit.line_number, hit.line_content)
    }
}
