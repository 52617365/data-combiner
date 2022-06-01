use crate::my::combine::combine;
use crate::my::write_as_csv::write_as_csv;
mod my;

fn main() {
    let combined_data = combine("./data/tests/example_existing.txt");
    write_as_csv(combined_data, "./data/combined_data.csv").expect("Error writing csv lines into file.");
}
