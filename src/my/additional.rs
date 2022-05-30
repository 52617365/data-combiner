use std::error::Error;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Format {
    database:String,
    entries :String,
    dumped : String,
}


pub fn parse_additional() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path("./data/moreleaks.csv")?;

    let iter = rdr.deserialize();

    let mut additional_data: Vec<Format> = Vec::with_capacity(10000);
    for result in iter {
        let record : Format = result?;
        additional_data.push(record);
    }
    println!("{:?}", additional_data);
    Ok(())
}
