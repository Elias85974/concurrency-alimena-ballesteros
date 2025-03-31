use std::io::Error;
use std::thread;
use log::error;
use crate::file_searcher;

pub fn search<'a>(strategy: &'a str, pattern: &'a str, files: Vec<String>) {
    match strategy {
        "seq" => sequential(pattern, files),
        "conc" => concurrent(pattern, files),
        "c-chunk" => chunk(pattern, files, 5),
        _ => error!("No method found for grep")
    };
}

fn find_in_line(pattern: &str, linea: &str) -> bool {
    linea.contains(pattern)
}

fn chunk(pattern: &str, files: Vec<String>, chunk_size: usize) {
    for file in files {
        let pattern = pattern.to_string();
        let lines: Vec<String> = file.lines().map(|line| line.to_string()).collect();
        thread::spawn(move || {
            for chunk in lines.chunks(chunk_size) {
                let pattern = pattern.clone();
                let chunk = chunk.to_vec();

                thread::spawn(move || {
                    chunk.into_iter()
                        .filter(|line| find_in_line(&pattern, line))
                        .map(|line| println!("{}: {}", 1, line.to_string()))
                });
            }
        });
    }
}

fn concurrent(pattern: &str, files: Vec<String>) {
    for file in files {
        let pattern = pattern.to_string();
        thread::spawn(move || {
            file.lines()
                .filter(|line| find_in_line(&pattern, line))
                .for_each(|line| println!("{}: {}", 1, line.to_string()))
        });
    }
}

fn sequential(pattern: &str, files: Vec<String>) {
    for file in files {
        let text = file_searcher::find(file.as_str());
        let mut line_number = 0;
        for line in text.split("\n").map(|line| line.trim_end()) {
            if find_in_line(pattern, line) {
                println!("{}: {}", line_number, line.to_string());
                line_number += 1;
            }
        }
    }
}