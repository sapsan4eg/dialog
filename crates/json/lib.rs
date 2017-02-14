extern crate dialog;
extern crate rustc_serialize;
extern crate backtrace;
#[macro_use] extern crate log;
extern crate time;

use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;
use backtrace::Backtrace;
use dialog::Formatter;
use log::{LogRecord, LogLevel};

pub struct JsonFormatter {
    trace_types: Vec<LogLevel>
}

impl JsonFormatter {
    pub fn new(loglevel: Vec<LogLevel>) -> JsonFormatter {
        JsonFormatter {
            trace_types: loglevel
        }
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, record: &LogRecord) -> String {
        let mut d = BTreeMap::new();

        d.insert("level".to_string(), record.level().to_string().to_json());
        d.insert("extra".to_string(), Json::from_str(&record.args().to_string()).unwrap_or(record.args().to_string().to_json()));
        d.insert("file".to_string(), record.location().file().to_string().to_json());
        d.insert("line".to_string(), record.location().line().to_json());
        d.insert("time".to_string(), time::strftime(&"%FT%T%z".to_string(), &time::now()).unwrap().to_json());

        if self.trace_types.contains(&record.level()) {
            d.insert("trace".to_string(), format!("{:?}", Backtrace::new()).to_json());
        }

        Json::Object(d).to_string()
    }
}

pub fn log_error(program: &str, message: &str) {
    error!("{}", prepare_data_to_log(program, message));
}

pub fn log_warn(program: &str, message: &str) {
    warn!("{}", prepare_data_to_log(program, message));
}

pub fn log_info(program: &str, message: &str) {
    info!("{}", prepare_data_to_log(program, message));
}

fn prepare_data_to_log(program: &str, message: &str) -> Json {
    let mut m = BTreeMap::new();
    m.insert("program".to_string(), program.to_string().to_json());
    m.insert("data".to_string(), Json::from_str(message).unwrap_or(message.to_string().to_json()));
    Json::Object(m)
}
