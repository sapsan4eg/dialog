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
    trace_types: Vec<LogLevel>,
    program: String
}

impl JsonFormatter {
    pub fn new(loglevel: Vec<LogLevel>) -> JsonFormatter {
        JsonFormatter {
            trace_types: loglevel,
            program: "application/dump".to_string()
        }
    }

    pub fn program(&mut self, program: &str) {
        self.program = program.to_string()
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, record: &LogRecord) -> String {
        let mut d = BTreeMap::new();

        let json = match Json::from_str(&record.args().to_string()) {
            Ok(mut json) => {
                if let Some(mut j) = json.as_object_mut() {
                    if !j.contains_key("program") {
                        j.insert("program".to_string(), self.program.to_json());
                    }
                }
                json
            },
            Err(_) => {
                let mut json = BTreeMap::new();
                json.insert("program".to_string(), self.program.to_json());
                json.insert("data".to_string(), record.args().to_string().to_json());
                Json::Object(json)
            }
        };

        d.insert("level".to_string(), record.level().to_string().to_json());
        d.insert("extra".to_string(), json);
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
