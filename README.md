# Dialog - Logging for Rust
---

## Simply Example

```rust
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
    // please use 
    // logger.init().unwrap(); this can cover panic to error and logged it or
    logger.init_without_panics().unwrap(); 
    error!("something");
}


```

## Overview

Dialog it is the implementation of the chain of responsibilities for Log Crate. The main idea of the Dialog is that you use the macros of the Log Crate. Log add a record to the logger, it traverses the handler stack. Each handler decides whether it fully handled the record, and if so, the propagation of the record ends there.

Dialog is 100% safe code:

```sh
$ ack unsafe src | wc
       0       0       0
```

The Dialog itself does not produce action with logs. Middleware are the main ways to extend Dialog with new functionality. Most extensions that would be provided by middleware in other repositories.

## Handlers