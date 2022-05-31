use super::{parse_additional_csv::parse_additional_csv, parse_existing_json::parse_existing_json};

pub struct combined_format {
    pub database: String,
    pub breach_date: String,
    pub lines: String,
}

// here we are combining the wanted info from additional csv (breach date) and the wanted info from
// existing json ( database name.)
// existing path is path to the existing data.
pub fn combine(existing_path: &str) -> Vec<combined_format> {
    let mut database_names: Vec<String> = Vec::new();

    // Vec<serde_json::Map<String, Value>>
    let existing_data = parse_existing_json(existing_path);

    for data in existing_data.iter() {
        // Loop pushes all database names into vector.
        database_names.push(
            data["filename"]
                .as_str()
                .expect("Error turning database name into string")
                .to_string(),
        );
    }

    let mut combined_data: Vec<combined_format> = Vec::with_capacity(database_names.len());

    // Result<Vec<Format>, Box<dyn Error>>
    let additional_data = parse_additional_csv().expect("Error getting additional data.");

    for name in database_names.iter() {
        for data in additional_data.iter() {
            if &data.database == name {
                let combined = combined_format {
                    database: name.clone(),
                    breach_date: data.dumped.clone(),
                    lines: existing_data.len().to_string(),
                };
                combined_data.push(combined);
            }
        }
    }
    combined_data
}
