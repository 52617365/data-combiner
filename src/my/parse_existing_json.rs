use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use serde_json::Value;

pub fn parse_existing_json(filename: &str) -> Vec<Value> {
    let lines = read_file(filename);
    let mut json_lines: Vec<Value> = Vec::with_capacity(lines.len());

    for line in lines.iter() {
        let parsed: Value = serde_json::from_str(line).expect("Error turning line into json, check your format.");
        json_lines.push(parsed);
    }
    json_lines
}

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");

    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
