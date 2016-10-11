extern crate dialog;
#[macro_use] extern crate log;
extern crate dialog_stream;
extern crate dialog_formatter_json;

use dialog::Logger;
use log::LogLevel;
use dialog_stream::StreamHandler;
use dialog_formatter_json::JsonFormatter;

fn main() {
    let logger = Logger::new(LogLevel::Info);
    logger.append(StreamHandler::new(100, 2000u64, JsonFormatter::new(vec!(LogLevel::Error))));
    //logger.init().unwrap();
    logger.init_without_panics().unwrap();
    error!("something");
}
