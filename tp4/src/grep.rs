pub struct FileResult {
    pub file_name: String,
    pub lines: Vec<LineResult>
}

pub struct LineResult {
    pub line_number: i64,
    pub line_content: String
}

fn find_in_line(pattern: &str, linea: &str) -> bool {
    linea.to_lowercase().contains(pattern)
}

pub fn search(pattern: &str, text: &str) -> Vec<LineResult> {
    let mut lines_in_file: Vec<LineResult> = Vec::new();
    let mut line_number = 1;
    for line in text.split("\n") {
        if find_in_line(pattern, line) {
            lines_in_file.push(LineResult {line_number, line_content: line.to_string()});
        }
        line_number += 1;
    }
    lines_in_file
}