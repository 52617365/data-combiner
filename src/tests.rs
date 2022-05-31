#[cfg(test)]
mod tests {
    // use crate::my::parse_additional_csv::Format;

    use crate::my::parse_additional_csv::parse_additional_csv;
    use crate::my::parse_existing_json::parse_existing_json;

    #[test]
    fn additional_csv_parsing() {
        let expected_data_length: u16 = 3393;

        // Vec<Format>
        let resulting_data = parse_additional_csv().expect("Error parsing csv");

        assert_eq!(expected_data_length, resulting_data.len() as u16);
    }

    #[test]
    fn existing_json_parsing() {
        let expected_data_length: u16 = 1;

        // Vec<Value>
        let resulting_data = parse_existing_json("./data/tests/example_existing.txt");

        println!("{}", resulting_data.len());
        assert_eq!(expected_data_length, resulting_data.len() as u16);
    }
}
