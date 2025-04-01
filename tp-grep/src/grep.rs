use std::thread;
use crate::file_searcher;

pub struct FileResult {
    pub file_name: String,
    pub lines: Vec<LineResult>
}

pub struct LineResult {
    pub line_number: i64,
    pub line_content: String
}

pub fn search<'a>(strategy: &'a str, pattern: &'a str, files: Vec<String>) -> Vec<FileResult> {
    let result = match strategy {
        "seq" => sequential(pattern, files),
        "conc" => concurrent(pattern, files),
        "c-chunk" => chunk(pattern, files, 5),
        _ => Vec::new()
    };
    result
}

fn find_in_line(pattern: &str, linea: &str) -> bool {
    linea.contains(pattern)
}

fn chunk(pattern: &str, files: Vec<String>, chunk_size: usize) -> Vec<FileResult> {
    let mut handles = Vec::new();

    for file in files {
        let pattern = pattern.to_string();
        let file_clone = file.clone();
        let lines: Vec<String> = file.lines().map(|line| line.to_string()).collect();

        let handle = thread::spawn(move || {
            let mut chunk_handles = Vec::new();
            let mut line_number = 1;

            for chunk in lines.chunks(chunk_size) {
                let pattern = pattern.clone();
                let chunk_clone = chunk.to_vec();
                let start_line_number = line_number;

                let chunk_handle = thread::spawn(move || {
                    chunk_clone.into_iter()
                        .enumerate()
                        .filter_map(|(i, line)| {
                            let current_line_number = start_line_number + i as i64;
                            if find_in_line(&pattern, &line) {
                                Some(LineResult { line_number: current_line_number, line_content: line })
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<LineResult>>()
                });

                chunk_handles.push(chunk_handle);
                line_number += chunk.len() as i64;
            }

            let mut result = Vec::new();
            for chunk_handle in chunk_handles {
                result.push(chunk_handle.join().unwrap());
            }

            FileResult { file_name: get_file_name(file_clone), lines: result.into_iter().flatten().collect() }
        });

        handles.push(handle);
    }

    handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect()
}

fn concurrent(pattern: &str, files: Vec<String>) -> Vec<FileResult> {
    let mut handles = Vec::new();

    for file in files {
        let pattern = pattern.to_string();
        let file_name = file.clone();
        let text = file_searcher::find(file.as_str());
        let handle = thread::spawn(move || {
            let mut line_number = 1;
            let results = text.lines()
                .filter_map(|line| {
                    let result = if find_in_line(&pattern, line) {
                        Some(LineResult { line_number, line_content: line.to_string() })
                    } else {
                        None
                    };
                    line_number += 1;
                    result
                })
                .collect::<Vec<LineResult>>();
            FileResult { file_name: get_file_name(file_name), lines: results }
        });
        handles.push(handle);
    }

    handles.into_iter()
        .map(|t| t.join().unwrap())
        .collect()
}

fn sequential(pattern: &str, files: Vec<String>) -> Vec<FileResult> {
    let mut linesfound: Vec<FileResult> = Vec::new();
    for file in files {
        let mut lines_in_file: Vec<LineResult> = Vec::new();
        let text = file_searcher::find(file.as_str());
        let mut line_number = 1;
        for line in text.split("\n") {
            if find_in_line(pattern, line) {
                lines_in_file.push(LineResult {line_number, line_content: line.to_string()});
            }
            line_number += 1;
        }
        let file_name = get_file_name(file);
        linesfound.push(FileResult {file_name, lines: lines_in_file});
    }
    linesfound
}

fn get_file_name(file: String) -> String {
    file.split('/').last().unwrap_or("").to_string()
}