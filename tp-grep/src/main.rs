use std::env;

mod grep;
mod file_searcher;

fn main() {
    let args: Vec<String> = env::args().collect();
    let strategy = args[1].as_str();
    let pattern = args[2].as_str();
    let files = args[3..].iter().map(|file| file.to_string()).collect();
    let result = grep::search(strategy, pattern, files);
    result.iter().for_each(|file| {
        if !file.lines.is_empty() { println!("{} found in {}:", pattern, file.file_name); }
        file.lines.iter().for_each(|line| {println!("Line {}: {}", line.line_number, line.line_content)})
    });
}