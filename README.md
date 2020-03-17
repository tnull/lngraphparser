# `lngraphparser` -- Simply parse Lightning's network graph from JSON

[LND](https://github.com/lightningnetwork/lnd) with its `describegraph` functionality is an easy way to retrieve JSON-encoded data about the current state of Lightning's payment channel network. Somewhat regularly updated snapshots of this data can be found [here](https://gitlab.tu-berlin.de/rohrer/discharged-pc-data).

This simple library crate utilizes [serde_json](https://github.com/serde-rs/json) to parse the data into statically typed structs.

## Example:
```
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::Read;

use lngraphparser::{Graph, Node, Edge, NodePolicy, Address};

fn main() {
    let file_path = "./lngraph.json";
    let file = File::open(file_path).expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let _res = buf_reader.read_to_string(&mut contents).expect("Could not read from file");

    let graph = lngraphparser::from_json_str(&contents);
    println!("This is the current graph structure: {:?}", graph);
}
```
