use std::error::Error;

use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Format {
    pub database: String,
    pub entries: String,
    pub dumped: String,
}

pub fn parse_additional_csv() -> Result<Vec<Format>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path("./data/moreleaks.csv")?;

    let iter = rdr.deserialize();

    let mut additional_data: Vec<Format> = Vec::with_capacity(10000);
    for result in iter {
        let record: Format = result?;
        additional_data.push(record);
    }
    Ok(additional_data)
}
