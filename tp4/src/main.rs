mod server;
mod router;
mod threadpool;
mod worker;
mod logs_controller;
mod grep;
mod log_handler;

fn main() -> std::io::Result<()> {
    server::execute()
}
