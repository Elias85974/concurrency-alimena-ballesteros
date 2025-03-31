use std::thread;
use crate::file_searcher;

pub fn search<'a>(strategy: &'a str, pattern: &'a str, files: Vec<String>) -> Vec<Vec<String>> {
    let result = match strategy {
        "seq" => sequential(pattern, files),
        "conc" => concurrent(pattern, files),
        "c-chunk" => chunk(pattern, files, 5),
        _ => Vec::new()
    };
    result
}

fn find_in_line(pattern: &str, linea: &str) -> bool {
    linea.eq(pattern)
}

fn chunk(pattern: &str, files: Vec<String>, chunk_size: usize) -> Vec<Vec<String>> {
    let mut handles = Vec::new();

    for file in files {
        let pattern = pattern.to_string();
        let lines: Vec<String> = file.lines().map(|line| line.to_string()).collect();

        let handle = thread::spawn(move || {
            let mut chunk_handles = Vec::new();

            for chunk in lines.chunks(chunk_size) {
                let pattern = pattern.clone();
                let chunk = chunk.to_vec();

                let chunk_handle = thread::spawn(move || {
                    chunk.into_iter()
                        .filter(|line| find_in_line(&pattern, line))
                        .map(|line| line.to_string())
                        .collect::<Vec<String>>()
                });

                chunk_handles.push(chunk_handle);
            }

            let mut result = Vec::new();
            for chunk_handle in chunk_handles {
                result.push(chunk_handle.join().unwrap());
            }

            result
        });

        handles.push(handle);
    }

    handles.into_iter()
        .map(|h| h.join().unwrap())
        .flatten()
        .collect()
}

fn concurrent(pattern: &str, files: Vec<String>) -> Vec<Vec<String>> {
    let mut handles = Vec::new();

    for file in files {
        let pattern = pattern.to_string();
        let handle = thread::spawn(move || {
            file.lines()
                .filter(|line| find_in_line(&pattern, line))
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        });
        handles.push(handle);
    }

    handles.into_iter()
        .map(|t| t.join().unwrap())
        .collect()
}

fn sequential(pattern: &str, files: Vec<String>) -> Vec<Vec<String>> {
    let mut linesfound: Vec<Vec<String>> = Vec::new();
    for file in files {
        let mut lines_in_file: Vec<String> = Vec::new();
        let text = file_searcher::find(file.as_str());
        for line in text.split("\n") {
            if find_in_line(pattern, line) {
                lines_in_file.push(line.to_string());
            }
        }
        linesfound.push(lines_in_file);
    }
    linesfound
}