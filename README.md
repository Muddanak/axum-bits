# Axum bits and pieces (Also a WIP)

## Basic implementations for anyone to view to get started with Axum

Feel free to use this as a getting-started guide for a quick setup for Axum.  I wouldn't exactly call this an Axum tutorial, just more a demo of using it.  I'll add more in time.

### The starting point is the aptly named [start](https://github.com/Muddanak/axum-bits/tree/master/basics/start/src/main.rs)

Then you can look at either [responders](https://github.com/Muddanak/axum-bits/tree/master/basics/responders) or [extractors](https://github.com/Muddanak/axum-bits/tree/master/basics/extractors)

### Added in some medium-level additions

## How to run each module

`cargo run --bin start` from the main `axum-bits` directory

### Current Modules


| Module Name | Description |
|---|---|
start       | basic Axum server
responders  | Demo of Axum Response
extractors  | Demo of Axum Extractor
htmlfile    | Demo of serving a static HTML file with tower_http
state       | Demo of how to send a state of data with the server
state-modification | Demo of how to modify the state inside of the server with Arc and Mutex


## [dependencies]

[axum](https://crates.io/crates/axum) 

[serde](https://crates.io/crates/serde) 

[serde_json](https://crates.io/crates/serde_json) 

[tokio](https://crates.io/crates/tokio) 

---
MIT License
