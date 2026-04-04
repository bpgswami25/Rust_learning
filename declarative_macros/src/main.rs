use declarative_macros::log as log;

fn main() {
    log!(LogType::Info, "Starting main function");
    log!(LogType::Error, "This is an error");
    log!(LogType::Debug, "Debug info");
}
