use super::{Record, errors};

pub fn raw_csv_to_records(raw: &str) -> Result<Vec<Record>, errors::Error> {
    if !raw.contains(',') || !raw.contains('\n') {
        return Err(errors::Error::UnableToParse);
    }

    let mut data: Vec<Record> = Vec::new();

    for line in raw.lines() {
        let record: Record = line.split(',')
                                .map(|item| item.to_string())
                                .collect();
        data.push(record);
    }
    Ok(data)
}

pub fn records_to_string(records: &Vec<Record>) -> String {
    let mut combined_records: Vec<String> = Vec::new();

    for record in records {
        let mut record_string = String::new();
        for item in record {
            record_string.push_str(item);
            record_string.push(',');
        }

        // removes the last ","
        record_string.pop();
        record_string.push('\n');
        combined_records.push(record_string);
    }

    let mut result_string = combined_records.into_iter().collect::<String>();
    result_string.pop(); // removes last "\n" character
    result_string
}


pub fn path_validate(path: &str) {
    if !path.ends_with(".csv") {
        panic!("Path does not point to a CSV file. Please check your CSV instance...")
    }
}


#[cfg(test)]
mod tests {
    use crate::Record;
    use super::*;

    #[test]
    fn utils_records_to_string() {
        let records: Vec<Record> = vec![
            vec![String::from("one"), String::from("two"), String::from("three")],
            vec![String::from("four"), String::from("five"), String::from("six")],
            vec![String::from("seven"), String::from("eight"), String::from("nine")],
        ];

        let result = records_to_string(&records);
        assert_eq!(String::from("one,two,three\nfour,five,six\nseven,eight,nine"), result);
    }

    #[test]
    fn utils_records_to_string_with_blanks() {
        let records: Vec<Record> = vec![
            vec![String::from("one"), "".to_string(), String::from("three")],
            vec![String::from("four"), String::from("five"), String::from("six")],
            vec![String::from("seven"), String::from("eight"), String::from("nine")],
        ];

        let result = records_to_string(&records);
        assert_eq!(String::from("one,,three\nfour,five,six\nseven,eight,nine"), result);
    }

    #[test]
    fn utils_raw_csv_to_records() {
        let expected: Vec<Record> = vec![
            vec![String::from("one"), String::from("two"), String::from("three")],
            vec![String::from("four"), String::from("five"), String::from("six")],
            vec![String::from("seven"), String::from("eight"), String::from("nine")],
        ];
        let csv_string = String::from("one,two,three\nfour,five,six\nseven,eight,nine");

        let result = raw_csv_to_records(&csv_string).unwrap_or(vec![vec!["FAIL".to_string()]]);
        assert_eq!(expected, result);
    }
}
