use super::combine::combined_format;
use csv::Writer;
use std::error::Error;

pub fn write_as_csv(
    data_structs: Vec<combined_format>,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;
    wtr.write_record([&"database", &"breach_date", &"lines"])?;
    for data in data_structs.iter() {
        wtr.write_record([&data.database, &data.breach_date, &data.lines])?;
    }
    wtr.flush()?;
    Ok(())
}
