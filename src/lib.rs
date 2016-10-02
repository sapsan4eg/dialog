#[macro_use] extern crate log;
extern crate log_panics;

use log::*;
use std::sync::Mutex;
use std::sync::Arc;

pub trait Handler: Send + Sync + 'static {
    fn handle(&self, record: &LogRecord) -> Option<bool>;
}

impl Handler for Box<Handler> {
    fn handle(&self, record: &LogRecord) -> Option<bool> {
        (**self).handle(record)
    }
}

pub struct Logger {
    handlers: Arc<Mutex<Vec<Box<Handler>>>>,
    level: LogLevel
}

impl Logger {

    pub fn init(self) -> Result<(), SetLoggerError> {
        log_panics::init();
        log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Info);
            Box::new(self)
        })
    }

    pub fn new(level: LogLevel) -> Logger {
        Logger {
            handlers: Arc::new(Mutex::new(Vec::new())),
            level: level
        }
    }

    pub fn append<H: Handler>(&self, handler: H) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(Box::new(handler));
    }
}


impl log::Log for Logger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {

        if self.enabled(record.metadata()) {

            let handlers = self.handlers.lock().unwrap();

            for handler in handlers.iter() {
                if handler.handle(record).is_none() {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test;