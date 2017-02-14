use log::*;
use Logger;
use Handler;

struct DummyHandler;

impl Handler for DummyHandler {
    fn handle(&self, record: &LogRecord) -> bool {
        println!("{}", record.args().to_string());
        true
    }
}

#[test] fn test_handler() {
    let logger = Logger::new(LogLevelFilter::Info);
    logger.append(DummyHandler);
    info!("hello");
}