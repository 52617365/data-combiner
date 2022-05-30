use std::process;

mod my;

fn main() {
    if let Err(err) = my::parse_additional_csv::parse_additional_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
