use std::env;

mod grep;
mod file_searcher;

fn main() {
    let args: Vec<String> = env::args().collect();
    let strategy = args[1].as_str();
    let pattern = args[2].as_str();
    let files = args[3..].iter().map(|file| file.to_string()).collect();
    println!("{:?}", grep::search(strategy, pattern, files));
}