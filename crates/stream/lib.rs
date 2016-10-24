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
    , file: String
}

impl StreamHandler {
    pub fn new<F>(file: String, count: usize, delay: u64, formatter: F) -> StreamHandler
        where F: Formatter {

        StreamHandler {
            channels: Arc::new(Mutex::new(HashMap::new())),
            last_time: Mutex::new(time::precise_time_ns()),
            count: count,
            delay: delay,
            flush_type: LogLevel::Error
            , formatter: Box::new(formatter)
            , file: file
        }
    }

    fn flush(&self, res: &mut Vec<String>) {

        if res.len() == 0 {
            return;
        }

        if let Ok(mut f) = OpenOptions::new().write(true).create(true).append(true).open(format!("{}", self.file)) {
            for key in res.clone() {
                f.write_all(&format!("{}{}" , key, "\n").into_bytes()).unwrap();
            }
            f.sync_all().unwrap();
            res.truncate(0);
        } else {
            println!("Cannot write log to: {}", self.file);
        }
    }
}

impl Handler for StreamHandler {
    fn handle(&self, record: &LogRecord) -> bool {

        let mut channel = self.channels.lock().unwrap();
        let ch: String = "base".to_string();

        if !channel.contains_key(&ch) {
            channel.insert((ch.clone()), Vec::new());
        }

        if let Some(res) = channel.get_mut(&ch) {
            res.push(self.formatter.format(record));

            let mut t = self.last_time.lock().unwrap();

            if res.len() > self.count || time::precise_time_ns() - *t > self.delay * 1000000 {
                self.flush(res);
            }

            *t = time::precise_time_ns();
        }

        if record.level() == self.flush_type {
            for channel_row in channel.values_mut() {
                self.flush(channel_row);
            }
        }

        true
    }
}
