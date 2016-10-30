#[macro_use] extern crate log;
extern crate log_panics;

use log::*;
use std::sync::Mutex;
use std::sync::Arc;

pub trait Handler: Send + Sync + 'static {
    fn handle(&self, record: &LogRecord) -> bool;
}

impl Handler for Box<Handler> {
    fn handle(&self, record: &LogRecord) -> bool {
        (**self).handle(record)
    }
}

pub trait Formatter: Send + Sync + 'static {
    fn format(&self, record: &LogRecord) -> String;
}

pub struct Logger {
    handlers: Arc<Mutex<Vec<Box<Handler>>>>,
    level: LogLevelFilter
}

impl Logger {

    pub fn init(self) -> Result<(), SetLoggerError> {
        log_panics::init();
        log::set_logger(|max_log_level| {
            max_log_level.set(self.level);
            Box::new(self)
        })
    }

    pub fn init_without_panics(self) -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(self.level);
            Box::new(self)
        })
    }

    pub fn new(level: LogLevelFilter) -> Logger {
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
                if true == handler.handle(record) {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test;