use std::{fs, str, collections::HashMap};

pub mod errors;
mod utils;

/// *START HERE* - The CSV struct is used to store and manipulate date within memory
/// before saving/writing to an file on disk.
/// 
/// *IMPORTANT:* Data within the CSV must be UTF8 encoded. Other endcodings will
/// not be gauranteed and may cause errors or will crash the program.
#[derive(Debug)]
pub struct CSV<'a> {
    pub path: &'a str,
    pub data: Vec<Record>,
    state: SaveState,
}

/// Self explanatory. All new instances of CSV will default to an
/// unsaved state. Only CSVs that call the [save](CSV::save) method will have
/// a saved state.
#[derive(Debug)]
pub enum SaveState {
    Saved,
    Unsaved,
}

/// Records are the basis of the stored data within a CSV. All record values typed
/// string for easy parsing and writing. If you need to manipulate numbers, for
/// example, you will need to use [std::str::parse](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse).
pub type Record = Vec<String>;


impl<'a> CSV<'a> {
    
    /// Creates a blank CSV instance with a desired file path
    pub fn new(path: &str) -> CSV {
        utils::path_validate(path);
        CSV {
            path,
            data: Vec::new(),
            state: SaveState::Unsaved,
        }
    }

    /// Create a new CSV instance when you have data ready to insert
    pub fn new_with_data(path: &str, data: Vec<Record>) -> CSV {
        utils::path_validate(path);
        CSV {
            path,
            data,
            state: SaveState::Unsaved,
        }
    }

    /// Create a new CSV instance from an existing CSV file. This method uses
    /// [fs::read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
    /// and appropriately parses in data into a vector of [`Record`]s.
    pub fn new_from_file(path: &str) -> Result<CSV, errors::Error> {
        if !path.ends_with(".csv") {
            return Err(errors::Error::IncorrectFileType);
        }

        let file_attempt = fs::read_to_string(path);
        let file_data = match file_attempt {
            Ok(data) => data,
            Err(_) => return Err(errors::Error::FileRead),
        };

        let data = utils::raw_csv_to_records(&file_data)?;

        Ok(CSV {
            path,
            data,
            state: SaveState::Unsaved,
        })
    }

    /// Check the state of a CSV written to memory. Only CSV's that have used the 
    /// 'save' method will have a saved state. It is impotant to note that all CSVs that
    /// are built, even with 'new_from_file', will have an unsaved state by default.
    pub fn check_state(&self) -> &SaveState {
        &self.state
    }


    pub fn get_last_record(&self) -> Result<&Record, errors::Error> {
        let last_line = &self.data[&self.data.len() - 1];
        if last_line.is_empty() {
            return Err(errors::Error::DataNotFound);
        }
        Ok(last_line)
    }

    /// Retreive the first line of the CSV instance. 
    pub fn get_headers(&self) -> Result<HashMap<usize, &String>, errors::Error> {
        let first_line = &self.data[0];
        if first_line.is_empty() {
            return Err(errors::Error::DataNotFound);
        }
        let mut map: HashMap<usize, &String> = HashMap::new();
        for (i, v) in first_line.iter().enumerate() {
            map.insert(i, v);
        }
        Ok(map)
    }

    /// List the number of records including the header, limited by 
    /// [std::usize::MAX](https://doc.rust-lang.org/std/usize/constant.MAX.html).
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks to see if there are any records in `CSV.data`
    pub fn is_empty(&self) -> bool {
        if self.data.is_empty() {
            return true;
        }
        false
    }

    /// List the number of fields in the first record, limited by 
    /// [std::usize::MAX](https://doc.rust-lang.org/std/usize/constant.MAX.html).
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


    /// Create or overwrite an existing CSV file with the data
    /// attached to the CSV instance.
    pub fn save(&mut self) -> Result<(), errors::Error> {
        let temp_data = utils::records_to_string(&self.data);

        if fs::write(self.path, temp_data).is_err() {
            return Err(errors::Error::Write);
        }

        self.state = SaveState::Saved;
        Ok(())
        
    }

}


// Testing CSV files is in done within ./tests/ directory
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_test_type() {
        let _record: Record = vec!["one".to_string(), "two".to_string()];
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
    fn csv_new_with_data() {
        let data: Vec<Record> = vec![
            vec![String::from("Header 1"), String::from("Header 2"), String::from("Header 3")],
            vec![String::from("Value 1"), String::from("Value 2"), String::from("Value 3")],
            vec![String::from("Value 4"), String::from("Value 5"), String::from("Value 6")],
            vec![String::from("Header 1"), String::from("Header 2"), String::from("Header 3")],
        ];

        let _csv = CSV::new_with_data("test.csv", data);

        assert!(true);
    }

    #[test]
    #[should_panic]
    fn csv_new_with_data_panic() {
        let data: Vec<Record> = vec![
            vec![String::from("Header 1"), String::from("Header 2"), String::from("Header 3")],
            vec![String::from("Value 1"), String::from("Value 2"), String::from("Value 3")],
            vec![String::from("Value 4"), String::from("Value 5"), String::from("Value 6")],
            vec![String::from("Header 1"), String::from("Header 2"), String::from("Header 3")],
        ];

        let _csv = CSV::new_with_data("test.abc", data);
    }

    #[test]
    fn test_check_state() {
        let csv = CSV::new("test.csv");
        assert!(match csv.check_state() {
            SaveState::Saved => false,
            SaveState::Unsaved => true,
        });
    }

    #[test]
    fn test_get_record_methods() {
        let data: Vec<Record> = vec![
            vec![String::from("Header 1"), String::from("Header 2"), String::from("Header 3")],
            vec![String::from("Value 1"), String::from("Value 2"), String::from("Value 3")],
            vec![String::from("Value 4"), String::from("Value 5"), String::from("Value 6")],
            vec![String::from("Header 1"), String::from("Header 2"), String::from("Header 3")],
        ];

        let csv = CSV::new_with_data("test.csv", data);

        if let Ok(header_data) = csv.get_headers() {
            assert_eq!(header_data.get(&0), Some(&&"Header 1".to_string()));
            assert_eq!(header_data.get(&1), Some(&&"Header 2".to_string()));
            assert_eq!(header_data.get(&2), Some(&&"Header 3".to_string()));
        } else {
            assert!(false, "failed to get header data");
        }
        

        if let Ok(record) = csv.get_last_record() {
            let expected = vec![String::from("Header 1"), String::from("Header 2"), String::from("Header 3")];
            assert_eq!(expected[0], record[0]);
            assert_eq!(expected[1], record[1]);
            assert_eq!(expected[2], record[2]);
        } else {
            assert!(false, "failed to get last record")
        }
        let length = csv.len();
        assert_eq!(4, length);
        
    }

    #[test]
    fn test_inserts_lengths() {
        let mut csv = CSV::new("test.csv");
        assert_eq!(csv.len(), 0);

        let single_record: Record = vec![String::from("Head 1"), String::from("Head 2")];
        csv.insert_one(single_record);
        assert_eq!(csv.len(), 1);
        assert_eq!(csv.record_len(), 2);

        let multi_record: Vec<Record> = vec![
            vec![String::from("Value 1"), String::from("Value 2")],
            vec![String::from("Value 4"), String::from("Value 5")],
            vec![String::from("Header 1"), String::from("Header 2")],
        ];
        csv.insert_multi(multi_record);
        assert_eq!(csv.len(), 4);
    }


}