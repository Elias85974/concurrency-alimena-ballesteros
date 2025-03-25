use std::thread;

fn search(strategy: &str, pattern: &str, files: Vec<String>) -> Vec<Vec<&'_ str>> {
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

fn chunk(pattern: &str, files: Vec<String>, chunk_size: usize) -> Vec<Vec<&'_ str>> {
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
        .collect()
}
fn concurrent(pattern: &str, files: Vec<String>) -> Vec<Vec<&'_ str>> {
    let mut linesfound = Vec::new();

    for file in files {
        let pattern = pattern.to_string();
        let lines = thread::spawn(move || {
            let mut lines_in_file = Vec::new();
            for line in file.lines() {
                if find_in_line(&pattern, line) {
                    lines_in_file.push(line.to_string());
                }
            }
            lines_in_file
        });
        linesfound.push(lines);
    }

    linesfound.into_iter()
        .map(|t| t.join().unwrap())
        .collect()
}

fn sequential(pattern: &str, files: Vec<String>) -> Vec<Vec<&'_ str>> {
    let mut linesfound: Vec<Vec<&str>> = Vec::new();
    for file in files {
        let mut lines_in_file: Vec<&str> = Vec::new();
        for line in file.split("\n") {
            if find_in_line(pattern, line) {
                lines_in_file.push(line);
            }
        }
        linesfound.push(lines_in_file);
    }
    linesfound
}