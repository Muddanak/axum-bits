# Axum bits and pieces (Also a WIP)

## Basic implementations for anyone to view to get started with Axum

Feel free to use this as a getting-started guide for a quick setup for Axum.  I wouldn't exactly call this an Axum tutorial, just more a demo of using it.  I'll add more in time.
Any examples you want to see here put in an issue ticket with "REQUEST: " in the title and I'll see if I can get to it in a timely manner!

### The starting point is the aptly named [start](https://github.com/Muddanak/axum-bits/tree/master/basics/start/src/main.rs)

Then you can look at either [responders](https://github.com/Muddanak/axum-bits/tree/master/basics/responders) or [extractors](https://github.com/Muddanak/axum-bits/tree/master/basics/extractors)

### Added in some medium-level additions

## How to run each module

`cargo run --bin <module>` from the main `axum-bits` directory

### Current Modules


| Module Name | Description |
|---|---|
[start](https://github.com/Muddanak/axum-bits/blob/master/basics/start/src/main.rs)       | basic Axum server
[responders](https://github.com/Muddanak/axum-bits/blob/master/basics/responders/src/main.rs)  | Demo of Axum Response
[extractors](https://github.com/Muddanak/axum-bits/blob/master/basics/extractors/src/main.rs)  | Demo of Axum Extractor
[htmlfile](https://github.com/Muddanak/axum-bits/blob/master/basics/htmlfile/src/main.rs)    | Demo of serving a static HTML file with tower_http
[htmlstatic](https://github.com/Muddanak/axum-bits/blob/master/basics/htmlstatic/src/main.rs) | Demo of statically storing an HTML file to serve as a route
[logging](https://github.com/Muddanak/axum-bits/blob/master/basics/logging/src/main.rs) | Demo of debug logging using tracing_subscriber middleware
[state](https://github.com/Muddanak/axum-bits/blob/master/basics/state/src/main.rs)       | Demo of how to send a state of data with the server
[state-modification](https://github.com/Muddanak/axum-bits/blob/master/medium/state-modification/src/main.rs) | Demo of how to modify the state inside of the server with Arc and Mutex


## [dependencies]

[axum](https://crates.io/crates/axum) 

[serde](https://crates.io/crates/serde) 

[serde_json](https://crates.io/crates/serde_json) 

[tokio](https://crates.io/crates/tokio) 

[tower_http](https://crates.io/crates/tower-http)

[tracing-subscriber](https://crates.io/crates/tracing-subscriber)

---
MIT License
