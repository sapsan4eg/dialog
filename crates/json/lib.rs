extern crate dialog;
extern crate rustc_serialize;
extern crate backtrace;
extern crate log;
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
struct LogJson {
    level: String,
    extra: String,
    file: String,
    line: u32,
    time: String,
    trace: String
}

impl ToJson for LogJson {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();

        d.insert("level".to_string(), self.level.to_json());
        d.insert("extra".to_string(), Json::from_str(&self.extra).unwrap_or(self.extra.to_json()));
        d.insert("file".to_string(), self.file.to_json());
        d.insert("line".to_string(), self.line.to_json());
        d.insert("time".to_string(), self.time.to_json());
        d.insert("trace".to_string(), self.trace.to_json());
        Json::Object(d)
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, record: &LogRecord) -> String {
        let mut trace = String::new();

        if self.trace_types.contains(&record.level()) {
            trace = format!("{:?}", Backtrace::new());
        }

        LogJson {
            level: record.level().to_string(),
            extra: record.args().to_string(),
            file: record.location().file().to_string(),
            line: record.location().line(),
            time: time::strftime(&"%FT%T%z".to_string(), &time::now()).unwrap(),
            trace: trace
        }.to_json().to_string()
    }
}
