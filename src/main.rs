use std::process;

use crate::my::combine::combine;
use crate::my::parse_existing_json::parse_existing_json;

mod my;

fn main() {
    if let Err(err) = my::parse_additional_csv::parse_additional_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
    let json_lines = parse_existing_json("./data/tests/example_existing.txt");
    let database_names = combine("./data/tests/example_existing.txt");
}
