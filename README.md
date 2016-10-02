# Dialog - Logging for Rust
---

## Simply Example

```rust
extern crate dialog;
#[macro_use] extern crate log;

use dialog::Logger;

fn main() {

    let logger = Logger::new();
    logger.link(FileLoggerHandler::new());
    logger.init().unwrap();
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