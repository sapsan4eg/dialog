extern crate dialog;
extern crate rustc_serialize;
extern crate backtrace;
extern crate log;
extern crate time;

use rustc_serialize::json::{self, Json, ToJson};
use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;
use std::fs::{OpenOptions};
use std::io::prelude::*;
use backtrace::Backtrace;
use std::sync::Arc;
use dialog::Handler;
use log::LogRecord;

struct LogJson {
    level: String,
    extra: MessageJson,
    file: String,
    program: String,
    line: u32,
    time: String,
    trace: String
}

#[derive(RustcDecodable)]
pub struct MessageJson {
    pub message: String,
    pub description: String
}

impl ToJson for MessageJson {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("message".to_string(), self.message.to_json());
        d.insert("description".to_string(), self.description.to_json());
        Json::Object(d)
    }
}

impl ToJson for LogJson {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("level".to_string(), self.level.to_json());
        d.insert("extra".to_string(), self.extra.to_json());
        d.insert("file".to_string(), self.file.to_json());
        d.insert("program".to_string(), self.program.to_json());
        d.insert("line".to_string(), self.line.to_json());
        d.insert("time".to_string(), self.time.to_json());
        d.insert("trace".to_string(), self.trace.to_json());
        Json::Object(d)
    }
}

pub struct StreamHandler {
    channels: Arc<Mutex<HashMap<String, Vec<String>>>>
    , last_time: Mutex<u64>
    , count: usize
    , delay: u64
    , flush_type: String
    , trace_types: Vec<String>
}

impl StreamHandler {
    pub fn new() -> StreamHandler {
        StreamHandler {
            channels: Arc::new(Mutex::new(HashMap::new())),
            last_time: Mutex::new(time::precise_time_ns()),
            count: 100,
            delay: 1000u64,
            flush_type: "ERROR".to_string(),
            trace_types: vec!("ERROR".to_string())
        }
    }

    fn write(&self, msg: &LogJson) {
        let mut channel = self.channels.lock().unwrap();

        if !channel.contains_key(&msg.program.to_string()) {
            channel.insert(msg.program.to_string(), Vec::new());
        }

        self.write_row(&mut channel, &msg);

        if msg.level == self.flush_type {
            for (channel_name, channel_row) in channel.iter_mut() {
                self.flush(&channel_name.to_string(), channel_row);
            }
        }
    }

    fn write_row(&self, channel: &mut HashMap<String, Vec<String>>, msg: &LogJson) {
        if let Some(res) = channel.get_mut(&msg.program.to_string()) {
            res.push(msg.to_json().to_string());
            let mut t = self.last_time.lock().unwrap();

            if res.len() > self.count || time::precise_time_ns() - *t > self.delay * 1000000 {
                self.flush(&msg.program.to_string(), res);
            }

                *t = time::precise_time_ns();
        }
    }

    fn flush(&self, path: &str, res: &mut Vec<String>) {

        if res.len() == 0 {
            return;
        }

        if let Ok(mut f) = OpenOptions::new().write(true).create(true).append(true).open(format!("{}.{}", path, "txt")) {
            for key in res.clone() {
                f.write_all(&format!("{}{}" , key, "\n").into_bytes()).unwrap();
            }
            f.sync_all().unwrap();
            res.truncate(0);
        }
    }
}

impl Handler for StreamHandler {
    fn handle(&self, record: &LogRecord) -> Option<bool> {
        let mut trace = String::new();

        if self.trace_types.contains(&record.level().to_string()) {
            trace = format!("{:?}", Backtrace::new());
        }

        let msg = LogJson {
            level: record.level().to_string(),
            extra: match json::decode(&record.args().to_string()) {
                Ok(n) => n,
                Err(_) => MessageJson{ message: format!("{}", record.args().to_string()), description: "".to_string() },
            },
            file: record.location().file().to_string(),
            program: record.location().module_path().to_string(),
            line: record.location().line(),
            time: time::strftime(&"%FT%T%z".to_string(), &time::now()).unwrap(),
            trace: trace
        };

        self.write(&msg);

        Some(true)
    }
}