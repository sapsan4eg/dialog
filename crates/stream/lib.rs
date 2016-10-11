extern crate dialog;
extern crate log;
extern crate time;

use std::collections::HashMap;
use std::sync::Mutex;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::Arc;
use dialog::{Handler, Formatter};
use log::{LogRecord, LogLevel};

pub struct StreamHandler {
    channels: Arc<Mutex<HashMap<String, Vec<String>>>>
    , last_time: Mutex<u64>
    , count: usize
    , delay: u64
    , flush_type: LogLevel
    , formatter: Box<Formatter>
    , path: String
}

impl StreamHandler {
    pub fn new<F>(mut path: String, count: usize, delay: u64, formatter: F) -> StreamHandler
        where F: Formatter {

        if path.rfind("/").unwrap_or(0) != path.len() {
            path.push('/');
        }

        StreamHandler {
            channels: Arc::new(Mutex::new(HashMap::new())),
            last_time: Mutex::new(time::precise_time_ns()),
            count: count,
            delay: delay,
            flush_type: LogLevel::Error
            , formatter: Box::new(formatter)
            , path: path
        }
    }

    fn flush(&self, path: &str, res: &mut Vec<String>) {

        if res.len() == 0 {
            return;
        }

        if let Ok(mut f) = OpenOptions::new().write(true).create(true).append(true).open(format!("{}{}.{}", self.path, path, "txt")) {
            for key in res.clone() {
                f.write_all(&format!("{}{}" , key, "\n").into_bytes()).unwrap();
            }
            f.sync_all().unwrap();
            res.truncate(0);
        } else {
            println!("{}{}.{}", self.path, path, "txt");
        }
    }
}

impl Handler for StreamHandler {
    fn handle(&self, record: &LogRecord) -> bool {

        let mut channel = self.channels.lock().unwrap();

        if !channel.contains_key(&record.location().module_path().to_string()) {
            channel.insert((record.location().module_path().to_string()), Vec::new());
        }

        if let Some(res) = channel.get_mut(&record.location().module_path().to_string()) {
            res.push(self.formatter.format(record));

            let mut t = self.last_time.lock().unwrap();

            if res.len() > self.count || time::precise_time_ns() - *t > self.delay * 1000000 {
                self.flush(&record.location().module_path().to_string(), res);
            }

            *t = time::precise_time_ns();
        }

        if record.level() == self.flush_type {
            for (channel_name, channel_row) in channel.iter_mut() {
                self.flush(&channel_name.to_string(), channel_row);
            }
        }

        true
    }
}
