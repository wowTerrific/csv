use std::collections::HashMap;
use std::{fs, str};

pub mod errors;

/// The CSV struct is used to store and manipulate date within memory
/// before saving/writing to an external file
#[derive(Debug)]
pub struct CSV<'a> {
    path: &'a str,
    data: Vec<Record>,
    state: SaveState,
}

#[derive(Debug)]
pub enum SaveState {
    Saved,
    Unsaved,
}

pub type Record = Vec<String>;


impl<'a> CSV<'a> {
    
    /// Creates a blank CSV Instance with a desired file path
    pub fn new(path: &str) -> CSV {
        path_validate(path);
        CSV {
            path,
            data: Vec::new(),
            state: SaveState::Unsaved,
        }
    }

    pub fn new_with_data(path: &str, data: Vec<Record>) -> CSV {
        path_validate(path);
        CSV {
            path,
            data,
            state: SaveState::Unsaved,
        }
    }

    pub fn new_from_file(path: &str) -> Result<CSV, errors::Error> {
        if !path.ends_with(".csv") {
            return Err(errors::Error::IncorrectFileType);
        }

        let file_attempt = fs::read_to_string(path);
        let file_data = match file_attempt {
            Ok(data) => data,
            Err(_) => return Err(errors::Error::FileRead),
        };

        let data = raw_csv_to_records(&file_data)?;

        Ok(CSV {
            path,
            data,
            state: SaveState::Unsaved,
        })
    }


    pub fn get_last_record(&self) -> Result<&Record, errors::Error> {
        let last_line = &self.data[&self.data.len() - 1];
        if last_line.len() == 0 {
            return Err(errors::Error::DataNotFound);
        }
        Ok(last_line)
    }


    pub fn get_headers(&self) -> Result<HashMap<usize, &String>, errors::Error> {
        let first_line = &self.data[0];
        if first_line.len() == 0 {
            return Err(errors::Error::DataNotFound);
        }
        let mut map: HashMap<usize, &String> = HashMap::new();
        for (i, v) in first_line.into_iter().enumerate() {
            map.insert(i, v);
        }
        Ok(map)
    }


    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn record_len(&self) -> usize {
        self.data[0].len()
    }

    pub fn insert_one(&mut self, data: Record) {
        self.data.push(data);
    }

    pub fn insert_multi(&mut self, data: Vec<Record>) {
        for x in data {
            self.data.push(x);
        }
    }

    pub fn save(&mut self) -> Result<(), errors::Error> {
        let temp_data = records_to_string(&self.data);

        if let Err(_) = fs::write(&self.path, temp_data) {
            return Err(errors::Error::Write);
        } else {
            self.state = SaveState::Saved;
            return Ok(())
        }
    }

}


fn raw_csv_to_records(raw: &String) -> Result<Vec<Record>, errors::Error> {
    if !raw.contains(",") || !raw.contains("\n") {
        return Err(errors::Error::UnableToParse);
    }

    let mut data: Vec<Record> = Vec::new();

    for line in raw.lines() {
        let record: Record = line.split(',')
                                .into_iter()
                                .map(|item| item.to_string())
                                .collect();
        data.push(record);
    }
    Ok(data)
}

fn records_to_string(records: &Vec<Record>) -> String {
    let mut combined_records: Vec<String> = Vec::new();

    for record in records {
        let mut record_string = String::new();
        for item in record {
            record_string.push_str(item);
            record_string.push_str(",");
        }

        // removes the last ","
        record_string.pop();
        record_string.push_str("\n");
        combined_records.push(record_string);
    }

    combined_records.into_iter().collect::<String>()
}


fn path_validate(path: &str) {
    if !path.ends_with(".csv") {
        panic!("Path does not point to a CSV file. Please check your CSV instance...")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_test_type() {
        let _record: Record = vec!["one".to_string(), "two".to_string()];
        assert!(true);
    }

    #[test]
    fn csv_manual_build_test() {
        let record: Record = vec!["one".to_string(), "two".to_string()];
        let _csv = CSV {
            path: "test.csv",
            data: vec![record],
            state: SaveState::Unsaved,
        };
        assert!(true);
    }

    #[test]
    fn csv_new_no_data() {
        let _csv = CSV::new("test.csv");
        assert!(true);
    }

    #[test]
    #[should_panic]
    fn csv_new_no_data_panic() {
        let _csv = CSV::new("test");
    }

    #[test]
    fn helper_records_to_string() {
        let records: Vec<Record> = vec![
            vec![String::from("one"), String::from("two"), String::from("three")],
            vec![String::from("four"), String::from("five"), String::from("six")],
            vec![String::from("seven"), String::from("eight"), String::from("nine")],
        ];

        let result = records_to_string(&records);
        assert_eq!(String::from("one,two,three\nfour,five,six\nseven,eight,nine\n"), result);
    }

    #[test]
    fn helper_records_to_string_with_blanks() {
        let records: Vec<Record> = vec![
            vec![String::from("one"), "".to_string(), String::from("three")],
            vec![String::from("four"), String::from("five"), String::from("six")],
            vec![String::from("seven"), String::from("eight"), String::from("nine")],
        ];

        let result = records_to_string(&records);
        assert_eq!(String::from("one,,three\nfour,five,six\nseven,eight,nine\n"), result);
    }

    #[test]
    fn helper_raw_csv_to_records() {
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
