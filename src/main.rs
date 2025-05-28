fn main() {
    let hits = grepper::grep::recursive_grep().unwrap();
    grepper::output::display(hits);
}
