extern crate dialog;
#[macro_use] extern crate log;
extern crate dialog_stream;
extern crate dialog_formatter_json;
extern crate rustc_serialize;
use dialog::Logger;
use log::{LogLevel, LogLevelFilter};
use dialog_stream::StreamHandler;
use dialog_formatter_json::JsonFormatter;


use rustc_serialize::json::{ Json, ToJson};
use std::collections::BTreeMap;

#[derive(RustcDecodable)]
pub struct MessageJson {
    pub message: String,
    pub description: String,
    pub program: String
}

impl ToJson for MessageJson {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("message".to_string(), self.message.to_json());
        d.insert("description".to_string(), self.description.to_json());
        d.insert("program".to_string(), self.program.to_json());
        Json::Object(d)
    }
}


fn main() {
    let logger = Logger::new(LogLevelFilter::Info);
    logger.append(StreamHandler::new(format!("/usr/local/www/dialog/pipe.txt"), 100, 2000u64, JsonFormatter::new(vec!(LogLevel::Error))));
    logger.init().unwrap();
    let json = MessageJson {message: "hello".to_string(), description: "all fine".to_string(), program: "some/program".to_string()};
    info!("{}", json.to_json().to_string());
    warn!("some");
    std::thread::sleep(std::time::Duration::from_millis(2000u64));
    warn!("some");
    //logger.init_without_panics().unwrap();
    error!("something");
  //  error!("{}", MessageJson{ program: "server_error".to_string(), message: "something going wrong".to_string(), description: "yes".to_string()}.to_json());
}
