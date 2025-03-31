use std::fs;

pub fn find(route: &str) -> String {
    let file = fs::read_to_string(route)
        .expect("Should have been able to read the file");
    file
}
