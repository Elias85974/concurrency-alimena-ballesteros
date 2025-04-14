mod server;
mod router;
mod threadpool;
mod worker;
mod logs_controller;

fn main() -> std::io::Result<()> {
    server::execute()
}
