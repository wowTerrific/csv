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
        if last_line.len() == 0 {
            return Err(errors::Error::DataNotFound);
        }
        Ok(last_line)
    }

    /// Retreive the first line of the CSV instance. 
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

    /// List the number of records, limited by 
    /// [std::usize::MAX](https://doc.rust-lang.org/std/usize/constant.MAX.html).
    pub fn len(&self) -> usize {
        self.data.len()
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

        if let Err(_) = fs::write(&self.path, temp_data) {
            return Err(errors::Error::Write);
        } else {
            self.state = SaveState::Saved;
            return Ok(())
        }
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

}
