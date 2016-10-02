use log::*;
use Logger;
use Handler;

struct DummyHandler;

impl Handler for DummyHandler {
    fn handle(&self, record: &LogRecord) -> Option<bool> {
        println!("{}", record.args().to_string());
        Some(true)
    }
}

#[test] fn test_handler() {
    let logger = Logger::new(LogLevel::Info);
    logger.append(DummyHandler);
    info!("hello");
}